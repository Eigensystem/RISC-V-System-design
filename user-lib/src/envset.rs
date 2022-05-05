
use core::slice;

pub fn bss_init() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe{
        slice::from_raw_parts_mut(sbss as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}
