#![no_std]
#![no_main]
mod sbi;
mod console;
mod lang_items;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
fn rust_main(){
    clean_memory();

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

