use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::RefCell;
use std::thread_local;

#[global_allocator]
pub static GLOBAL: CountingAllocator = CountingAllocator::new();

pub struct CountingAllocator;

impl CountingAllocator {
    pub const fn new() -> Self {
        CountingAllocator
    }

    pub fn get_local_allocated() -> usize {
        LOCAL_ALLOCATED.with(|allocated| *allocated.borrow())
    }

    pub fn get_local_deallocated() -> usize {
        LOCAL_DEALLOCATED.with(|deallocated| *deallocated.borrow())
    }

    pub fn reset_local() {
        LOCAL_ALLOCATED.with(|allocated| *allocated.borrow_mut() = 0);
        LOCAL_DEALLOCATED.with(|deallocated| *deallocated.borrow_mut() = 0);
    }
}

thread_local! {
    static LOCAL_ALLOCATED: RefCell<usize> = RefCell::new(0);
    static LOCAL_DEALLOCATED: RefCell<usize> = RefCell::new(0);
}

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = System.alloc(layout);
        if !ptr.is_null() {
            LOCAL_ALLOCATED.with(|allocated| *allocated.borrow_mut() += layout.size());
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        LOCAL_DEALLOCATED.with(|deallocated| *deallocated.borrow_mut() += layout.size());
    }
}

pub struct ScopedAllocCounter {
    bf_local_allocated: usize,
    bf_local_deallocated: usize,
}

impl ScopedAllocCounter {
    pub fn new() -> Self {
        ScopedAllocCounter {
            bf_local_allocated: CountingAllocator::get_local_allocated(),
            bf_local_deallocated: CountingAllocator::get_local_deallocated(),
        }
    }
}

impl Drop for ScopedAllocCounter {
    fn drop(&mut self) {
        let local_allocated = CountingAllocator::get_local_allocated() - self.bf_local_allocated;
        let local_deallocated = CountingAllocator::get_local_deallocated() - self.bf_local_deallocated;

        // This is where you can log or report the allocations
        // println!(
        //     "Local Allocations: +{}B -{}B",
        //     local_allocated, local_deallocated
        // );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alloc() {
        let _alloc_counter = ScopedAllocCounter::new();
        {
            let mut _v = vec![0u8; 1024];
            println!("{:?}", _v[1023] + 12);
        }
        assert!(CountingAllocator::get_local_allocated() >= 1024);
        assert!(CountingAllocator::get_local_deallocated() >= 1024);
        assert!(CountingAllocator::get_local_allocated() < 1100); // arbitraily, it could be just few bytes more.
    }
}
