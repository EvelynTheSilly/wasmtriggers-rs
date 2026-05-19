use std::alloc::{alloc as std_alloc, dealloc as std_dealloc, Layout};
use std::ptr;

#[unsafe(no_mangle)]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    if size == 0 {
        return ptr::null_mut();
    }
    let layout = Layout::from_size_align(size, 1).expect("invalid layout");
    unsafe { std_alloc(layout) }
}

#[unsafe(no_mangle)]
pub extern "C" fn dealloc(ptr: *mut u8, size: usize) {
    if ptr.is_null() {
        return;
    }
    let layout = Layout::from_size_align(size, 1).expect("invalid layout");
    unsafe { std_dealloc(ptr, layout) }
}
