ENTRY(_start)

SECTIONS {
    . = 0xFFFFFFFF80000000;

    .all : ALIGN(4K) {
        KEEP(*(.limine_reqs))
        . = ALIGN(4K);
        KEEP(*(.text.entry))
        *(.text .text.*)
        *(.rodata .rodata.*)
        *(.data .data.*)
        *(COMMON)
        *(.data.stack)
        *(.bss .bss.*)
        . += 0x100000;
    }
}