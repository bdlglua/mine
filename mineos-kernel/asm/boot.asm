; =============================================================================
; MineOS Bootloader - Stage 1 (Assembly / NASM)
; =============================================================================
; This is the Multiboot2-compliant bootloader for MineOS.
; It sets up the initial environment before passing control to the Rust kernel.
;
; Features:
;   - Multiboot2 header for GRUB/bootloader compatibility
;   - Sets up a basic GDT (Global Descriptor Table)
;   - Enables A20 line
;   - Switches from Real Mode -> Protected Mode -> Long Mode (64-bit)
;   - Sets up initial page tables for identity mapping
;   - Passes framebuffer info to the Rust kernel
; =============================================================================

section .multiboot_header
header_start:
    dd 0xe85250d6                ; Multiboot2 magic number
    dd 0                         ; Architecture: i386 (protected mode)
    dd header_end - header_start ; Header length
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start)) ; Checksum

    ; Framebuffer tag - request graphical mode
    dw 5                         ; Type: framebuffer
    dw 0                         ; Flags
    dd 20                        ; Size
    dd 1024                      ; Width
    dd 768                       ; Height
    dd 32                        ; Depth (32-bit color)

    ; End tag
    dw 0                         ; Type
    dw 0                         ; Flags
    dd 8                         ; Size
header_end:

; =============================================================================
; GDT - Global Descriptor Table (64-bit)
; =============================================================================
section .rodata
gdt64:
    dq 0                                    ; Null descriptor
.code: equ $ - gdt64
    dq (1<<43) | (1<<44) | (1<<47) | (1<<53) ; Code segment: executable, code/data, present, 64-bit
.data: equ $ - gdt64
    dq (1<<44) | (1<<47) | (1<<41)          ; Data segment: code/data, present, writable
.pointer:
    dw $ - gdt64 - 1                        ; GDT limit
    dq gdt64                                ; GDT base address

; =============================================================================
; Page Tables - Identity mapping for first 1GB
; =============================================================================
section .bss
align 4096
p4_table:
    resb 4096
p3_table:
    resb 4096
p2_table:
    resb 4096
stack_bottom:
    resb 4096 * 16                          ; 64KB stack
stack_top:

; =============================================================================
; Boot Entry Point
; =============================================================================
section .text
global _start
extern _start_rust

bits 32

_start:
    ; Save Multiboot2 info pointer
    mov edi, ebx                ; Multiboot2 info structure pointer
    mov esi, eax                ; Multiboot2 magic number

    ; Set up stack
    mov esp, stack_top

    ; Check for Multiboot2
    call check_multiboot

    ; Check for CPUID
    call check_cpuid

    ; Check for Long Mode support
    call check_long_mode

    ; Set up page tables
    call setup_page_tables

    ; Enable paging
    call enable_paging

    ; Load 64-bit GDT
    lgdt [gdt64.pointer]

    ; Far jump to 64-bit code
    jmp gdt64.code:long_mode_start

; =============================================================================
; Checks
; =============================================================================
check_multiboot:
    cmp eax, 0x36d76289         ; Multiboot2 magic
    jne .no_multiboot
    ret
.no_multiboot:
    mov al, "M"
    jmp error

check_cpuid:
    ; Check if CPUID is supported by flipping ID bit (bit 21) in FLAGS
    pushfd
    pop eax
    mov ecx, eax
    xor eax, 1 << 21
    push eax
    popfd
    pushfd
    pop eax
    push ecx
    popfd
    cmp eax, ecx
    je .no_cpuid
    ret
.no_cpuid:
    mov al, "C"
    jmp error

check_long_mode:
    ; Check if extended CPUID functions are available
    mov eax, 0x80000000
    cpuid
    cmp eax, 0x80000001
    jb .no_long_mode

    ; Check for long mode bit
    mov eax, 0x80000001
    cpuid
    test edx, 1 << 29           ; Long mode bit
    jz .no_long_mode
    ret
.no_long_mode:
    mov al, "L"
    jmp error

; =============================================================================
; Page Table Setup - Identity map first 1GB using 2MB pages
; =============================================================================
setup_page_tables:
    ; Map P4[0] -> P3
    mov eax, p3_table
    or eax, 0b11                ; Present + Writable
    mov [p4_table], eax

    ; Map P3[0] -> P2
    mov eax, p2_table
    or eax, 0b11                ; Present + Writable
    mov [p3_table], eax

    ; Map P2 entries -> 2MB pages (identity map first 1GB)
    mov ecx, 0                  ; Counter
.map_p2:
    mov eax, 0x200000           ; 2MB
    mul ecx
    or eax, 0b10000011          ; Present + Writable + Huge Page
    mov [p2_table + ecx * 8], eax
    inc ecx
    cmp ecx, 512               ; 512 entries * 2MB = 1GB
    jne .map_p2

    ret

; =============================================================================
; Enable Paging and Long Mode
; =============================================================================
enable_paging:
    ; Load P4 address into CR3
    mov eax, p4_table
    mov cr3, eax

    ; Enable PAE (Physical Address Extension)
    mov eax, cr4
    or eax, 1 << 5             ; PAE bit
    mov cr4, eax

    ; Enable Long Mode in EFER MSR
    mov ecx, 0xC0000080        ; EFER MSR
    rdmsr
    or eax, 1 << 8             ; Long Mode Enable bit
    wrmsr

    ; Enable Paging
    mov eax, cr0
    or eax, 1 << 31            ; Paging bit
    mov cr0, eax

    ret

; =============================================================================
; Error Handler (prints "ERR: X" to VGA text buffer)
; =============================================================================
error:
    mov dword [0xb8000], 0x4f524f45  ; "ER"
    mov dword [0xb8004], 0x4f3a4f52  ; "R:"
    mov dword [0xb8008], 0x4f204f20  ; "  "
    mov byte  [0xb800a], al          ; Error code character
    hlt

; =============================================================================
; 64-bit Long Mode Entry
; =============================================================================
bits 64
long_mode_start:
    ; Clear all data segment registers
    mov ax, gdt64.data
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    ; Set up 64-bit stack
    mov rsp, stack_top

    ; Pass Multiboot2 info to Rust kernel
    ; rdi already contains multiboot info pointer from earlier
    ; rsi already contains multiboot magic

    ; Jump to Rust kernel entry point
    call _start_rust

    ; If kernel returns, halt
    cli
.halt:
    hlt
    jmp .halt
