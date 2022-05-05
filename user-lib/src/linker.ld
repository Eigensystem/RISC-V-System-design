OUTPUT_ARCH(RISCV)
ENTRY(_start)
BASE_ADDR = 0x80400000

SECTIONS
{
    . = BASE_ADDR;
    stext = .;
    .text : {
        *(.text .text.*)
    }
    etext = .;

    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }
    erodata = .;

    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }
    edata = .;

    sbss = .;
    .bss : {
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
    ebss = .;
}