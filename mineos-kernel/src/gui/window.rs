// =============================================================================
// MineOS - Window Manager (Rust)
// =============================================================================
// Manages windows on the desktop - creation, movement, focus, rendering.
// =============================================================================

use crate::drivers::framebuffer::{Color, FrameBuffer, palette};
use alloc::string::String;
use alloc::vec::Vec;

/// Window state
#[derive(Clone)]
pub struct Window {
    pub id: usize,
    pub title: String,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub visible: bool,
    pub minimized: bool,
    pub maximized: bool,
    pub focused: bool,
    pub z_index: usize,
    pub app_type: AppType,
    // Previous bounds for restore from maximize
    pub prev_x: usize,
    pub prev_y: usize,
    pub prev_w: usize,
    pub prev_h: usize,
}

#[derive(Clone, Copy, PartialEq)]
pub enum AppType {
    Terminal,
    Calculator,
    TextEditor,
    FileManager,
    TaskManager,
    Settings,
    About,
}

/// Title bar height in pixels
pub const TITLEBAR_HEIGHT: usize = 32;
/// Title bar button size
const BTN_SIZE: usize = 20;

impl Window {
    pub fn new(id: usize, title: &str, x: usize, y: usize, width: usize, height: usize, app_type: AppType) -> Self {
        Window {
            id,
            title: String::from(title),
            x, y, width, height,
            visible: true,
            minimized: false,
            maximized: false,
            focused: false,
            z_index: id,
            app_type,
            prev_x: x,
            prev_y: y,
            prev_w: width,
            prev_h: height,
        }
    }

    /// Draw the window frame (title bar + border)
    pub fn draw_frame(&self, fb: &mut FrameBuffer) {
        if self.minimized || !self.visible {
            return;
        }

        // Window background with rounded corners
        fb.fill_rounded_rect(
            self.x, self.y, self.width, self.height,
            6, palette::WINDOW_BG,
        );

        // Window border
        fb.draw_rect(
            self.x, self.y, self.width, self.height,
            if self.focused { Color::new(0, 60, 70) } else { palette::WINDOW_BORDER },
        );

        // Title bar background
        let tb_color = if self.focused { palette::TITLEBAR_ACTIVE } else { palette::TITLEBAR_BG };
        fb.fill_rect(self.x + 1, self.y + 1, self.width - 2, TITLEBAR_HEIGHT, tb_color);

        // Title bar bottom border
        fb.draw_hline(self.x, self.y + TITLEBAR_HEIGHT, self.width, palette::WINDOW_BORDER);

        // Title text
        fb.draw_text(
            self.x + 12, self.y + (TITLEBAR_HEIGHT - 16) / 2,
            &self.title, palette::TEXT_PRIMARY, 1,
        );

        // Window control buttons (right side of title bar)
        let btn_y = self.y + (TITLEBAR_HEIGHT - BTN_SIZE) / 2;
        
        // Close button (red)
        let close_x = self.x + self.width - BTN_SIZE - 8;
        fb.fill_rounded_rect(close_x, btn_y, BTN_SIZE, BTN_SIZE, 3, Color::new(60, 20, 20));
        // Draw X
        self.draw_x(fb, close_x + 5, btn_y + 5, 10, palette::DANGER);

        // Maximize button
        let max_x = close_x - BTN_SIZE - 4;
        fb.fill_rounded_rect(max_x, btn_y, BTN_SIZE, BTN_SIZE, 3, palette::SURFACE_HOVER);
        fb.draw_rect(max_x + 5, btn_y + 5, 10, 10, palette::TEXT_SECONDARY);

        // Minimize button
        let min_x = max_x - BTN_SIZE - 4;
        fb.fill_rounded_rect(min_x, btn_y, BTN_SIZE, BTN_SIZE, 3, palette::SURFACE_HOVER);
        fb.draw_hline(min_x + 5, btn_y + BTN_SIZE / 2, 10, palette::TEXT_SECONDARY);
    }

    /// Draw an X symbol for the close button
    fn draw_x(&self, fb: &mut FrameBuffer, x: usize, y: usize, size: usize, color: Color) {
        for i in 0..size {
            fb.put_pixel(x + i, y + i, color);
            fb.put_pixel(x + size - 1 - i, y + i, color);
        }
    }

    /// Get the content area rectangle (excluding title bar)
    pub fn content_rect(&self) -> (usize, usize, usize, usize) {
        (
            self.x + 1,
            self.y + TITLEBAR_HEIGHT + 1,
            self.width - 2,
            self.height - TITLEBAR_HEIGHT - 2,
        )
    }

    /// Check if a point is in the title bar
    pub fn is_in_titlebar(&self, px: usize, py: usize) -> bool {
        px >= self.x && px < self.x + self.width &&
        py >= self.y && py < self.y + TITLEBAR_HEIGHT
    }

