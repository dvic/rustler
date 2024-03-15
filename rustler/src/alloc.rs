use std::alloc::{GlobalAlloc, Layout};

/// Allocator implementation that forwards all allocation calls to Erlang's allocator. Allows the
/// memory usage to be tracked by the BEAM.
pub struct EnifAllocator;

unsafe impl GlobalAlloc for EnifAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // TODO: Check the requested alignment.
        rustler_sys::enif_alloc(layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        rustler_sys::enif_free(ptr as *mut rustler_sys::c_void);
    }
}
