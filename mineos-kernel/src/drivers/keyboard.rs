// =============================================================================
// MineOS - PS/2 Keyboard Driver (Rust)
// =============================================================================

#![allow(dead_code)]

use spin::Mutex;
use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use alloc::collections::VecDeque;

/// Maximum number of keys in the input buffer
const KEY_BUFFER_SIZE: usize = 256;

/// Keyboard state
pub struct KeyboardState {
    keyboard: Keyboard<layouts::Us104Key, ScancodeSet1>,
    buffer: VecDeque<KeyEvent>,
    shift_pressed: bool,
    ctrl_pressed: bool,
    alt_pressed: bool,
}

/// A keyboard event
#[derive(Debug, Clone, Copy)]
pub struct KeyEvent {
    pub character: Option<char>,
    pub scancode: u8,
    pub pressed: bool,
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
}

impl KeyEvent {
    /// Check if this is a printable character
    pub fn is_printable(&self) -> bool {
        self.character.map_or(false, |c| c.is_ascii_graphic() || c == ' ')
    }
}

lazy_static! {
    static ref KEYBOARD: Mutex<KeyboardState> = Mutex::new(KeyboardState {
        keyboard: Keyboard::new(
            ScancodeSet1::new(),
            layouts::Us104Key,
            HandleControl::Ignore,
        ),
        buffer: VecDeque::with_capacity(KEY_BUFFER_SIZE),
        shift_pressed: false,
        ctrl_pressed: false,
        alt_pressed: false,
    });
}

/// Initialize the keyboard driver
pub fn init() {
    // Keyboard is initialized via lazy_static
    // IRQ1 handler is set up in the IDT
}

/// Process a raw scancode from the PS/2 keyboard
/// Called from the keyboard interrupt handler
pub fn process_scancode(scancode: u8) {
    let mut state = KEYBOARD.lock();
    
    // Track modifier keys
    match scancode {
        0x2A | 0x36 => { state.shift_pressed = true; return; }    // Shift press
        0xAA | 0xB6 => { state.shift_pressed = false; return; }   // Shift release
        0x1D => { state.ctrl_pressed = true; return; }             // Ctrl press
        0x9D => { state.ctrl_pressed = false; return; }            // Ctrl release
        0x38 => { state.alt_pressed = true; return; }              // Alt press
        0xB8 => { state.alt_pressed = false; return; }             // Alt release
        _ => {}
    }
    
    // Process through pc-keyboard crate
    if let Ok(Some(key_event)) = state.keyboard.add_byte(scancode) {
        if let Some(key) = state.keyboard.process_keyevent(key_event) {
            let character = match key {
                DecodedKey::Unicode(c) => Some(c),
                DecodedKey::RawKey(_) => None,
            };
            
            let event = KeyEvent {
                character,
                scancode,
                pressed: scancode < 0x80, // Key press vs release
                shift: state.shift_pressed,
                ctrl: state.ctrl_pressed,
                alt: state.alt_pressed,
            };
            
            // Add to buffer if it's a key press
            if event.pressed {
                if state.buffer.len() >= KEY_BUFFER_SIZE {
                    state.buffer.pop_front();
                }
                state.buffer.push_back(event);
            }
        }
    }
}

/// Read the next key event from the buffer (non-blocking)
pub fn read_key() -> Option<KeyEvent> {
    KEYBOARD.lock().buffer.pop_front()
}

/// Check if there are keys available in the buffer
pub fn has_key() -> bool {
    !KEYBOARD.lock().buffer.is_empty()
}

/// Wait for a key press (blocking)
pub fn wait_key() -> KeyEvent {
    loop {
        if let Some(event) = read_key() {
            return event;
        }
        x86_64::instructions::hlt(); // Sleep until next interrupt
    }
}

/// Read a line of text from keyboard (blocking, with echo support)
pub fn read_line(echo_fn: Option<fn(char)>) -> alloc::string::String {
    let mut line = alloc::string::String::new();
    
    loop {
        let event = wait_key();
        
        if let Some(ch) = event.character {
            match ch {
                '\n' | '\r' => {
                    if let Some(echo) = echo_fn {
                        echo('\n');
                    }
                    break;
                }
                '\x08' => {
                    // Backspace
                    if !line.is_empty() {
                        line.pop();
                        if let Some(echo) = echo_fn {
                            echo('\x08');
                        }
                    }
                }
                c if c.is_ascii_graphic() || c == ' ' => {
                    line.push(c);
                    if let Some(echo) = echo_fn {
                        echo(c);
                    }
                }
                _ => {}
            }
        }
    }
    
    line
}
