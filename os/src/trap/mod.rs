use core::arch::global_asm;
use riscv::register::scause::{Trap, Exception};
use riscv::register::utvec::TrapMode;
use riscv::register::{sstatus, scause, stval, stvec};
use riscv::register::sstatus::Sstatus;
use context::TrapContext;

mod context;

global_asm!(include_str!("Trap.S"));

pub fn trap_context_init() {
    extern "C" {
        fn __trapall();
    }
    unsafe{
        stvec::write(__trapall as usize, TrapMode::Direct);
    }
}

#[no_mangle]
fn trap_handler(tc: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            //TODO syscall
        },
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[KERNEL] Illegal Instruction, killed.\nStart running next application");
            //TODO run next
        },
        _ => {
            panic!(
                "[KERNEL]Unknown Trap {:#?}:{:#?}.\nPaniced!",
                scause.cause(),
                stval
            );
        },
    }
    return tc;
}