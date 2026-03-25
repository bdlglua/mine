// =============================================================================
// MineOS - Framebuffer Graphics Driver (Rust)
// =============================================================================
// Direct pixel-level access to the linear framebuffer provided by the
// bootloader. This is the foundation for all GUI rendering in MineOS.
//
// Supports:
//   - 32-bit color (BGRA format)
//   - Pixel plotting, line drawing, rectangle filling
//   - Double buffering to prevent screen tearing
//   - Bitmap font rendering
// =============================================================================

use alloc::vec::Vec;
use spin::Mutex;
use lazy_static::lazy_static;

/// BGRA color representation (32-bit)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Color { b, g, r, a: 0xFF }
    }

    pub const fn with_alpha(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { b, g, r, a }
    }

    /// Blend this color over another (alpha compositing)
    pub fn blend_over(self, bg: Color) -> Color {
        let alpha = self.a as u16;
        let inv_alpha = 255 - alpha;
        Color {
            r: ((self.r as u16 * alpha + bg.r as u16 * inv_alpha) / 255) as u8,
            g: ((self.g as u16 * alpha + bg.g as u16 * inv_alpha) / 255) as u8,
            b: ((self.b as u16 * alpha + bg.b as u16 * inv_alpha) / 255) as u8,
            a: 0xFF,
        }
    }
}

// MineOS Color Palette - "Obsidian Glass" Theme
pub mod palette {
    use super::Color;
    
    pub const DESKTOP_BG: Color       = Color::new(5, 5, 10);
    pub const WINDOW_BG: Color        = Color::new(15, 15, 22);
    pub const WINDOW_BORDER: Color    = Color::new(40, 40, 55);
    pub const TITLEBAR_BG: Color      = Color::new(10, 10, 16);
    pub const TITLEBAR_ACTIVE: Color  = Color::new(0, 30, 35);
    pub const TASKBAR_BG: Color       = Color::new(8, 8, 14);
    pub const ACCENT: Color           = Color::new(0, 240, 255);
    pub const ACCENT_HOVER: Color     = Color::new(51, 243, 255);
    pub const DANGER: Color           = Color::new(255, 85, 0);
    pub const TEXT_PRIMARY: Color     = Color::new(248, 250, 252);
    pub const TEXT_SECONDARY: Color   = Color::new(148, 163, 184);
    pub const TEXT_MUTED: Color       = Color::new(71, 85, 105);
    pub const SURFACE_HOVER: Color    = Color::new(25, 25, 35);
    pub const SURFACE_ACTIVE: Color   = Color::new(30, 30, 42);
    pub const BLACK: Color            = Color::new(0, 0, 0);
    pub const WHITE: Color            = Color::new(255, 255, 255);
    pub const TERMINAL_GREEN: Color   = Color::new(0, 255, 128);
}

/// Framebuffer state
pub struct FrameBuffer {
    /// Physical address of the framebuffer
    pub base_addr: usize,
    /// Total size in bytes
    pub size: usize,
    /// Screen width in pixels
    pub width: usize,
    /// Screen height in pixels
    pub height: usize,
    /// Bytes per pixel (typically 4 for 32-bit)
    pub bpp: usize,
    /// Stride (bytes per scanline, may include padding)
    pub stride: usize,
    /// Back buffer for double buffering
    pub back_buffer: Vec<u8>,
}

impl FrameBuffer {
    /// Create a new framebuffer instance
    pub fn new(
        base_addr: usize,
        size: usize,
        width: usize,
        height: usize,
        bpp: usize,
        stride: usize,
    ) -> Self {
        let buf_size = stride * bpp * height;
        let mut back_buffer = Vec::new();
        back_buffer.resize(buf_size, 0u8);
        
        FrameBuffer {
            base_addr,
            size,
            width,
            height,
            bpp,
            stride,
            back_buffer,
        }
    }

