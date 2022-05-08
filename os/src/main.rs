#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![allow(unused)]
#[macro_use]
mod console;
mod sbi;
mod lang_items;

use core::arch::global_asm;

use crate::{console::print, sbi::shutdown};

global_asm!(include_str!("entry.asm"));

#[no_mangle]
fn rust_main(){
    clean_memory();
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn stack_top();
        fn stack_bottom();
    }
    println!("[KERNEL] .text range: {:#x} - {:#x}", stext as usize, etext as usize);
    println!("[KERNEL] .rodata range: {:#x} - {:#x}", srodata as usize, erodata as usize);
    println!("[KERNEL] .data range: {:#x} - {:#x}", sdata as usize, edata as usize);
    println!("[KERNEL] .bss range: {:#x} - {:#x}", sbss as usize, ebss as usize);
    println!("[KERNEL] kernel stack range: {:#x} - {:#x}", stack_top as usize, stack_bottom as usize);
    println!("MIN OPERATING ENV.");
    shutdown();
}

fn clean_memory(){
    extern "C" {
        fn sbss();
        fn ebss();
    }
    for i in sbss as usize..ebss as usize {
        unsafe {
            (i as *mut u8).write_volatile(0);
        }
    }
}

