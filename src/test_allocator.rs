use std::{
    alloc::{GlobalAlloc, Layout},
    cell::Cell,
};

pub(crate) struct TestAllocator<A>(pub A);

thread_local! {
    static ALLOCATIONS: Cell<Option<usize>> = const { Cell::new(None) };
}

fn record_allocation() {
    let _ = ALLOCATIONS.try_with(|count| {
        if let Some(value) = count.get() {
            count.set(Some(value + 1));
        }
    });
}

unsafe impl<A: GlobalAlloc> GlobalAlloc for TestAllocator<A> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        record_allocation();
        self.0.alloc(layout)
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        record_allocation();
        self.0.alloc_zeroed(layout)
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        record_allocation();
        self.0.realloc(ptr, layout, size)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.dealloc(ptr, layout)
    }
}

pub(crate) fn count_allocations(operation: impl FnOnce()) -> usize {
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            ALLOCATIONS.with(|count| count.set(None));
        }
    }

    ALLOCATIONS.with(|count| count.set(Some(0)));
    let _reset = Reset;
    operation();
    ALLOCATIONS.with(|count| count.get().unwrap())
}
