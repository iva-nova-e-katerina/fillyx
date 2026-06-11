ENTRY(_start)

SECTIONS {
    . = 0xFFFFFFFF80000000;

    .all : {
        *(.limine_reqs_start .limine_reqs .limine_reqs_end)
        . = ALIGN(16);
        *(.rodata .rodata.*)
        . = ALIGN(16);
        *(.text .text.*)
        . = ALIGN(16);
        *(.got .got.*)
        . = ALIGN(16);
        *(.data .data.*)
        *(COMMON)
        *(.bss .bss.*)
    }
}