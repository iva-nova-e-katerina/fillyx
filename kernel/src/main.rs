#![no_std]
#![no_main]

pub mod arch;
pub mod fs;
pub mod llm_hooks;
pub mod memory;
pub mod process;

use core::panic::PanicInfo;

use limine::request::{
    FramebufferRequest, HhdmRequest, MemmapRequest, ModulesRequest, StackSizeRequest,
};
use limine::{BaseRevision, RequestsEndMarker, RequestsStartMarker};

#[used]
#[link_section = ".limine_reqs"]
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
#[link_section = ".limine_reqs"]
static END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    arch::serial::write(b"PANIC: ");
    if let Some(msg) = info.message().as_str() {
        arch::serial::write(msg.as_bytes());
    }
    arch::serial::write(b"\n");
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    arch::serial::init();
    arch::serial::write(b"Fillyx OS v0.1.0\n");

    let hhdm = HHDM_REQUEST
        .response()
        .expect("HHDM request failed");
    let hhdm_offset = hhdm.offset;

    let memory_map = MEMORY_MAP_REQUEST
        .response()
        .expect("Memory map request failed");
    let entries = memory_map.entries();

    memory::init(entries, hhdm_offset);
    arch::serial::write(b"[OK] Memory init\n");

    arch::gdt::init();
    arch::serial::write(b"[OK] GDT init\n");

    arch::idt::init();
    arch::serial::write(b"[OK] IDT init\n");

    arch::interrupts::init();
    arch::serial::write(b"[OK] Interrupts init\n");

    fs::init();
    arch::serial::write(b"[OK] VFS init\n");

    process::init();
    arch::serial::write(b"[OK] Process init\n");

    llm_hooks::init();
    arch::serial::write(b"[OK] LLM hooks init\n");

    let logo = b"\n   __ _ _ _      _\n  / _(_) | |    | |\n | |_ _| | |    | |_ _   _ _ __   ___\n |  _| | | |    | __| | | | '_ \\ / _ \\\n | | | | | |____| |_| |_| | |_) |  __/\n |_| |_|_|______|\\__|\\__, | .__/ \\___|\n                      __/ | |\n                     |___/|_|\n\n";
    arch::serial::write(logo);
    arch::serial::write(b"LLM-native OS ready.\n");

    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

core::arch::global_asm!(
    ".globl _start",
    ".section .text, \"ax\"",
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
    ".section .bss, \"aw\", @nobits",
    ".balign 16",
    "_stack_begin:",
    ".space 32768",
    "_stack_end:",
);