    /// Check if a point is on the close button
    pub fn is_on_close_btn(&self, px: usize, py: usize) -> bool {
        let btn_y = self.y + (TITLEBAR_HEIGHT - BTN_SIZE) / 2;
        let close_x = self.x + self.width - BTN_SIZE - 8;
        px >= close_x && px < close_x + BTN_SIZE &&
        py >= btn_y && py < btn_y + BTN_SIZE
    }

    /// Check if a point is on the minimize button
    pub fn is_on_minimize_btn(&self, px: usize, py: usize) -> bool {
        let btn_y = self.y + (TITLEBAR_HEIGHT - BTN_SIZE) / 2;
        let close_x = self.x + self.width - BTN_SIZE - 8;
        let max_x = close_x - BTN_SIZE - 4;
        let min_x = max_x - BTN_SIZE - 4;
        px >= min_x && px < min_x + BTN_SIZE &&
        py >= btn_y && py < btn_y + BTN_SIZE
    }

    /// Check if a point is on the maximize button
    pub fn is_on_maximize_btn(&self, px: usize, py: usize) -> bool {
        let btn_y = self.y + (TITLEBAR_HEIGHT - BTN_SIZE) / 2;
        let close_x = self.x + self.width - BTN_SIZE - 8;
        let max_x = close_x - BTN_SIZE - 4;
        px >= max_x && px < max_x + BTN_SIZE &&
        py >= btn_y && py < btn_y + BTN_SIZE
    }

    /// Check if a point is inside the window
    pub fn contains(&self, px: usize, py: usize) -> bool {
        px >= self.x && px < self.x + self.width &&
        py >= self.y && py < self.y + self.height
    }

    /// Toggle maximize
    pub fn toggle_maximize(&mut self, screen_w: usize, screen_h: usize) {
        if self.maximized {
            self.x = self.prev_x;
            self.y = self.prev_y;
            self.width = self.prev_w;
            self.height = self.prev_h;
            self.maximized = false;
        } else {
            self.prev_x = self.x;
            self.prev_y = self.y;
            self.prev_w = self.width;
            self.prev_h = self.height;
            self.x = 0;
            self.y = 0;
            self.width = screen_w;
            self.height = screen_h - super::taskbar::TASKBAR_HEIGHT;
            self.maximized = true;
        }
    }
}

/// Window manager - tracks all open windows
pub struct WindowManager {
    pub windows: Vec<Window>,
    next_id: usize,
    next_z: usize,
}

impl WindowManager {
    pub fn new() -> Self {
        WindowManager {
            windows: Vec::new(),
            next_id: 1,
            next_z: 1,
        }
    }

    /// Open a new window
    pub fn open(&mut self, title: &str, x: usize, y: usize, w: usize, h: usize, app_type: AppType) -> usize {
        // Check if window of this type is already open
        for win in &mut self.windows {
            if win.app_type == app_type {
                win.minimized = false;
                win.visible = true;
                win.focused = true;
                win.z_index = self.next_z;
                self.next_z += 1;
                return win.id;
            }
        }

        let id = self.next_id;
        self.next_id += 1;
        let mut win = Window::new(id, title, x, y, w, h, app_type);
        win.z_index = self.next_z;
        win.focused = true;
        self.next_z += 1;
        
        // Unfocus all others
        for w in &mut self.windows {
            w.focused = false;
        }
        
        self.windows.push(win);
        id
    }

    /// Close a window by id
    pub fn close(&mut self, id: usize) {
        self.windows.retain(|w| w.id != id);
    }

    /// Focus a window
    pub fn focus(&mut self, id: usize) {
        for w in &mut self.windows {
            w.focused = w.id == id;
            if w.id == id {
                w.z_index = self.next_z;
                self.next_z += 1;
            }
        }
    }

    /// Draw all windows (sorted by z-index)
    pub fn draw_all(&self, fb: &mut FrameBuffer) {
        let mut sorted: Vec<&Window> = self.windows.iter()
            .filter(|w| w.visible && !w.minimized)
            .collect();
        sorted.sort_by_key(|w| w.z_index);
        
        for win in sorted {
            win.draw_frame(fb);
        }
    }

    /// Find the topmost window at a point
    pub fn window_at(&self, px: usize, py: usize) -> Option<usize> {
        let mut found: Option<(usize, usize)> = None;
        for w in &self.windows {
            if w.visible && !w.minimized && w.contains(px, py) {
                match found {
                    Some((_, z)) if w.z_index > z => found = Some((w.id, w.z_index)),
                    None => found = Some((w.id, w.z_index)),
                    _ => {}
                }
            }
        }
        found.map(|(id, _)| id)
    }

    /// Get active (visible, non-minimized) windows
    pub fn active_windows(&self) -> Vec<&Window> {
        self.windows.iter().filter(|w| w.visible).collect()
    }
}
