.section .text.entry
    .globl _start
_start:
    la sp, stack_bottom
    call rust_main

.section .bss.stack
    .globl stack_top
stack_top:
    .space 4096 * 16
    .globl stack_bottom
stack_bottom: