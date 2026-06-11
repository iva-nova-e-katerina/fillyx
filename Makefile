TARGET := x86_64-unknown-none
KERNEL_DIR := kernel
BUILD_DIR := $(KERNEL_DIR)/target/$(TARGET)/release
ISO_DIR := iso
TOOLS_DIR := tools/limine-binary

KERNEL_ELF := $(BUILD_DIR)/fillyx-kernel
ISO_FILE := fillyx.iso

export RUSTFLAGS := -Crelocation-model=static -Clink-arg=-T$(CURDIR)/kernel/linker.ld -Clink-arg=--no-dynamic-linker -Clink-arg=--no-pie

.PHONY: all kernel iso run clean

all: kernel iso

kernel:
	. "$$HOME/.cargo/env" && cargo build --release --target $(TARGET) --manifest-path $(KERNEL_DIR)/Cargo.toml 2>&1
	cp $(KERNEL_ELF) $(ISO_DIR)/boot/fillyx-kernel.elf

iso: kernel
	xorriso -as mkisofs -b boot/limine-bios-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot boot/limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		$(ISO_DIR) -o $(ISO_FILE) 2>&1
	$(TOOLS_DIR)/limine bios-install $(ISO_FILE) 2>&1

run: iso
	qemu-system-x86_64 -cdrom $(ISO_FILE) \
		-serial stdio \
		-m 256M

clean:
	cargo clean --manifest-path $(KERNEL_DIR)/Cargo.toml
	rm -f $(ISO_FILE)
	rm -rf $(ISO_DIR)/boot/fillyx-kernel.elf