#![feature(linkage)]
#![no_main]
#![no_std]
// use console;
mod envset; 

#[no_mangle]
fn _start() {
    envset::bss_init();
}



#[linkage = "weak"]
#[no_mangle]
fn main() -> () {
    // println!("There is not main function, exiting...");
}