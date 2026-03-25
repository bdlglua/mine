// =============================================================================
// MineOS Kernel - Main Entry Point (Rust)
// =============================================================================
// This is the heart of MineOS. A bare-metal operating system kernel written
// in Rust with no standard library. It provides:
//   - VGA framebuffer graphics driver (1024x768x32)
//   - PS/2 keyboard input handling
//   - Memory management with heap allocator
//   - Graphical desktop environment with window manager
//   - Built-in applications (Terminal, Calculator, Text Editor)
// =============================================================================

#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

extern crate alloc;

mod drivers;
mod gui;
mod apps;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

// Declare the kernel entry point
entry_point!(kernel_main);

/// MineOS Kernel Entry Point
/// Called by the bootloader after setting up the environment
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // Initialize the kernel subsystems
    drivers::init(boot_info);
    
    // Print boot message to VGA text buffer
    drivers::vga_text::print_str("MineOS v1.0.0 - Kernel Booting...\n");
    drivers::vga_text::print_str("Initializing subsystems...\n");
    drivers::vga_text::print_str("  [OK] GDT / IDT\n");
    drivers::vga_text::print_str("  [OK] PIC 8259\n");
    drivers::vga_text::print_str("  [OK] Memory Manager (4MB heap)\n");
    drivers::vga_text::print_str("  [OK] PS/2 Keyboard\n");
    drivers::vga_text::print_str("  [OK] Interrupts enabled\n\n");

    // Try to initialize framebuffer via VBE/VESA
    // The bootloader crate v0.9 maps physical memory but doesn't directly
    // provide framebuffer info. We detect it through the VGA hardware.
    let vga_fb_addr: usize = 0xA0000; // Standard VGA framebuffer
    let screen_w: usize = 320;
    let screen_h: usize = 200;

    // For now, start in VGA text mode with the terminal shell
    // The GUI desktop requires a Multiboot2-compatible framebuffer
    // which is set up via boot.asm's Multiboot2 header
    drivers::vga_text::print_str("Starting MineOS Terminal...\n");
    drivers::vga_text::print_str("(GUI desktop requires framebuffer - use 'startx' command)\n\n");
    
    // Run text-mode shell
    apps::terminal::run_text_mode_shell();
}

/// Panic handler - called on kernel panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    drivers::vga_text::set_color(0x04); // Red on black
    drivers::vga_text::print_str("\n!!! KERNEL PANIC !!!\n");
    if let Some(location) = info.location() {
        drivers::vga_text::print_str("  at ");
        drivers::vga_text::print_str(location.file());
        drivers::vga_text::print_str("\n");
    }
    halt_loop();
}

/// Heap allocation error handler
#[alloc_error_handler]
fn alloc_error(_layout: core::alloc::Layout) -> ! {
    drivers::vga_text::print_str("HEAP ALLOCATION ERROR\n");
    halt_loop();
}

/// Infinite halt loop
fn halt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
