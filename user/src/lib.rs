#![no_std]

mod syscall;
pub use syscall::{sys_exit, sys_yield, sys_get_time};

pub fn get_time() -> isize {
    sys_get_time()
}

pub fn yield_() -> isize {
    sys_yield()
}

pub fn exit(exit_code: i32) -> ! {
    sys_exit(exit_code);
    loop {}
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
        $crate::console::print("\n");
    }
}