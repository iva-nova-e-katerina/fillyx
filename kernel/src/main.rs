#![no_std]
#![no_main]

pub mod arch;

use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop { unsafe { asm!("hlt"); } }
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    arch::serial::init();
    arch::serial::write(b"Fillyx OS v0.1.0\n");
    loop { unsafe { asm!("hlt"); } }
}

core::arch::global_asm!(
    ".pushsection .limine_reqs, \"a\"",
    ".balign 8",
    ".globl req_ptr_array",
    "req_ptr_array:",
    ".quad base_revision_tag",
    ".quad 0",
    ".balign 8",
    "base_revision_tag:",
    ".quad 0xf9562b2d5c95a6c8",
    ".quad 0x6a7b384944536bdc",
    ".quad 0",
    ".popsection",

    ".globl _start",
    ".section .text.entry, \"ax\"",
    ".type _start, @function",
    "_start:",
    "cli",
    "lea rsp, [rip + _stack_end]",
    "and rsp, -16",
    "xor rbp, rbp",
    "call kernel_main",
    "cli",
    "1: hlt",
    "jmp 1b",

    ".pushsection .data.stack, \"aw\"",
    ".balign 4096",
    "_stack_begin:",
    ".space 16384",
    "_stack_end:",
    ".popsection",
);