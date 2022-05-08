// #![feature(panic_info_message)]
use core::panic::PanicInfo;
use crate::{println, sbi::shutdown};


#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "[KERNEL] Paniced at file {}, {}:{}\nReason:{}",
            location.file(), location.line(), location.column(),
            info.message().unwrap()
        );
    }
    else{
        println!("[KERNEL] Paniced info : {}", info.message().unwrap());
    }
    shutdown()
}