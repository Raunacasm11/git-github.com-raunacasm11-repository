mod heap_allocator;
mod address;
mod frame_allocator;
mod page_table;

// Export public interfaces
pub use heap_allocator::{init_heap, heap_test};
pub use address::{
    PhysAddr, VirtAddr, PhysPageNum, VirtPageNum,
    VPNRange, StepByOne
};
pub use frame_allocator::{
    FrameTracker, frame_alloc,
    init_frame_allocator, frame_allocator_test
};
pub use page_table::{
    PageTable, PageTableEntry, PTEFlags,
    translated_byte_buffer
};

pub fn init() {
    init_heap();
    heap_test();
    init_frame_allocator();
    frame_allocator_test();
}