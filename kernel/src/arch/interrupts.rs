use core::arch::asm;

use crate::arch::serial;

#[no_mangle]
pub extern "C" fn exception_handler(vector: u64, _error_code: u64, _rip: u64, _cs: u64, _rflags: u64) -> ! {
    serial::write(b"[PANIC] Exception #");
    let mut n = vector;
    let mut buf = [0u8; 4];
    let mut i = 0;
    if n == 0 { buf[0] = b'0'; i = 1; } else { while n > 0 { buf[i] = b'0' + (n % 10) as u8; n /= 10; i += 1; } }
    for b in buf[..i].iter().rev() { serial::write_byte(*b); }
    serial::write(b"\nSystem halted.\n");
    loop { unsafe { asm!("hlt"); } }
}

#[no_mangle]
pub extern "C" fn syscall_handler(number: u64, _arg1: u64, _arg2: u64, _arg3: u64) -> u64 {
    serial::write(b"syscall #");
    let mut n = number;
    let mut buf = [0u8; 4];
    let mut i = 0;
    if n == 0 { buf[0] = b'0'; i = 1; } else { while n > 0 { buf[i] = b'0' + (n % 10) as u8; n /= 10; i += 1; } }
    for b in buf[..i].iter().rev() { serial::write_byte(*b); }
    serial::write(b"\n");
    0
}

core::arch::global_asm!(
    ".macro def_handler name",
    ".globl \\name",
    "\\name:",
    "push 0",
    "jmp exception_common",
    ".endm",

    ".macro def_handler_err name",
    ".globl \\name",
    "\\name:",
    "jmp exception_common",
    ".endm",

    "def_handler divide_error_handler",
    "def_handler debug_handler",
    "def_handler non_maskable_interrupt_handler",
    "def_handler breakpoint_handler",
    "def_handler overflow_handler",
    "def_handler bound_range_exceeded_handler",
    "def_handler invalid_opcode_handler",
    "def_handler device_not_available_handler",
    "def_handler_err double_fault_handler",
    "def_handler invalid_tss_handler",
    "def_handler_err segment_not_present_handler",
    "def_handler_err stack_segment_fault_handler",
    "def_handler_err general_protection_fault_handler",
    "def_handler_err page_fault_handler",
    "def_handler floating_point_error_handler",
    "def_handler_err alignment_check_handler",
    "def_handler machine_check_handler",
    "def_handler simd_floating_point_exception_handler",
    "def_handler virtualization_exception_handler",
    "def_handler_err security_exception_handler",

    ".globl timer_interrupt_handler",
    "timer_interrupt_handler:",
    "push rax",
    "push rcx",
    "push rdx",
    "mov al, 0x20",
    "out 0x20, al",
    "pop rdx",
    "pop rcx",
    "pop rax",
    "iretq",

    ".globl keyboard_interrupt_handler",
    "keyboard_interrupt_handler:",
    "push rax",
    "in al, 0x60",
    "mov al, 0x20",
    "out 0x20, al",
    "pop rax",
    "iretq",

    ".globl syscall_dispatcher",
    "syscall_dispatcher:",
    "push rbx",
    "push rcx",
    "push rdx",
    "push rsi",
    "push rdi",
    "push r8",
    "push r9",
    "push r10",
    "push r11",
    "mov rdi, rax",
    "mov rsi, rbx",
    "mov rdx, rcx",
    "mov rcx, r11",
    "call syscall_handler",
    "pop r11",
    "pop r10",
    "pop r9",
    "pop r8",
    "pop rdi",
    "pop rsi",
    "pop rdx",
    "pop rcx",
    "pop rbx",
    "iretq",

    ".globl exception_common",
    "exception_common:",
    "push rbx",
    "push rcx",
    "push rdx",
    "push rsi",
    "push rdi",
    "push r8",
    "push r9",
    "push r10",
    "push r11",
    "mov rdi, rax",
    "mov rsi, [rsp + 9*8]",
    "mov rdx, [rsp + 10*8]",
    "mov rcx, [rsp + 11*8]",
    "mov r8, [rsp + 12*8]",
    "call exception_handler",
    "pop r11",
    "pop r10",
    "pop r9",
    "pop r8",
    "pop rdi",
    "pop rsi",
    "pop rdx",
    "pop rcx",
    "pop rbx",
    "pop rax",
    "add rsp, 8",
    "iretq",
);

pub fn init() {
    serial::write(b"Interrupts... ");
    unsafe {
        asm!(
            "mov al, 0x11",
            "out 0x20, al",
            "out 0xA0, al",
            "mov al, 0x20",
            "out 0x21, al",
            "mov al, 0x28",
            "out 0xA1, al",
            "mov al, 0x04",
            "out 0x21, al",
            "mov al, 0x02",
            "out 0xA1, al",
            "mov al, 0x01",
            "out 0x21, al",
            "out 0xA1, al",
            "mov al, 0x00",
            "out 0x21, al",
            "out 0xA1, al",
            options(preserves_flags),
        );
        asm!("sti");
    }
    serial::write(b"[OK]\n");
}