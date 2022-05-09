#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![allow(unused)]
#[macro_use]
mod console;
mod sbi;
mod lang_items;
mod environ;
mod trap;
use core::arch::global_asm;
use crate::{console::print, sbi::shutdown, environ::clean_memory};

global_asm!(include_str!("entry.asm"));

#[no_mangle]
fn rust_main(){
    clean_memory();
    // trap::init();
}



