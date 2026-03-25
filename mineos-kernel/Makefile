# ==============================================================================
# MineOS Kernel - Makefile
# ==============================================================================
# Build system for compiling the MineOS bare-metal kernel.
#
# Requirements:
#   - Rust nightly toolchain with rust-src component
#   - NASM assembler
#   - cargo-bootimage tool
#   - QEMU (optional, for testing)
#
# Usage:
#   make build     - Compile the kernel
#   make run       - Build and run in QEMU
#   make clean     - Clean build artifacts
#   make iso       - Create bootable ISO (requires grub-mkrescue)
# ==============================================================================

.PHONY: build run clean iso asm check

# Paths
KERNEL_BIN = target/x86_64-unknown-none/release/mineos-kernel
BOOTIMAGE = target/x86_64-unknown-none/release/boot-mineos-kernel.bin
ASM_DIR = asm
ASM_OUT = target/asm

# NASM flags
NASM_FLAGS = -f elf64

# QEMU flags
QEMU = qemu-system-x86_64
QEMU_FLAGS = -drive format=raw,file=$(BOOTIMAGE) \
             -m 256M \
             -serial stdio \
             -display sdl

# ============================================================
# Build targets
# ============================================================

# Build the complete kernel
build: asm
	cargo bootimage --release

# Assemble the assembly files
asm: $(ASM_OUT)
	nasm $(NASM_FLAGS) $(ASM_DIR)/boot.asm -o $(ASM_OUT)/boot.o
	nasm $(NASM_FLAGS) $(ASM_DIR)/interrupts.asm -o $(ASM_OUT)/interrupts.o

$(ASM_OUT):
	mkdir -p $(ASM_OUT)

# Run in QEMU
run: build
	$(QEMU) $(QEMU_FLAGS)

# Run in QEMU with no graphical output (serial only)
run-serial: build
	$(QEMU) -drive format=raw,file=$(BOOTIMAGE) \
	        -m 256M \
	        -serial stdio \
	        -nographic

# Check compilation without building
check:
	cargo check

# Clean build artifacts
clean:
	cargo clean
	rm -rf $(ASM_OUT)

# Create bootable ISO image
iso: build
	mkdir -p target/iso/boot/grub
	cp $(KERNEL_BIN) target/iso/boot/mineos.bin
	echo 'set timeout=0' > target/iso/boot/grub/grub.cfg
	echo 'set default=0' >> target/iso/boot/grub/grub.cfg
	echo '' >> target/iso/boot/grub/grub.cfg
	echo 'menuentry "MineOS" {' >> target/iso/boot/grub/grub.cfg
	echo '    multiboot2 /boot/mineos.bin' >> target/iso/boot/grub/grub.cfg
	echo '}' >> target/iso/boot/grub/grub.cfg
	grub-mkrescue -o target/mineos.iso target/iso

# ============================================================
# Debug targets
# ============================================================

# Run with GDB debugging support
debug: build
	$(QEMU) $(QEMU_FLAGS) -s -S &
	@echo "QEMU started. Connect GDB with: target remote localhost:1234"

# Disassemble the kernel
disasm: build
	objdump -d $(KERNEL_BIN) | less
