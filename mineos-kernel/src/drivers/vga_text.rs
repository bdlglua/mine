// =============================================================================
// MineOS - VGA Text Mode Driver
// =============================================================================
// Provides text output to the VGA text buffer at 0xB8000.
// Used for early boot messages and panic output.
// =============================================================================

use core::fmt;
use volatile::Volatile;
use spin::Mutex;
use lazy_static::lazy_static;

const VGA_BUFFER_ADDR: usize = 0xb8000;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

/// VGA color codes
#[allow(dead_code)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

/// VGA character entry (character + color attribute)
#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct VgaChar {
    character: u8,
    color: u8,
}

/// VGA text buffer - 80x25 grid of characters
#[repr(transparent)]
struct VgaBuffer {
    chars: [[Volatile<VgaChar>; VGA_WIDTH]; VGA_HEIGHT],
}

/// VGA text writer state
pub struct VgaWriter {
    col: usize,
    row: usize,
    color: u8,
    buffer: &'static mut VgaBuffer,
}

impl VgaWriter {
    /// Create a new VGA writer
    fn new() -> Self {
        VgaWriter {
            col: 0,
            row: 0,
            color: 0x0F, // White on black
            buffer: unsafe { &mut *(VGA_BUFFER_ADDR as *mut VgaBuffer) },
        }
    }

    /// Set text color attribute
    pub fn set_color(&mut self, color: u8) {
        self.color = color;
    }

    /// Write a single byte to the VGA buffer
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            b'\r' => self.col = 0,
            byte => {
                if self.col >= VGA_WIDTH {
                    self.new_line();
                }
                self.buffer.chars[self.row][self.col].write(VgaChar {
                    character: byte,
                    color: self.color,
                });
                self.col += 1;
            }
        }
    }

    /// Write a string to the VGA buffer
    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' | b'\r' => self.write_byte(byte),
                _ => self.write_byte(0xfe), // Unknown char placeholder
            }
        }
    }

    /// Move to a new line, scrolling if necessary
    fn new_line(&mut self) {
        if self.row < VGA_HEIGHT - 1 {
            self.row += 1;
        } else {
            // Scroll up
            for row in 1..VGA_HEIGHT {
                for col in 0..VGA_WIDTH {
                    let ch = self.buffer.chars[row][col].read();
                    self.buffer.chars[row - 1][col].write(ch);
                }
            }
            // Clear last row
            for col in 0..VGA_WIDTH {
                self.buffer.chars[VGA_HEIGHT - 1][col].write(VgaChar {
                    character: b' ',
                    color: self.color,
                });
            }
        }
        self.col = 0;
    }

    /// Clear the entire screen
    pub fn clear(&mut self) {
        for row in 0..VGA_HEIGHT {
            for col in 0..VGA_WIDTH {
                self.buffer.chars[row][col].write(VgaChar {
                    character: b' ',
                    color: self.color,
                });
            }
        }
        self.col = 0;
        self.row = 0;
    }
}

impl fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<VgaWriter> = Mutex::new(VgaWriter::new());
}

// =============================================================================
// Public API
// =============================================================================

/// Print a string to VGA text buffer
pub fn print_str(s: &str) {
    WRITER.lock().write_str(s);
}

/// Set VGA text color
pub fn set_color(color: u8) {
    WRITER.lock().set_color(color);
}

/// Clear the VGA text screen
pub fn clear_screen() {
    WRITER.lock().clear();
}

/// Macro for formatted printing (like println!)
#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {
        use core::fmt::Write;
        let _ = write!($crate::drivers::vga_text::WRITER.lock(), $($arg)*);
    };
}

#[macro_export]
macro_rules! kprintln {
    () => ($crate::kprint!("\n"));
    ($($arg:tt)*) => {
        $crate::kprint!($($arg)*);
        $crate::kprint!("\n");
    };
}
