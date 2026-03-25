// =============================================================================
// MineOS - Taskbar (Rust)
// =============================================================================

#![allow(dead_code)]

use crate::drivers::framebuffer::{Color, FrameBuffer, palette};
use super::window::WindowManager;
use alloc::string::String;

pub const TASKBAR_HEIGHT: usize = 40;
const START_BTN_WIDTH: usize = 40;

pub struct Taskbar {
    pub start_menu_open: bool,
}

impl Taskbar {
    pub fn new() -> Self {
        Taskbar { start_menu_open: false }
    }

    pub fn draw(&self, fb: &mut FrameBuffer, wm: &WindowManager) {
        let screen_h = fb.height;
        let screen_w = fb.width;
        let taskbar_y = screen_h - TASKBAR_HEIGHT;

        fb.fill_rect(0, taskbar_y, screen_w, TASKBAR_HEIGHT, palette::TASKBAR_BG);
        fb.draw_hline(0, taskbar_y, screen_w, palette::WINDOW_BORDER);

        let start_bg = if self.start_menu_open { Color::new(0, 40, 45) } else { palette::TASKBAR_BG };
        fb.fill_rect(4, taskbar_y + 4, START_BTN_WIDTH, TASKBAR_HEIGHT - 8, start_bg);

        let cx = 4 + START_BTN_WIDTH / 2;
        let cy = taskbar_y + TASKBAR_HEIGHT / 2;
        self.draw_hexagon(fb, cx, cy, 10, palette::ACCENT);

        let mut btn_x = START_BTN_WIDTH + 12;
        for win in wm.active_windows() {
            let btn_w = 120;
            let btn_bg = if win.focused {
                Color::new(0, 30, 35)
            } else if win.minimized {
                palette::TASKBAR_BG
            } else {
                palette::SURFACE_HOVER
            };

            fb.fill_rounded_rect(btn_x, taskbar_y + 6, btn_w, TASKBAR_HEIGHT - 12, 4, btn_bg);

            let title: String = if win.title.len() > 12 {
                let mut t = String::from(&win.title[..12]);
                t.push_str("..");
                t
            } else {
                win.title.clone()
            };
            fb.draw_text(btn_x + 8, taskbar_y + 14, &title,
                if win.focused { palette::ACCENT } else { palette::TEXT_SECONDARY }, 1);

            if win.focused && !win.minimized {
                fb.fill_rect(btn_x + btn_w / 2 - 8, taskbar_y + TASKBAR_HEIGHT - 4, 16, 2, palette::ACCENT);
            }
            btn_x += btn_w + 4;
        }

        let tray_x = screen_w - 180;
        self.draw_wifi_icon(fb, tray_x, taskbar_y + 12);
        self.draw_battery_icon(fb, tray_x + 30, taskbar_y + 14);
        self.draw_volume_icon(fb, tray_x + 60, taskbar_y + 12);
        fb.draw_text(tray_x + 90, taskbar_y + 14, "12:00", palette::TEXT_SECONDARY, 1);
    }

    fn draw_hexagon(&self, fb: &mut FrameBuffer, cx: usize, cy: usize, r: usize, color: Color) {
        let points: [(isize, isize); 6] = [
            (0, -(r as isize)),
            (r as isize * 7 / 8, -(r as isize) / 2),
            (r as isize * 7 / 8, r as isize / 2),
            (0, r as isize),
            (-(r as isize) * 7 / 8, r as isize / 2),
            (-(r as isize) * 7 / 8, -(r as isize) / 2),
        ];
        for i in 0..6 {
            let (x1, y1) = points[i];
            let (x2, y2) = points[(i + 1) % 6];
            self.draw_line(fb,
                (cx as isize + x1) as usize, (cy as isize + y1) as usize,
                (cx as isize + x2) as usize, (cy as isize + y2) as usize,
                color);
        }
    }

    fn draw_line(&self, fb: &mut FrameBuffer, x0: usize, y0: usize, x1: usize, y1: usize, color: Color) {
        let dx = (x1 as isize - x0 as isize).abs();
        let dy = -(y1 as isize - y0 as isize).abs();
        let sx: isize = if x0 < x1 { 1 } else { -1 };
        let sy: isize = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let mut x = x0 as isize;
        let mut y = y0 as isize;
        loop {
            if x >= 0 && y >= 0 { fb.put_pixel(x as usize, y as usize, color); }
            if x == x1 as isize && y == y1 as isize { break; }
            let e2 = 2 * err;
            if e2 >= dy { err += dy; x += sx; }
            if e2 <= dx { err += dx; y += sy; }
        }
    }

    fn draw_wifi_icon(&self, fb: &mut FrameBuffer, x: usize, y: usize) {
        fb.draw_hline(x + 2, y + 12, 12, palette::TEXT_MUTED);
        fb.draw_hline(x + 4, y + 8, 8, palette::TEXT_SECONDARY);
        fb.draw_hline(x + 6, y + 4, 4, palette::TEXT_SECONDARY);
        fb.put_pixel(x + 8, y, palette::ACCENT);
    }

    fn draw_battery_icon(&self, fb: &mut FrameBuffer, x: usize, y: usize) {
        fb.draw_rect(x, y, 18, 10, palette::TEXT_SECONDARY);
        fb.fill_rect(x + 18, y + 3, 2, 4, palette::TEXT_SECONDARY);
        fb.fill_rect(x + 2, y + 2, 12, 6, palette::ACCENT);
    }

    fn draw_volume_icon(&self, fb: &mut FrameBuffer, x: usize, y: usize) {
        fb.fill_rect(x, y + 4, 4, 8, palette::TEXT_SECONDARY);
        fb.fill_rect(x + 4, y + 2, 2, 12, palette::TEXT_SECONDARY);
        fb.draw_vline(x + 10, y + 3, 10, palette::TEXT_MUTED);
        fb.draw_vline(x + 14, y + 1, 14, palette::TEXT_MUTED);
    }

    pub fn is_start_clicked(&self, px: usize, py: usize, screen_h: usize) -> bool {
        let taskbar_y = screen_h - TASKBAR_HEIGHT;
        px < START_BTN_WIDTH + 8 && py >= taskbar_y
    }
}
