ENTRY(_start)

SECTIONS {
    . = 0xFFFFFFFF80000000;

    .limine_reqs_start : ALIGN(8) { KEEP(*(.limine_reqs_start)) }
    .limine_reqs      : ALIGN(8) { KEEP(*(.limine_reqs))      }
    .limine_reqs_end  : ALIGN(8) { KEEP(*(.limine_reqs_end))  }

    . = ALIGN(4K);

    .text.entry : ALIGN(4K) { KEEP(*(.text.entry)) }

    .text : ALIGN(4K) { *(.text .text.*) }

    .rodata : ALIGN(4K) { *(.rodata .rodata.*) }

    .data : ALIGN(4K) {
        *(.data .data.*)
        *(.data.stack)
        *(.got .got.*)
        *(COMMON)
    }

    .bss : ALIGN(4K) {
        _bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        . += 0x80000;
        _bss_end = .;
    }
}