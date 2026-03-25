; =============================================================================
; MineOS - Interrupt Service Routine Stubs (Assembly)
; =============================================================================
; These are the low-level ISR stubs that save CPU state before calling
; the Rust interrupt handlers, then restore state and return.
; =============================================================================

section .text
bits 64

; Macro for ISR that does NOT push an error code
%macro ISR_NOERRCODE 1
global isr_%1
isr_%1:
    push 0                      ; Push dummy error code
    push %1                     ; Push interrupt number
    jmp isr_common_stub
%endmacro

; Macro for ISR that DOES push an error code
%macro ISR_ERRCODE 1
global isr_%1
isr_%1:
    push %1                     ; Push interrupt number (error code already pushed by CPU)
    jmp isr_common_stub
%endmacro

; =============================================================================
; Exception ISRs (0-31)
; =============================================================================
ISR_NOERRCODE 0                 ; Division by Zero
ISR_NOERRCODE 1                 ; Debug
ISR_NOERRCODE 2                 ; Non-Maskable Interrupt
ISR_NOERRCODE 3                 ; Breakpoint
ISR_NOERRCODE 4                 ; Overflow
ISR_NOERRCODE 5                 ; Bound Range Exceeded
ISR_NOERRCODE 6                 ; Invalid Opcode
ISR_NOERRCODE 7                 ; Device Not Available
ISR_ERRCODE   8                 ; Double Fault
ISR_NOERRCODE 9                 ; Coprocessor Segment Overrun
ISR_ERRCODE   10                ; Invalid TSS
ISR_ERRCODE   11                ; Segment Not Present
ISR_ERRCODE   12                ; Stack Fault
ISR_ERRCODE   13                ; General Protection Fault
ISR_ERRCODE   14                ; Page Fault
ISR_NOERRCODE 15                ; Reserved
ISR_NOERRCODE 16                ; x87 FPU Error
ISR_ERRCODE   17                ; Alignment Check
ISR_NOERRCODE 18                ; Machine Check
ISR_NOERRCODE 19                ; SIMD Floating-Point

; =============================================================================
; Hardware IRQs (32-47) - Remapped via PIC
; =============================================================================
ISR_NOERRCODE 32                ; Timer (PIT)
ISR_NOERRCODE 33                ; Keyboard (PS/2)
ISR_NOERRCODE 34                ; Cascade
ISR_NOERRCODE 35                ; COM2
ISR_NOERRCODE 36                ; COM1
ISR_NOERRCODE 37                ; LPT2
ISR_NOERRCODE 38                ; Floppy Disk
ISR_NOERRCODE 39                ; LPT1
ISR_NOERRCODE 40                ; CMOS RTC
ISR_NOERRCODE 41                ; Free
ISR_NOERRCODE 42                ; Free
ISR_NOERRCODE 43                ; Free
ISR_NOERRCODE 44                ; Mouse (PS/2)
ISR_NOERRCODE 45                ; FPU
ISR_NOERRCODE 46                ; Primary ATA
ISR_NOERRCODE 47                ; Secondary ATA

; =============================================================================
; Common ISR Stub - Saves state, calls Rust handler, restores state
; =============================================================================
extern rust_interrupt_handler

isr_common_stub:
    ; Save all general-purpose registers
    push rax
    push rbx
    push rcx
    push rdx
    push rsi
    push rdi
    push rbp
    push r8
    push r9
    push r10
    push r11
    push r12
    push r13
    push r14
    push r15

    ; Save segment registers
    mov ax, ds
    push rax

    ; Load kernel data segment
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; Pass interrupt frame pointer to Rust handler
    mov rdi, rsp
    call rust_interrupt_handler

    ; Restore segment registers
    pop rax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; Restore general-purpose registers
    pop r15
    pop r14
    pop r13
    pop r12
    pop r11
    pop r10
    pop r9
    pop r8
    pop rbp
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    pop rbx
    pop rax

    ; Remove interrupt number and error code from stack
    add rsp, 16

    ; Return from interrupt
    iretq

; =============================================================================
; Port I/O Helper Functions (used by Rust via FFI)
; =============================================================================
global asm_outb
global asm_inb
global asm_outw
global asm_inw
global asm_io_wait

; void asm_outb(uint16_t port, uint8_t value)
asm_outb:
    mov dx, di          ; port (first argument, lower 16 bits of rdi)
    mov al, sil         ; value (second argument, lower 8 bits of rsi)
    out dx, al
    ret

; uint8_t asm_inb(uint16_t port)
asm_inb:
    mov dx, di          ; port
    in al, dx
    movzx eax, al
    ret

; void asm_outw(uint16_t port, uint16_t value)
asm_outw:
    mov dx, di
    mov ax, si
    out dx, ax
    ret

; uint16_t asm_inw(uint16_t port)
asm_inw:
    mov dx, di
    in ax, dx
    movzx eax, ax
    ret

; void asm_io_wait(void)
asm_io_wait:
    out 0x80, al        ; Write to unused port for ~1µs delay
    ret
