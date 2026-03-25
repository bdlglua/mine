// =============================================================================
// MineOS - Desktop Environment (Rust)
// =============================================================================
// The main desktop environment that manages the entire GUI.
// Renders the wallpaper, desktop icons, windows, taskbar, and handles input.
// =============================================================================

use crate::drivers::framebuffer::{Color, FrameBuffer, FB, palette};
use crate::drivers::keyboard;
use crate::gui::window::{WindowManager, AppType, TITLEBAR_HEIGHT};
use crate::gui::taskbar::{Taskbar, TASKBAR_HEIGHT};
use crate::gui::widgets;
use alloc::string::String;
use alloc::vec::Vec;

/// Desktop icon definition
struct DesktopIcon {
    label: &'static str,
    app_type: AppType,
    x: usize,
    y: usize,
}

const ICON_SIZE: usize = 48;
const ICON_SPACING: usize = 90;

/// Run the desktop environment main loop
pub fn run_desktop() -> ! {
    let mut wm = WindowManager::new();
    let mut taskbar = Taskbar::new();
    
    // Desktop icons
    let icons = [
        DesktopIcon { label: "Terminal",  app_type: AppType::Terminal,    x: 20, y: 20 },
        DesktopIcon { label: "Files",     app_type: AppType::FileManager, x: 20, y: 20 + ICON_SPACING },
        DesktopIcon { label: "Editor",    app_type: AppType::TextEditor,  x: 20, y: 20 + ICON_SPACING * 2 },
        DesktopIcon { label: "Calc",      app_type: AppType::Calculator,  x: 20, y: 20 + ICON_SPACING * 3 },
        DesktopIcon { label: "Tasks",     app_type: AppType::TaskManager, x: 20, y: 20 + ICON_SPACING * 4 },
        DesktopIcon { label: "Settings",  app_type: AppType::Settings,    x: 20, y: 20 + ICON_SPACING * 5 },
    ];

    // Main desktop loop
    loop {
        let mut fb_lock = FB.lock();
        if let Some(ref mut fb) = *fb_lock {
            // 1. Draw desktop background
            draw_wallpaper(fb);
            
            // 2. Draw grid overlay (subtle)
            draw_grid_overlay(fb);
            
            // 3. Draw desktop icons
            for icon in &icons {
                draw_desktop_icon(fb, icon);
            }

            // 4. Draw all windows
            wm.draw_all(fb);

            // 5. Draw application content inside windows
            for win in &wm.windows {
                if win.visible && !win.minimized {
                    let (cx, cy, cw, ch) = win.content_rect();
                    draw_app_content(fb, win.app_type, cx, cy, cw, ch);
                }
            }

            // 6. Draw taskbar
            taskbar.draw(fb, &wm);

            // 7. Swap buffers
            fb.swap_buffers();
        }
        drop(fb_lock);

        // Handle keyboard input
        if let Some(event) = keyboard::read_key() {
            if let Some(ch) = event.character {
                handle_keyboard_input(&mut wm, &mut taskbar, ch, &event);
            }
        }

        // Small delay to prevent 100% CPU
        for _ in 0..10000 {
            x86_64::instructions::nop();
        }
    }
}

/// Draw the desktop wallpaper (gradient + pattern)
fn draw_wallpaper(fb: &mut FrameBuffer) {
    for y in 0..fb.height {
        for x in 0..fb.width {
            // Dark gradient from top-left to bottom-right
            let r = (5.0 + (y as f32 / fb.height as f32) * 8.0) as u8;
            let g = (5.0 + (y as f32 / fb.height as f32) * 6.0) as u8;
            let b = (12.0 + (x as f32 / fb.width as f32) * 15.0 + (y as f32 / fb.height as f32) * 8.0) as u8;
            fb.put_pixel(x, y, Color::new(r, g, b));
        }
    }
}

/// Draw subtle grid overlay
fn draw_grid_overlay(fb: &mut FrameBuffer) {
    let grid_color = Color::with_alpha(0, 240, 255, 6);
    for y in (0..fb.height).step_by(100) {
        for x in 0..fb.width {
            let existing = fb.get_pixel(x, y);
            fb.put_pixel(x, y, Color::new(
                existing.r.saturating_add(2),
                existing.g.saturating_add(3),
                existing.b.saturating_add(4),
            ));
        }
    }
    for x in (0..fb.width).step_by(100) {
        for y in 0..fb.height {
            let existing = fb.get_pixel(x, y);
            fb.put_pixel(x, y, Color::new(
                existing.r.saturating_add(2),
                existing.g.saturating_add(3),
                existing.b.saturating_add(4),
            ));
        }
    }
}

