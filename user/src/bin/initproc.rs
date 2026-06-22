#![no_std]
#![no_main]
use user_lib::{fork, wait, exec, heap_init, exit};

#[no_mangle]
fn main() -> i32 {
    heap_init();
    loop {
        let pid = fork();
        if pid == 0 {
            // Child process: launch user shell
            exec("user_shell");
            exit(-1);
        } else {
            // Parent initproc: recycle all zombie children infinitely
            let mut status = 0;
            let child_pid = wait(&mut status);
            if child_pid > 0 {
                // Print recycle log for report demonstration
            }
        }
    }
}