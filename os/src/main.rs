#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![allow(static_mut_refs)]
#![allow(function_casts_as_integer)]

extern crate core;
extern crate alloc;
#[macro_use]
extern crate bitflags;

use core::arch::global_asm;

// Import assembly files
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

// All modules
mod config;
mod sbi;
mod timer;
mod trap;
mod task;
mod syscall;
mod loader;
mod mm;
mod sync;
#[macro_use]
mod console;

// Panic handler
use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Extern linker symbols (FIX: Use static for sbss/ebss/ekernel)
extern "C" {
    static sbss: u8;
    static ebss: u8;
    static ekernel: u8;
}

/// Clear BSS section
fn clear_bss() {
    unsafe {
        let start = &sbss as *const u8 as usize;
        let end = &ebss as *const u8 as usize;
        let bss = core::slice::from_raw_parts_mut(start as *mut u8, end - start);
        bss.fill(0);
    }
}

/// Kernel main entry
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    clear_bss();
    println!("[kernel] Hello, world!");

    // Initialize memory management (heap, frame, page table, address space)
    mm::init();
    println!("[kernel] back to world!");
    mm::remap_test();

    // Initialize trap & timer
    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();

    // Start first user task
    task::run_first_task();

    panic!("Unreachable in rust_main!");
}

// Boot stack
#[no_mangle]
static mut BOOT_STACK: [u8; 4096 * 4] = [0; 4096 * 4];
#[no_mangle]
static mut boot_stack_top: usize = 0;

#[no_mangle]
pub extern "C" fn init_stack() {
    unsafe {
        boot_stack_top = BOOT_STACK.as_ptr() as usize + BOOT_STACK.len();
    }
}