/// Draw a desktop icon
fn draw_desktop_icon(fb: &mut FrameBuffer, icon: &DesktopIcon) {
    // Icon background (hover effect would need mouse support)
    let icon_center_x = icon.x + ICON_SIZE / 2;
    
    // Draw icon shape based on app type
    match icon.app_type {
        AppType::Terminal => {
            // Terminal icon: ">_" in a box
            fb.draw_rect(icon.x + 8, icon.y + 4, 32, 24, palette::ACCENT);
            fb.draw_text(icon.x + 14, icon.y + 10, ">_", palette::ACCENT, 1);
        }
        AppType::FileManager => {
            // Folder icon
            fb.fill_rounded_rect(icon.x + 8, icon.y + 10, 32, 20, 3, palette::ACCENT);
            fb.fill_rect(icon.x + 8, icon.y + 4, 16, 8, palette::ACCENT);
        }
        AppType::TextEditor => {
            // Document icon
            fb.fill_rounded_rect(icon.x + 12, icon.y + 2, 24, 30, 2, Color::new(30, 30, 45));
            fb.draw_rect(icon.x + 12, icon.y + 2, 24, 30, palette::ACCENT);
            fb.draw_hline(icon.x + 16, icon.y + 10, 16, palette::TEXT_MUTED);
            fb.draw_hline(icon.x + 16, icon.y + 16, 16, palette::TEXT_MUTED);
            fb.draw_hline(icon.x + 16, icon.y + 22, 10, palette::TEXT_MUTED);
        }
        AppType::Calculator => {
            // Calculator icon
            fb.fill_rounded_rect(icon.x + 10, icon.y + 2, 28, 32, 3, Color::new(30, 30, 45));
            fb.draw_rect(icon.x + 10, icon.y + 2, 28, 32, palette::ACCENT);
            fb.fill_rect(icon.x + 14, icon.y + 6, 20, 8, palette::ACCENT);
            // Grid dots for buttons
            for row in 0..3 {
                for col in 0..3 {
                    fb.fill_rect(icon.x + 15 + col * 7, icon.y + 18 + row * 5, 4, 3, palette::TEXT_MUTED);
                }
            }
        }
        AppType::TaskManager => {
            // Chart icon
            fb.draw_rect(icon.x + 8, icon.y + 4, 32, 28, palette::ACCENT);
            // Bar chart bars
            fb.fill_rect(icon.x + 14, icon.y + 18, 5, 10, palette::ACCENT);
            fb.fill_rect(icon.x + 21, icon.y + 12, 5, 16, Color::new(0, 200, 210));
            fb.fill_rect(icon.x + 28, icon.y + 8, 5, 20, palette::ACCENT);
        }
        AppType::Settings => {
            // Gear icon (simplified with dots around center)
            let cx = icon.x + 24;
            let cy = icon.y + 16;
            // Draw 8 dots around center (pre-calculated positions)
            let offsets: [(isize, isize); 8] = [
                (0, -12), (8, -8), (12, 0), (8, 8),
                (0, 12), (-8, 8), (-12, 0), (-8, -8),
            ];
            for (dx, dy) in offsets.iter() {
                fb.put_pixel((cx as isize + dx) as usize, (cy as isize + dy) as usize, palette::ACCENT);
                fb.put_pixel((cx as isize + dx + 1) as usize, (cy as isize + dy) as usize, palette::ACCENT);
                fb.put_pixel((cx as isize + dx) as usize, (cy as isize + dy + 1) as usize, palette::ACCENT);
            }
            // Center circle
            for dy in -4isize..=4 {
                for dx in -4isize..=4 {
                    if dx*dx + dy*dy <= 16 {
                        fb.put_pixel((cx as isize + dx) as usize, (cy as isize + dy) as usize, palette::ACCENT);
                    }
                }
            }
        }
        _ => {}
    }
    
    // Icon label
    let text_w = FrameBuffer::text_width(icon.label, 1);
    let text_x = icon.x + (ICON_SIZE.saturating_sub(text_w)) / 2;
    fb.draw_text(text_x, icon.y + ICON_SIZE - 8, icon.label, palette::TEXT_PRIMARY, 1);
}

