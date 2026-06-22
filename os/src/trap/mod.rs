// Print macro exported at crate root
use crate::println;
use core::arch::global_asm;
use crate::sbi::set_timer;
use riscv::register::{
    stvec,
    stvec::TrapMode,
    sstatus::Sstatus,
    scause::Trap,
    stval,
    sie,
    time
};

// Import trap entry assembly
global_asm!(include_str!("trap.S"));

/// Initialize trap vector base address
pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    let trap_addr = __alltraps as usize;
    // Correct 2-argument Stvec::new + direct CSR write for riscv 0.15.0
    let stvec_val = stvec::Stvec::new(trap_addr, TrapMode::Direct);
    unsafe {
        stvec::write(stvec_val);
    }
}

/// Enable supervisor timer interrupt
pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

/// Disable supervisor timer interrupt
#[allow(unused)]
pub fn disable_timer_interrupt() {
    unsafe {
        sie::clear_stimer();
    }
}

/// Trap context save/restore structure
#[repr(C)]
#[derive(Debug)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
}

/// Global trap entry handler invoked from assembly __alltraps
#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause_val = riscv::register::scause::read();

    match scause_val.cause() {
        Trap::Interrupt(_) => {
            let now = time::read();
            set_timer(now + 1_000_000);
            crate::timer::set_next_trigger();
        }
        Trap::Exception(_) => {
            let val = stval::read();
            println!("[kernel] Exception, stval = {:#x}", val);
            panic!("Unhandled CPU Exception");
        }
    }

    cx
}