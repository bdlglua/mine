// =============================================================================
// MineOS - GUI Module
// =============================================================================

#![allow(dead_code)]

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
