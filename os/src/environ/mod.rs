mod context;

pub fn clean_memory(){
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