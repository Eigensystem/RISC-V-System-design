use riscv::register::{sstatus::{Sstatus,SPP, self}, sepc};
pub struct TrapContext {
    x: [usize; 32],
    sstatus: Sstatus,
    sepc: usize,
}

impl TrapContext {
    fn new_app_context(addr: usize) -> Self {
        let mut sstatus = sstatus::read();
        let sepc = addr;
        sstatus.set_spp(SPP::User);
        TrapContext { 
            x: [0; 32],
            sstatus,
            sepc,
        }
    }
}