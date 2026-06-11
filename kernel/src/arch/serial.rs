use core::arch::asm;

const COM1: u16 = 0x3F8;

pub fn init() {
    unsafe {
        asm!("out dx, al", in("dx") COM1 + 1, in("al") 0x00u8);
        asm!("out dx, al", in("dx") COM1 + 3, in("al") 0x80u8);
        asm!("out dx, al", in("dx") COM1, in("al") 0x01u8);
        asm!("out dx, al", in("dx") COM1 + 1, in("al") 0x00u8);
        asm!("out dx, al", in("dx") COM1 + 3, in("al") 0x03u8);
        asm!("out dx, al", in("dx") COM1 + 2, in("al") 0xC7u8);
        asm!("out dx, al", in("dx") COM1 + 4, in("al") 0x0Bu8);
    }
}

fn is_transmit_empty() -> bool {
    let lsr: u8;
    unsafe {
        asm!("in al, dx", out("al") lsr, in("dx") COM1 + 5);
    }
    lsr & 0x20 != 0
}

pub fn write_byte(byte: u8) {
    while !is_transmit_empty() {
        unsafe { asm!("pause"); }
    }
    unsafe {
        asm!("out dx, al", in("dx") COM1, in("al") byte);
    }
}

pub fn write(data: &[u8]) {
    for &byte in data {
        write_byte(byte);
    }
}