#![no_std]
#![no_main]

pub mod arch;

use core::arch::asm;
use core::panic::PanicInfo;

use limine::request::{
    FramebufferRequest, HhdmRequest, MemmapRequest, ModulesRequest, StackSizeRequest,
};
use limine::{BaseRevision, RequestsEndMarker, RequestsStartMarker};

#[used]
#[link_section = ".limine_reqs_start"]
static START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[link_section = ".limine_reqs"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[link_section = ".limine_reqs"]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[link_section = ".limine_reqs"]
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

#[used]
#[link_section = ".limine_reqs"]
static MEMORY_MAP_REQUEST: MemmapRequest = MemmapRequest::new();

#[used]
#[link_section = ".limine_reqs"]
static MODULE_REQUEST: ModulesRequest = ModulesRequest::new();

#[used]
#[link_section = ".limine_reqs"]
static STACK_SIZE_REQUEST: StackSizeRequest = StackSizeRequest::new(32768);

#[used]
#[link_section = ".limine_reqs_end"]
static END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop { unsafe { asm!("hlt"); } }
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    unsafe {
        asm!("out dx, al", in("dx") 0x3FBu16, in("al") 0x80u8);
        asm!("out dx, al", in("dx") 0x3F8u16, in("al") 0x01u8);
        asm!("out dx, al", in("dx") 0x3F9u16, in("al") 0x00u8);
        asm!("out dx, al", in("dx") 0x3FBu16, in("al") 0x03u8);
        asm!("out dx, al", in("dx") 0x3FAu16, in("al") 0xC7u8);
        asm!("out dx, al", in("dx") 0x3FCu16, in("al") 0x0Bu8);
    }
    let lsr: u8;
    unsafe { asm!("in al, dx", out("al") lsr, in("dx") 0x3FDu16); }
    if lsr & 0x20 != 0 {
        unsafe { asm!("out dx, al", in("dx") 0x3F8u16, in("al") b'F'); }
    }
    loop { unsafe { asm!("hlt"); } }
}

core::arch::global_asm!(
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