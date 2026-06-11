use core::arch::asm;

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct IdtEntry {
    offset_low: u16,
    segment_selector: u16,
    ist: u8,
    type_attributes: u8,
    offset_mid: u16,
    offset_high: u32,
    zero: u32,
}

#[repr(C, packed)]
struct Idtr {
    limit: u16,
    base: u64,
}

const IDT_ENTRIES: usize = 256;

#[used]
#[link_section = ".data"]
static mut IDT: [IdtEntry; IDT_ENTRIES] = [IdtEntry { offset_low: 0, segment_selector: 0, ist: 0, type_attributes: 0, offset_mid: 0, offset_high: 0, zero: 0 }; IDT_ENTRIES];

fn set_entry(index: usize, handler: u64, type_attributes: u8) {
    unsafe {
        IDT[index] = IdtEntry {
            offset_low: handler as u16,
            segment_selector: 0x08,
            ist: 0,
            type_attributes,
            offset_mid: (handler >> 16) as u16,
            offset_high: (handler >> 32) as u32,
            zero: 0,
        };
    }
}

extern "C" {
    fn divide_error_handler();
    fn debug_handler();
    fn non_maskable_interrupt_handler();
    fn breakpoint_handler();
    fn overflow_handler();
    fn bound_range_exceeded_handler();
    fn invalid_opcode_handler();
    fn device_not_available_handler();
    fn double_fault_handler();
    fn invalid_tss_handler();
    fn segment_not_present_handler();
    fn stack_segment_fault_handler();
    fn general_protection_fault_handler();
    fn page_fault_handler();
    fn floating_point_error_handler();
    fn alignment_check_handler();
    fn machine_check_handler();
    fn simd_floating_point_exception_handler();
    fn virtualization_exception_handler();
    fn security_exception_handler();
    fn timer_interrupt_handler();
    fn keyboard_interrupt_handler();
    fn syscall_dispatcher();
}

pub fn init() {
    set_entry(0, divide_error_handler as u64, 0x8E);
    set_entry(1, debug_handler as u64, 0x8E);
    set_entry(2, non_maskable_interrupt_handler as u64, 0x8E);
    set_entry(3, breakpoint_handler as u64, 0x8E);
    set_entry(4, overflow_handler as u64, 0x8E);
    set_entry(5, bound_range_exceeded_handler as u64, 0x8E);
    set_entry(6, invalid_opcode_handler as u64, 0x8E);
    set_entry(7, device_not_available_handler as u64, 0x8E);
    set_entry(8, double_fault_handler as u64, 0x8E);
    set_entry(10, invalid_tss_handler as u64, 0x8E);
    set_entry(11, segment_not_present_handler as u64, 0x8E);
    set_entry(12, stack_segment_fault_handler as u64, 0x8E);
    set_entry(13, general_protection_fault_handler as u64, 0x8E);
    set_entry(14, page_fault_handler as u64, 0x8E);
    set_entry(16, floating_point_error_handler as u64, 0x8E);
    set_entry(17, alignment_check_handler as u64, 0x8E);
    set_entry(18, machine_check_handler as u64, 0x8E);
    set_entry(19, simd_floating_point_exception_handler as u64, 0x8E);
    set_entry(20, virtualization_exception_handler as u64, 0x8E);
    set_entry(30, security_exception_handler as u64, 0x8E);
    set_entry(32, timer_interrupt_handler as u64, 0x8E);
    set_entry(33, keyboard_interrupt_handler as u64, 0x8E);
    set_entry(0x80, syscall_dispatcher as u64, 0xEE);

    let idtr = Idtr {
        limit: (core::mem::size_of::<[IdtEntry; IDT_ENTRIES]>() - 1) as u16,
        base: unsafe { &IDT as *const _ as u64 },
    };
    unsafe {
        asm!("lidt [{}]", in(reg) &idtr, options(readonly, nostack));
    }
}