/// Draw application content inside a window
fn draw_app_content(fb: &mut FrameBuffer, app_type: AppType, x: usize, y: usize, w: usize, h: usize) {
    match app_type {
        AppType::Terminal => {
            // Terminal: black background with cyan text
            fb.fill_rect(x, y, w, h, palette::BLACK);
            fb.draw_text(x + 12, y + 12, "MineOS Terminal v1.0.0", palette::ACCENT, 1);
            fb.draw_text(x + 12, y + 32, "Type 'help' for commands.", palette::TEXT_SECONDARY, 1);
            fb.draw_text(x + 12, y + 56, "user@mineos:~$ _", palette::ACCENT, 1);
        }
        AppType::Calculator => {
            // Calculator display
            fb.fill_rect(x, y, w, 60, Color::new(8, 8, 14));
            fb.draw_text(x + w - 40, y + 20, "0", palette::TEXT_PRIMARY, 2);
            
            // Calculator buttons
            let btn_labels = [
                ["C", "+/-", "%", "/"],
                ["7", "8", "9", "*"],
                ["4", "5", "6", "-"],
                ["1", "2", "3", "+"],
                ["0", ".", "DEL", "="],
            ];
            
            let btn_w = (w - 20) / 4;
            let btn_h = (h - 80) / 5;
            
            for (row, labels) in btn_labels.iter().enumerate() {
                for (col, label) in labels.iter().enumerate() {
                    let bx = x + 4 + col * (btn_w + 2);
                    let by = y + 64 + row * (btn_h + 2);
                    
                    let bg = match *label {
                        "=" => palette::ACCENT,
                        "/" | "*" | "-" | "+" => Color::new(0, 30, 35),
                        "C" => Color::new(40, 15, 0),
                        _ => palette::SURFACE_HOVER,
                    };
                    let fg = match *label {
                        "=" => palette::BLACK,
                        "/" | "*" | "-" | "+" => palette::ACCENT,
                        "C" => palette::DANGER,
                        _ => palette::TEXT_PRIMARY,
                    };
                    
                    fb.fill_rounded_rect(bx, by, btn_w, btn_h, 4, bg);
                    let tw = FrameBuffer::text_width(label, 1);
                    fb.draw_text(bx + (btn_w - tw) / 2, by + (btn_h - 16) / 2, label, fg, 1);
                }
            }
        }
        AppType::TextEditor => {
            // Editor toolbar
            fb.fill_rect(x, y, w, 28, Color::new(8, 8, 14));
            fb.draw_text(x + 8, y + 6, "File  Edit  View", palette::TEXT_SECONDARY, 1);
            fb.draw_hline(x, y + 28, w, palette::WINDOW_BORDER);
            
            // Line numbers
            fb.fill_rect(x, y + 29, 40, h - 29, Color::new(10, 10, 16));
            for i in 1..=20 {
                let line_y = y + 29 + (i - 1) * 18;
                if line_y + 16 > y + h { break; }
                let num_str = alloc::format!("{:>3}", i);
                fb.draw_text(x + 4, line_y + 2, &num_str, palette::TEXT_MUTED, 1);
            }
            
            // Editor content area
            fb.fill_rect(x + 41, y + 29, w - 41, h - 29, Color::new(12, 12, 18));
            fb.draw_text(x + 48, y + 31, "Welcome to MineOS!", palette::TEXT_PRIMARY, 1);
            fb.draw_text(x + 48, y + 49, "", palette::TEXT_PRIMARY, 1);
            fb.draw_text(x + 48, y + 67, "This is your text editor.", palette::TEXT_SECONDARY, 1);
        }
        AppType::FileManager => {
            // Sidebar
            fb.fill_rect(x, y, 150, h, Color::new(10, 10, 16));
            fb.draw_vline(x + 150, y, h, palette::WINDOW_BORDER);
            
            let folders = ["Home", "Documents", "Pictures", "Music", "Downloads"];
            for (i, name) in folders.iter().enumerate() {
                let fy = y + 8 + i * 28;
                if i == 0 {
                    fb.fill_rect(x + 4, fy, 142, 24, Color::new(0, 25, 30));
                }
                fb.draw_text(x + 12, fy + 4, name, 
                    if i == 0 { palette::ACCENT } else { palette::TEXT_SECONDARY }, 1);
            }
            
            // File grid
            let files = ["Documents/", "Pictures/", "Music/", "Downloads/", "readme.txt", "notes.txt"];
            let grid_x = x + 158;
            for (i, name) in files.iter().enumerate() {
                let col = i % 4;
                let row = i / 4;
                let fx = grid_x + col * 100;
                let fy = y + 12 + row * 90;
                
                let is_folder = name.ends_with('/');
                let icon_color = if is_folder { palette::ACCENT } else { palette::TEXT_SECONDARY };
                
                // Simple icon
                if is_folder {
                    fb.fill_rounded_rect(fx + 16, fy + 4, 40, 28, 3, icon_color);
                } else {
                    fb.fill_rounded_rect(fx + 20, fy + 2, 32, 32, 2, Color::new(30, 30, 45));
                    fb.draw_rect(fx + 20, fy + 2, 32, 32, icon_color);
                }
                
                fb.draw_text(fx + 4, fy + 40, name.trim_end_matches('/'), palette::TEXT_PRIMARY, 1);
            }
        }
        AppType::TaskManager => {
            // Tabs
            fb.fill_rect(x, y, w, 32, Color::new(8, 8, 14));
            fb.draw_text(x + 12, y + 8, "Processes", palette::ACCENT, 1);
            fb.draw_hline(x + 8, y + 28, 80, palette::ACCENT);
            fb.draw_text(x + 100, y + 8, "Performance", palette::TEXT_SECONDARY, 1);
            fb.draw_hline(x, y + 32, w, palette::WINDOW_BORDER);
            
            // Stats header
            fb.draw_text(x + 12, y + 40, "CPU", palette::TEXT_MUTED, 1);
            fb.draw_text(x + 12, y + 58, "23%", palette::ACCENT, 2);
            
            fb.draw_text(x + w / 3, y + 40, "MEMORY", palette::TEXT_MUTED, 1);
            fb.draw_text(x + w / 3, y + 58, "42%", palette::ACCENT, 2);
            
            fb.draw_text(x + 2 * w / 3, y + 40, "DISK", palette::TEXT_MUTED, 1);
            fb.draw_text(x + 2 * w / 3, y + 58, "16%", palette::ACCENT, 2);
            
            // Process list header
            let list_y = y + 100;
            fb.fill_rect(x, list_y, w, 24, Color::new(8, 8, 14));
            fb.draw_text(x + 12, list_y + 4, "NAME", palette::TEXT_MUTED, 1);
            fb.draw_text(x + w / 2, list_y + 4, "STATUS", palette::TEXT_MUTED, 1);
            fb.draw_text(x + 3 * w / 4, list_y + 4, "MEMORY", palette::TEXT_MUTED, 1);
        }
        AppType::Settings => {
            // Settings sidebar
            fb.fill_rect(x, y, 160, h, Color::new(10, 10, 16));
            fb.draw_vline(x + 160, y, h, palette::WINDOW_BORDER);
            
            let sections = ["Display", "Appearance", "Sound", "About"];
            for (i, name) in sections.iter().enumerate() {
                let sy = y + 12 + i * 32;
                if i == 0 {
                    fb.fill_rect(x + 4, sy, 152, 28, Color::new(0, 25, 30));
                }
                fb.draw_text(x + 16, sy + 6, name, 
                    if i == 0 { palette::ACCENT } else { palette::TEXT_SECONDARY }, 1);
            }
            
            // Settings content
            fb.draw_text(x + 180, y + 16, "Display", palette::TEXT_PRIMARY, 2);
            
            fb.draw_text(x + 180, y + 60, "Animations", palette::TEXT_PRIMARY, 1);
            fb.draw_text(x + 180, y + 78, "Enable window animations", palette::TEXT_MUTED, 1);
            
            // Toggle switch
            let toggle_x = x + w - 60;
            fb.fill_rounded_rect(toggle_x, y + 64, 40, 22, 11, palette::ACCENT);
            // Toggle knob
            for dy in -8isize..=8 {
                for dx in -8isize..=8 {
                    if dx*dx + dy*dy <= 64 {
                        fb.put_pixel(
                            (toggle_x as isize + 30 + dx) as usize,
                            (y as isize + 75 + dy) as usize,
                            palette::BLACK,
                        );
                    }
                }
            }
        }
        _ => {
            fb.draw_text(x + 20, y + 20, "Application", palette::TEXT_PRIMARY, 1);
        }
    }
}

/// Handle keyboard input for the desktop
fn handle_keyboard_input(wm: &mut WindowManager, taskbar: &mut Taskbar, ch: char, event: &keyboard::KeyEvent) {
    // Keyboard shortcuts
    if event.ctrl {
        match ch {
            't' | 'T' => {
                wm.open("Terminal", 100, 50, 600, 400, AppType::Terminal);
            }
            'e' | 'E' => {
                wm.open("Text Editor", 150, 60, 650, 450, AppType::TextEditor);
            }
            'w' | 'W' => {
                // Close focused window
                if let Some(win) = wm.windows.iter().find(|w| w.focused) {
                    let id = win.id;
                    wm.close(id);
                }
            }
            _ => {}
        }
    }
    
    // Number keys to open apps
    if event.alt {
        match ch {
            '1' => { wm.open("Terminal", 100, 50, 600, 400, AppType::Terminal); }
            '2' => { wm.open("Files", 120, 60, 700, 450, AppType::FileManager); }
            '3' => { wm.open("Text Editor", 140, 70, 650, 420, AppType::TextEditor); }
            '4' => { wm.open("Calculator", 300, 100, 280, 400, AppType::Calculator); }
            '5' => { wm.open("Task Manager", 160, 80, 600, 400, AppType::TaskManager); }
            '6' => { wm.open("Settings", 180, 90, 600, 400, AppType::Settings); }
            _ => {}
        }
    }
}
