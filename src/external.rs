use core::ptr;

extern "C" {
    fn __dlmalloc_alloc(size: usize) -> usize;

    #[cfg(feature = "global")]
    fn __dlmalloc_acquire_global_lock();

    #[cfg(feature = "global")]
    fn __dlmalloc_release_global_lock();

    static __DLMALLOC_PAGE_SIZE: usize;
}

pub unsafe fn alloc(size: usize) -> (*mut u8, usize, u32) {
    let size = size / page_size() * page_size(); // TODO: Is this needed?

    let start = __dlmalloc_alloc(size);
    if start == usize::max_value() {
        return (ptr::null_mut(), 0, 0);
    }
    (start as *mut u8, size, 0)
}

pub unsafe fn remap(_ptr: *mut u8, _oldsize: usize, _newsize: usize, _can_move: bool) -> *mut u8 {
    // TODO: I think this can be implemented near the end?
    ptr::null_mut()
}

pub unsafe fn free_part(_ptr: *mut u8, _oldsize: usize, _newsize: usize) -> bool {
    false
}

pub unsafe fn free(_ptr: *mut u8, _size: usize) -> bool {
    false
}

pub fn can_release_part(_flags: u32) -> bool {
    false
}

#[cfg(feature = "global")]
pub fn acquire_global_lock() {
    __dlmalloc_acquire_global_lock()
}

#[cfg(feature = "global")]
pub fn release_global_lock() {
    __dlmalloc_release_global_lock()
}

pub fn allocates_zeros() -> bool {
    true
}

pub fn page_size() -> usize {
    unsafe { __DLMALLOC_PAGE_SIZE }
}
