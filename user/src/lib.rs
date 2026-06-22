#![no_std]
#![no_main]
#![allow(unused)]

// Import raw syscall functions
mod syscall;
pub use syscall::{sys_exit, sys_yield, sys_get_time, sys_read, sys_getpid, sys_fork, sys_exec, sys_waitpid};

// Buddy system heap allocator for dynamic String
extern crate buddy_system_allocator;
use buddy_system_allocator::Heap;
use core::alloc::{GlobalAlloc, Layout};

// 16KB user heap space
const USER_HEAP_SIZE: usize = 16384;
static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];
static mut HEAP: Heap<32> = Heap::new();

// Global heap allocator
struct UserHeapAllocator;
unsafe impl GlobalAlloc for UserHeapAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        HEAP.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        HEAP.dealloc(ptr, layout);
    }
}
#[global_allocator]
static ALLOCATOR: UserHeapAllocator = UserHeapAllocator;

// Heap initialization
pub fn heap_init() {
    unsafe {
        HEAP.init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    }
}

// High-level wrapped user APIs
pub fn exit(code: i32) -> ! {
    sys_exit(code);
    unreachable!()
}

pub fn yield_cpu() {
    sys_yield();
}

pub fn get_time() -> usize {
    sys_get_time() as usize
}

pub fn read(fd: usize, buf: &mut [u8], len: usize) -> isize {
    sys_read(fd, buf, len)
}

pub fn getpid() -> isize {
    sys_getpid()
}

pub fn fork() -> isize {
    sys_fork()
}

pub fn exec(path: &str) -> isize {
    sys_exec(path)
}

pub fn waitpid(pid: isize, status: &mut i32) -> isize {
    sys_waitpid(pid, status)
}

// Blocking wait for any child process
pub fn wait(status: &mut i32) -> isize {
    waitpid(-1, status)
}

// Simple sleep function based on timer yield
pub fn sleep(seconds: usize) {
    let start = get_time();
    while get_time() < start + seconds * 1000 {
        yield_cpu();
    }
}