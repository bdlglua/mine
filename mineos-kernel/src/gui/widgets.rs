// =============================================================================
// MineOS - GUI Widgets (Rust)
// =============================================================================
// Reusable UI components for the desktop environment.
// =============================================================================

use crate::drivers::framebuffer::{Color, FrameBuffer, palette};
use alloc::string::String;

/// Button widget
pub struct Button {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub label: String,
    pub bg_color: Color,
    pub text_color: Color,
    pub hovered: bool,
}

impl Button {
    pub fn new(x: usize, y: usize, width: usize, height: usize, label: &str) -> Self {
        Button {
            x, y, width, height,
            label: String::from(label),
            bg_color: palette::SURFACE_HOVER,
            text_color: palette::TEXT_PRIMARY,
            hovered: false,
        }
    }

    pub fn draw(&self, fb: &mut FrameBuffer) {
        let bg = if self.hovered { palette::SURFACE_ACTIVE } else { self.bg_color };
        fb.fill_rounded_rect(self.x, self.y, self.width, self.height, 4, bg);
        fb.draw_rect(self.x, self.y, self.width, self.height, palette::WINDOW_BORDER);
        
        // Center text
        let text_w = FrameBuffer::text_width(&self.label, 1);
        let tx = self.x + (self.width.saturating_sub(text_w)) / 2;
        let ty = self.y + (self.height.saturating_sub(16)) / 2;
        fb.draw_text(tx, ty, &self.label, self.text_color, 1);
    }

    pub fn contains(&self, px: usize, py: usize) -> bool {
        px >= self.x && px < self.x + self.width &&
        py >= self.y && py < self.y + self.height
    }
}

/// Text input field
pub struct TextInput {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub text: String,
    pub cursor_pos: usize,
    pub focused: bool,
    pub placeholder: String,
}

impl TextInput {
    pub fn new(x: usize, y: usize, width: usize, placeholder: &str) -> Self {
        TextInput {
            x, y, width, height: 28,
            text: String::new(),
            cursor_pos: 0,
            focused: false,
            placeholder: String::from(placeholder),
        }
    }

    pub fn draw(&self, fb: &mut FrameBuffer) {
        let border_color = if self.focused { palette::ACCENT } else { palette::WINDOW_BORDER };
        fb.fill_rect(self.x, self.y, self.width, self.height, Color::new(10, 10, 16));
        fb.draw_rect(self.x, self.y, self.width, self.height, border_color);
        
        let text_y = self.y + (self.height.saturating_sub(16)) / 2;
        if self.text.is_empty() && !self.focused {
            fb.draw_text(self.x + 8, text_y, &self.placeholder, palette::TEXT_MUTED, 1);
        } else {
            fb.draw_text(self.x + 8, text_y, &self.text, palette::TEXT_PRIMARY, 1);
        }
        
        // Draw cursor
        if self.focused {
            let cursor_x = self.x + 8 + FrameBuffer::text_width(&self.text[..self.cursor_pos.min(self.text.len())], 1);
            fb.fill_rect(cursor_x, text_y, 2, 16, palette::ACCENT);
        }
    }

    pub fn type_char(&mut self, ch: char) {
        if ch == '\x08' {
            // Backspace
            if self.cursor_pos > 0 {
                self.text.remove(self.cursor_pos - 1);
                self.cursor_pos -= 1;
            }
        } else if ch.is_ascii_graphic() || ch == ' ' {
            self.text.insert(self.cursor_pos, ch);
            self.cursor_pos += 1;
        }
    }
}

/// Progress bar
pub struct ProgressBar {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub value: f32, // 0.0 to 1.0
    pub color: Color,
}

impl ProgressBar {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        ProgressBar {
            x, y, width, height,
            value: 0.0,
            color: palette::ACCENT,
        }
    }

    pub fn draw(&self, fb: &mut FrameBuffer) {
        // Background
        fb.fill_rect(self.x, self.y, self.width, self.height, Color::new(20, 20, 30));
        
        // Fill
        let fill_width = (self.width as f32 * self.value.clamp(0.0, 1.0)) as usize;
        if fill_width > 0 {
            fb.fill_rounded_rect(self.x, self.y, fill_width, self.height, 2, self.color);
        }
    }
}

/// Label (static text)
pub fn draw_label(fb: &mut FrameBuffer, x: usize, y: usize, text: &str, color: Color, scale: usize) {
    fb.draw_text(x, y, text, color, scale);
}
