use crate::timer::get_time_ms;

pub fn sys_exit(xstate: i32) -> ! {
    println!("Application exited with code {}", xstate);
    crate::task::exit_current_and_run_next();
    loop {}
}

pub fn sys_yield() -> isize {
    crate::task::suspend_current_and_run_next();
    0
}

pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}