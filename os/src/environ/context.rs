use core::mem::size_of;

const KERNEL_STACK_SIZE: usize = 2 * 4096;
const USER_STACK_SIZE: usize = 2 * 4096;

struct KernelStack {
    sp: usize,
    data: [u8; KERNEL_STACK_SIZE],
}

static KERNEL_STACK: KernelStack = KernelStack {
    sp: 0,
    data: [0; KERNEL_STACK_SIZE],
};

impl KernelStack {
    fn init_sp(&mut self) {
        self.sp = self.get_bp();
    }

    pub fn get_sp(&self) -> usize {
        self.sp
    }

    pub fn get_bp(&self) -> usize {
        self.data.as_ptr() as usize + size_of::<KernelStack>() - size_of::<usize>() 
    }

    pub fn push<T>(&mut self, object: T) {
        let addr = (self.get_sp() - size_of::<T>()) as *mut T;
        unsafe {
            *addr = object;
        }
        self.sp -= size_of::<T>();
    }
}


struct UserStack {
    sp: usize,
    data: [u8; USER_STACK_SIZE],
}

static USER_STACK: UserStack = UserStack {
    sp: 0,
    data: [0; USER_STACK_SIZE],
};

impl UserStack {
    pub fn init_sp(&mut self) {
        self.sp = self.get_bp();
    }

    pub fn get_sp(&self) -> usize {
        self.sp
    }

    pub fn get_bp(&self) -> usize {
        self.data.as_ptr() as usize + size_of::<UserStack>() - size_of::<usize>()     
    }
}