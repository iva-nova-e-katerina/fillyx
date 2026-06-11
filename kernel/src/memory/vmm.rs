use core::arch::asm;

pub fn init() {
    unsafe {
        asm!(
            "mov rax, cr0",
            "or al, 0x01",
            "mov cr0, rax",
            options(preserves_flags),
        );
    }
}