.altmacro
.macro SAVE_REG n
    sd x\n, \n * 8(sp)
.endm

.macro RESTORE_REG n
    ld x\n, \n * 8 (sp)
.endm

.section .text:
    .align 2
    .globl __trapin
    .globl __restore
__trapin:
    csrrw sp, sscratch, sp;
    addi sp, sp, -8 * 34; # sp = sp + -8 * 33
    .set n, 1
    .rept 31
        SAVE_REG %n;
        .set n, n+1;
    .endr
    csrr t0, sscratch;
    csrr t1, sstatus;
    csrr t2, sepc;
    sd t0, 8 * 2(sp);
    sd t1, 8 * 32(sp);
    sd t2, 8 * 33(sp);
    mv  a0, sp # a0 = sp
    //sp : head addr of TrapContext
    //as the first argument of func trap_handler
    call trap_handler;

__restore:
    // 1.func trap_handler returns the mut borrow of TrapContext
    //  return value a0 pointer to the stack top before calling 
    //  func trap_handler
    // 2.the first argument of restore should be stack top pointer 
    mv  sp, a0 # sp = a0
    ld t0, 8 * 2(sp);
    ld t1, 8 * 32(sp);
    ld t2, 8 * 33(sp);
    // load user stack top pointer to CSR sscratch to switch with
    //  sp later
    csrw t0, sscratch;
    csrw t1, sscratch;
    csrw t2, sepc;
    ld x1, 1 * 8(sp);
    //override x2(sp), because sp is pointing to kernel stack now
    //if we override it, it would be difficult to restore the rest register
    .set n, 3;
    .rept 29
        RESTORE_REG %n;
        .set n, n+1;
    .endr
    addi sp, sp, 8 * 34; # sp = sp + 8 * 33
    csrrw sp, sscratch, sp;
    sret;

