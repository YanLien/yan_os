use super::{align_up, Locked};
use alloc::alloc::{GlobalAlloc, Layout};
use pc_keyboard::layouts;
use core::ptr;

pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}

impl BumpAllocator {
    pub const fn new() -> Self {
        BumpAllocator { 
            heap_start: 0, 
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    }
    // heap_start和heap_end字段跟踪堆内存区域的下限和上限。
    pub fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = heap_start; 
        // next字段的目的是始终指向堆的第一个未使用字节，即下一个分配的起始地址。
    }
}

// 所有堆分配器都需要实现GlobalAlloc trait，该trait定义了alloc和dealloc方法。
unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut bump = self.lock();

        let alloc_start = align_up(bump.next, layout.align());  //此布局的内存块的最小字节对齐。
        // 将next地址四舍五入为Layout参数指定的对齐方式
        let alloc_end = alloc_start + layout.size();

        if alloc_end > bump.heap_end {
            ptr::null_mut()
        } else {
            bump.next = alloc_end;
            bump.allocations += 1;
            alloc_start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        let mut bump = self.lock();

        bump.allocations -= 1;
        if bump.allocations == 0 {
            bump.next = bump.heap_start;
        }
    }
}
