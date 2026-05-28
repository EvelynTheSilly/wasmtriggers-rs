use std::alloc::{Layout, alloc as std_alloc, dealloc as std_dealloc};
use std::ptr;

/// # Safety
/// see `std::alloc::alloc`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn alloc(size: usize) -> *mut u8 {
    if size == 0 {
        return ptr::null_mut();
    }
    let layout = Layout::from_size_align(size, 1).expect("invalid layout");
    unsafe { std_alloc(layout) }
}

/// # Safety
/// see `std::alloc::dealloc`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dealloc(ptr: *mut u8, size: usize) {
    if ptr.is_null() {
        return;
    }
    let layout = Layout::from_size_align(size, 1).expect("invalid layout");
    unsafe { std_dealloc(ptr, layout) }
}
