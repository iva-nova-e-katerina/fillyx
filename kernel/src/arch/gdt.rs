use core::arch::asm;

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    flags_limit_high: u8,
    base_high: u8,
}

#[repr(C, packed)]
struct Gdtr {
    limit: u16,
    base: u64,
}

const NULL: GdtEntry = GdtEntry { limit_low: 0, base_low: 0, base_mid: 0, access: 0, flags_limit_high: 0, base_high: 0 };
const KERNEL_CODE: GdtEntry = GdtEntry { limit_low: 0xFFFF, base_low: 0, base_mid: 0, access: 0x9A, flags_limit_high: 0xAF, base_high: 0 };
const KERNEL_DATA: GdtEntry = GdtEntry { limit_low: 0xFFFF, base_low: 0, base_mid: 0, access: 0x92, flags_limit_high: 0xAF, base_high: 0 };
const USER_CODE: GdtEntry = GdtEntry { limit_low: 0xFFFF, base_low: 0, base_mid: 0, access: 0xFA, flags_limit_high: 0xAF, base_high: 0 };
const USER_DATA: GdtEntry = GdtEntry { limit_low: 0xFFFF, base_low: 0, base_mid: 0, access: 0xF2, flags_limit_high: 0xAF, base_high: 0 };

#[used]
#[link_section = ".data"]
static GDT: [GdtEntry; 5] = [NULL, KERNEL_CODE, KERNEL_DATA, USER_CODE, USER_DATA];

pub fn init() {
    let gdtr = Gdtr {
        limit: (core::mem::size_of::<[GdtEntry; 5]>() - 1) as u16,
        base: &GDT as *const _ as u64,
    };
    unsafe {
        asm!("lgdt [{}]", in(reg) &gdtr, options(readonly, nostack));
        asm!(
            "mov ax, 0x10",
            "mov ds, ax",
            "mov es, ax",
            "mov fs, ax",
            "mov gs, ax",
            "mov ss, ax",
            "push 0x08",
            "lea rax, [rip + 9f]",
            "push rax",
            "retfq",
            "9:",
            options(preserves_flags),
        );
    }
}