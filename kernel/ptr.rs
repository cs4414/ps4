use core::ptr::offset;

/// Calculate the offset from a mut pointer. The count *must* be in bounds or
/// otherwise the loads of this address are undefined.
/// The `count` argument is in units of T; e.g. a `count` of 3
/// represents a pointer offset of `3 * sizeof::<T>()` bytes.
#[inline]
pub unsafe fn mut_offset<T>(ptr: *mut T, count: int) -> *mut T {
    offset(ptr as *T, count) as *mut T
}
