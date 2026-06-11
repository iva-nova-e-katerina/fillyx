use core::arch::asm;

const COM1: u16 = 0x3F8;

pub fn init() {
    unsafe {
        asm!("mov dx, {}", in(reg) COM1 + 1, options(nostack, preserves_flags));
    }
}

fn is_transmit_empty() -> bool {
    let lsr: u8;
    unsafe {
        asm!("in al, dx", out("al") lsr, in("dx") COM1 + 5, options(nostack, preserves_flags));
    }
    lsr & 0x20 != 0
}

pub fn write_byte(byte: u8) {
    while !is_transmit_empty() {
        unsafe { asm!("pause", options(nostack)); }
    }
    unsafe {
        asm!("out dx, al", in("dx") COM1, in("al") byte, options(nostack, preserves_flags));
    }
}

pub fn write(data: &[u8]) {
    for &byte in data {
        write_byte(byte);
    }
}