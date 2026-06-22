#![no_std]
#![allow(unused)]

// Existing syscall constants from your file
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;

// New syscall IDs added for Process Management Experiment
const SYSCALL_READ: usize = 63;
const SYSCALL_GETPID: usize = 172;
const SYSCALL_FORK: usize = 220;
const SYSCALL_EXEC: usize = 221;
const SYSCALL_WAITPID: usize = 222;

// Low-level ecall syscall entry point
fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret;
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("a0") args[0] => ret,
            in("a1") args[1],
            in("a2") args[2],
            in("a7") id,
        );
    }
    ret
}

// Existing wrapper functions
pub fn sys_exit(exit_code: i32) -> isize {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
}

pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0, 0, 0])
}

pub fn sys_get_time() -> isize {
    syscall(SYSCALL_GET_TIME, [0, 0, 0])
}

// ---------------------- New Experiment Syscall Wrappers ----------------------
/// Read single byte from file descriptor fd into buffer
pub fn sys_read(fd: usize, buffer: &mut [u8], len: usize) -> isize {
    syscall(SYSCALL_READ, [fd, buffer.as_mut_ptr() as usize, len])
}

/// Get current process PID
pub fn sys_getpid() -> isize {
    syscall(SYSCALL_GETPID, [0, 0, 0])
}

/// Fork current process, returns 0 for child, child PID for parent
pub fn sys_fork() -> isize {
    syscall(SYSCALL_FORK, [0, 0, 0])
}

/// Replace current process image with target ELF program
pub fn sys_exec(path: &str) -> isize {
    syscall(SYSCALL_EXEC, [path.as_ptr() as usize, 0, 0])
}

/// Wait for child process with target pid, store exit code to status ptr
pub fn sys_waitpid(pid: isize, status: &mut i32) -> isize {
    syscall(SYSCALL_WAITPID, [pid as usize, status as *mut i32 as usize, 0])
}