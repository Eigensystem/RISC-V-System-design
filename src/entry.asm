    .section .text.entry
    .global _start
_start:
    la sp, boot_stack_top
    call rust_main

    .section .stack
    .globl boot_stack
boot_stack:
    .space 4096 * 16
    .globl boot_stack_top
boot_stack_top: