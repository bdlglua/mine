// =============================================================================
// MineOS - GUI Module
// =============================================================================
// The graphical user interface system for MineOS.
// Provides the desktop environment, window management, and widget rendering.
// =============================================================================

pub mod desktop;
pub mod window;
pub mod taskbar;
pub mod font;
pub mod widgets;

use crate::drivers::framebuffer::{FrameBuffer, FB};

/// Initialize the GUI subsystem
pub fn init(fb: FrameBuffer) {
    *FB.lock() = Some(fb);
}
