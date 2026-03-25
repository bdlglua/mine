// =============================================================================
// MineOS - Drivers Module
// =============================================================================

pub mod vga_text;
pub mod framebuffer;
pub mod keyboard;
pub mod pic;
pub mod memory;

use bootloader::BootInfo;

/// Initialize all hardware drivers
pub fn init(boot_info: &'static BootInfo) {
    // Initialize GDT and IDT
    interrupts_init();
    
    // Initialize PIC (Programmable Interrupt Controller)
    pic::init();
    
    // Initialize memory management and heap allocator
    memory::init(boot_info);
    
    // Initialize keyboard driver
    keyboard::init();
    
    // Enable hardware interrupts
    x86_64::instructions::interrupts::enable();
}

/// Set up IDT (Interrupt Descriptor Table)
fn interrupts_init() {
    use x86_64::structures::idt::InterruptDescriptorTable;
    use lazy_static::lazy_static;
    
    lazy_static! {
        static ref IDT: InterruptDescriptorTable = {
            let mut idt = InterruptDescriptorTable::new();
            
            // CPU Exceptions
            idt.breakpoint.set_handler_fn(breakpoint_handler);
            idt.double_fault.set_handler_fn(double_fault_handler);
            idt.page_fault.set_handler_fn(page_fault_handler);
            
            // Hardware Interrupts (IRQs via PIC)
            idt[pic::InterruptIndex::Timer.as_usize()]
                .set_handler_fn(timer_interrupt_handler);
            idt[pic::InterruptIndex::Keyboard.as_usize()]
                .set_handler_fn(keyboard_interrupt_handler);
            
            idt
        };
    }
    
    IDT.load();
}

// =============================================================================
// Interrupt Handlers
// =============================================================================

use x86_64::structures::idt::InterruptStackFrame;

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    vga_text::print_str("EXCEPTION: BREAKPOINT\n");
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, 
    _error_code: u64
) -> ! {
    vga_text::print_str("EXCEPTION: DOUBLE FAULT\n");
    loop { x86_64::instructions::hlt(); }
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: x86_64::structures::idt::PageFaultErrorCode,
) {
    vga_text::print_str("EXCEPTION: PAGE FAULT\n");
    loop { x86_64::instructions::hlt(); }
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // Tick the GUI system
    gui_tick();
    
    // Send EOI (End of Interrupt) to PIC
    pic::end_of_interrupt(pic::InterruptIndex::Timer.as_u8());
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // Read scancode from PS/2 keyboard port
    let scancode: u8 = unsafe {
        x86_64::instructions::port::Port::new(0x60).read()
    };
    
    // Process the scancode
    keyboard::process_scancode(scancode);
    
    // Send EOI
    pic::end_of_interrupt(pic::InterruptIndex::Keyboard.as_u8());
}

/// Called by timer interrupt to update GUI
fn gui_tick() {
    // Update cursor blink, animations, etc.
    // This is called ~18.2 times per second (PIT default)
}
