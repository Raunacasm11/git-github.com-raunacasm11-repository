// Page size (4KB)
pub const PAGE_SIZE: usize = 0x1000;
pub const PAGE_SIZE_BITS: usize = 12;

// Kernel heap size (3MB)
pub const KERNEL_HEAP_SIZE: usize = 0x30_0000;

// Physical memory end (QEMU RISC-V)
pub const MEMORY_END: usize = 0x80800000;

// Trampoline & Trap Context address (top of virtual address space)
pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;

// Stack sizes
pub const USER_STACK_SIZE: usize = 4096 * 2;
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;

// App limits
pub const MAX_APP_NUM: usize = 4;
pub const APP_BASE_ADDRESS: usize = 0x0;
pub const APP_SIZE_LIMIT: usize = 0x20000;

/// Calculate kernel stack bottom/top for each app
pub fn kernel_stack_position(app_id: usize) -> (usize, usize) {
    let top = MEMORY_END - app_id * KERNEL_STACK_SIZE;
    let bottom = top - KERNEL_STACK_SIZE;
    (bottom, top)
}
