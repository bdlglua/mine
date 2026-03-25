// =============================================================================
// MineOS - PIC (Programmable Interrupt Controller) Driver
// =============================================================================
// Manages the Intel 8259 PIC for hardware interrupt routing.
// =============================================================================

use pic8259::ChainedPics;
use spin::Mutex;

/// PIC interrupt offset (remapped from 0-15 to 32-47)
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

/// Hardware interrupt indices
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard = PIC_1_OFFSET + 1,
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_usize(self) -> usize {
        self as usize
    }
}

/// Chained PIC instance
static PICS: Mutex<ChainedPics> = Mutex::new(unsafe {
    ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)
});

/// Initialize the PIC
pub fn init() {
    unsafe {
        PICS.lock().initialize();
    }
}

/// Send End-of-Interrupt signal
pub fn end_of_interrupt(interrupt_id: u8) {
    unsafe {
        PICS.lock().notify_end_of_interrupt(interrupt_id);
    }
}
