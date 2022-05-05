OUTPUT_ARCH(RISCV)
ENTRY(_start)
BASE_ADDR = 0x80200000

SECTIONS
{
    . = BASE_ADDR;
    skernel = .;

    stext = .;
    .text : {
        *(.text_entry)
        *(.text .text.*)
    }
    . = ALIGN(4K);
    etext = .;

    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }
    . = ALIGN(4K);
    erodata = .;

    .data : {
        *(.stack)
        sdata = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
    }
    . = ALIGN(4K);
    edata = .;

    sbss = .;
    .bss : {
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
    . = ALIGN(4K);
    ebss = .;

    /DISCARD/ : {
        *.(eh_frame)
    }
}