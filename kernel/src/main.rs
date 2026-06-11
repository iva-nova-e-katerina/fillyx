#![no_std]
#![no_main]

use core::arch::asm;
use limine::BaseRevision;
use limine::RequestsStartMarker;
use limine::RequestsEndMarker;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop { unsafe { asm!("hlt"); } }
}

#[used]
#[no_mangle]
#[link_section = ".limine_reqs"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[no_mangle]
#[link_section = ".limine_reqs"]
static START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[no_mangle]
#[link_section = ".limine_reqs"]
static END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

fn serial_init() {
    unsafe {
        asm!("out dx, al", in("dx") 0x3FBu16, in("al") 0x80u8);
        asm!("out dx, al", in("dx") 0x3F8u16, in("al") 0x01u8);
        asm!("out dx, al", in("dx") 0x3F9u16, in("al") 0x00u8);
        asm!("out dx, al", in("dx") 0x3FBu16, in("al") 0x03u8);
        asm!("out dx, al", in("dx") 0x3FAu16, in("al") 0x00u8);
        asm!("out dx, al", in("dx") 0x3FCu16, in("al") 0x00u8);
    }
}

fn serial_tx_ready() -> bool {
    let result: u8;
    unsafe {
        asm!("in al, dx", out("al") result, in("dx") 0x3FDu16);
    }
    result & 0x20 != 0
}

fn serial_write(byte: u8) {
    while !serial_tx_ready() {
        unsafe { asm!("pause"); }
    }
    unsafe {
        asm!("out dx, al", in("dx") 0x3F8u16, in("al") byte);
    }
}

fn serial_write_str(s: &str) {
    for &byte in s.as_bytes() {
        if byte == b'\n' {
            serial_write(b'\r');
        }
        serial_write(byte);
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_init();
    serial_write_str("Fillyx v0.1 booting...\n");
    serial_write_str("Kernel ready.\n");
    loop { unsafe { asm!("hlt"); } }
}