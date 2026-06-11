ENTRY(_start)

SECTIONS {
    . = 0xFFFFFFFF80000000;

    .limine_reqs : ALIGN(8) {
        KEEP(*(.limine_reqs_start))
        KEEP(*(.limine_reqs))
        KEEP(*(.limine_reqs_end))
    }

    .text : ALIGN(4K) {
        *(.text .text.*)
    }

    .rodata : ALIGN(4K) {
        *(.rodata .rodata.*)
    }

    .data : ALIGN(4K) {
        *(.data .data.*)
    }

    .bss : ALIGN(4K) {
        *(COMMON)
        *(.bss .bss.*)
    }
}