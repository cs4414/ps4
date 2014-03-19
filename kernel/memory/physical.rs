use core::fail::abort;

use kernel;
use kernel::memory;
use kernel::memory::Allocator;

pub static mut frames: memory::Alloc = memory::Alloc {
    base: 0x200_000 as *mut u8,
    el_size: 12,
    parent: memory::BuddyAlloc {
        order: 13,
        tree: memory::Bitv { storage: 0 as memory::BitvStorage }
    }
};

pub fn init() {
    unsafe {
        frames.parent.tree.storage = kernel::zero_alloc(0x1000) as memory::BitvStorage;
    }
}

pub unsafe fn alloc_frames(count: uint) -> *mut u8 {
    match frames.alloc(count) {
        (_, 0) => abort(),
        (ptr, _) => ptr
    }
}

pub unsafe fn zero_alloc_frames(count: uint) -> *mut u8 {
    match frames.zero_alloc(count) {
        (_, 0) => abort(),
        (ptr, _) => ptr
    }
}