    /// Plot a single pixel at (x, y) with given color
    #[inline(always)]
    pub fn put_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }
        let offset = (y * self.stride + x) * self.bpp;
        if offset + 3 < self.back_buffer.len() {
            self.back_buffer[offset] = color.b;
            self.back_buffer[offset + 1] = color.g;
            self.back_buffer[offset + 2] = color.r;
            self.back_buffer[offset + 3] = color.a;
        }
    }

    /// Get the color of a pixel at (x, y)
    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        if x >= self.width || y >= self.height {
            return Color::new(0, 0, 0);
        }
        let offset = (y * self.stride + x) * self.bpp;
        Color {
            b: self.back_buffer[offset],
            g: self.back_buffer[offset + 1],
            r: self.back_buffer[offset + 2],
            a: self.back_buffer[offset + 3],
        }
    }

    /// Fill the entire screen with a color
    pub fn clear(&mut self, color: Color) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.put_pixel(x, y, color);
            }
        }
    }

    /// Draw a filled rectangle
    pub fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: Color) {
        let x_end = (x + w).min(self.width);
        let y_end = (y + h).min(self.height);
        for py in y..y_end {
            for px in x..x_end {
                self.put_pixel(px, py, color);
            }
        }
    }

    /// Draw a rectangle outline
    pub fn draw_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: Color) {
        // Top and bottom
        for px in x..(x + w).min(self.width) {
            self.put_pixel(px, y, color);
            if y + h - 1 < self.height {
                self.put_pixel(px, y + h - 1, color);
            }
        }
        // Left and right
        for py in y..(y + h).min(self.height) {
            self.put_pixel(x, py, color);
            if x + w - 1 < self.width {
                self.put_pixel(x + w - 1, py, color);
            }
        }
    }

    /// Draw a horizontal line
    pub fn draw_hline(&mut self, x: usize, y: usize, length: usize, color: Color) {
        if y >= self.height { return; }
        let end = (x + length).min(self.width);
        for px in x..end {
            self.put_pixel(px, y, color);
        }
    }

    /// Draw a vertical line
    pub fn draw_vline(&mut self, x: usize, y: usize, length: usize, color: Color) {
        if x >= self.width { return; }
        let end = (y + length).min(self.height);
        for py in y..end {
            self.put_pixel(x, py, color);
        }
    }

    /// Draw a filled rectangle with rounded corners (approximate)
    pub fn fill_rounded_rect(
        &mut self, x: usize, y: usize, w: usize, h: usize,
        radius: usize, color: Color,
    ) {
        // Fill main body
        self.fill_rect(x + radius, y, w - 2 * radius, h, color);
        self.fill_rect(x, y + radius, w, h - 2 * radius, color);
        
        // Fill corners with circle approximation
        self.fill_circle_quarter(x + radius, y + radius, radius, 0, color);
        self.fill_circle_quarter(x + w - radius - 1, y + radius, radius, 1, color);
        self.fill_circle_quarter(x + radius, y + h - radius - 1, radius, 2, color);
        self.fill_circle_quarter(x + w - radius - 1, y + h - radius - 1, radius, 3, color);
    }

    /// Fill a quarter circle (for rounded rectangles)
    fn fill_circle_quarter(
        &mut self, cx: usize, cy: usize, r: usize,
        quarter: u8, color: Color,
    ) {
        let r_sq = (r * r) as isize;
        for dy in 0..=r {
            for dx in 0..=r {
                if (dx * dx + dy * dy) as isize <= r_sq {
                    let (px, py) = match quarter {
                        0 => (cx - dx, cy - dy), // Top-left
                        1 => (cx + dx, cy - dy), // Top-right
                        2 => (cx - dx, cy + dy), // Bottom-left
                        3 => (cx + dx, cy + dy), // Bottom-right
                        _ => continue,
                    };
                    self.put_pixel(px, py, color);
                }
            }
        }
    }

    /// Swap the back buffer to the front (display it)
    pub fn swap_buffers(&mut self) {
        unsafe {
            let fb_ptr = self.base_addr as *mut u8;
            let buf_ptr = self.back_buffer.as_ptr();
            let copy_size = self.back_buffer.len().min(self.size);
            core::ptr::copy_nonoverlapping(buf_ptr, fb_ptr, copy_size);
        }
    }

    /// Draw a character using the built-in bitmap font
    pub fn draw_char(
        &mut self, x: usize, y: usize,
        ch: char, color: Color, scale: usize,
    ) {
        let glyph = crate::gui::font::get_glyph(ch);
        for (row, &bits) in glyph.iter().enumerate() {
            for col in 0..8 {
                if bits & (1 << (7 - col)) != 0 {
                    for sy in 0..scale {
                        for sx in 0..scale {
                            self.put_pixel(
                                x + col * scale + sx,
                                y + row * scale + sy,
                                color,
                            );
                        }
                    }
                }
            }
        }
    }

    /// Draw a string of text
    pub fn draw_text(
        &mut self, x: usize, y: usize,
        text: &str, color: Color, scale: usize,
    ) {
        let char_width = 8 * scale;
        let mut cx = x;
        let mut cy = y;
        for ch in text.chars() {
            if ch == '\n' {
                cx = x;
                cy += 16 * scale;
                continue;
            }
            self.draw_char(cx, cy, ch, color, scale);
            cx += char_width + scale; // +scale for letter spacing
        }
    }

    /// Measure text width in pixels
    pub fn text_width(text: &str, scale: usize) -> usize {
        let char_width = 8 * scale + scale;
        text.len() * char_width
    }
}

lazy_static! {
    pub static ref FB: Mutex<Option<FrameBuffer>> = Mutex::new(None);
}
