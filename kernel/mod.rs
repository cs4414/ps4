use core::option::{Option, Some, None};
use core::fail::out_of_memory;

use platform::{cpu, io, drivers};
use cpu::interrupt;

use self::memory::Allocator;

pub mod int;
pub mod ptr;
pub mod memory;
pub mod sgash;

#[cfg(target_word_size = "32")]
pub mod rt;

pub static mut heap: memory::Alloc = memory::Alloc {
    base: 0x110_000 as *mut u8,
    el_size: 0,
    parent: memory::BuddyAlloc {
        order: 17,
        tree: memory::Bitv { storage: 0x100_000 as memory::BitvStorage }
    }
};

pub static mut int_table: Option<interrupt::Table> = None;

#[lang="start"]
#[no_mangle]
pub fn main() {
    memory::BuddyAlloc::new(17, memory::Bitv { storage: 0x100_000 as memory::BitvStorage });
    memory::physical::init();
    let table = interrupt::Table::new();
    unsafe {
        int_table = Some(table);
    }
    cpu::init();

    table.load();
    drivers::init();
    unsafe {
        drivers::keydown = Some(sgash::parsekey);
        io::init(640, 480);
    }
}

#[lang = "exchange_malloc"]
#[inline]
pub unsafe fn malloc_raw(size: uint) -> *mut u8 {
    if size == 0 {
        0 as *mut u8
    }
    else {
        let (ptr, sz) = heap.alloc(size);
        if sz == 0 {
            out_of_memory();
        }
        ptr
    }
}

#[lang = "exchange_free"]
#[inline]
pub unsafe fn free(ptr: *mut u8) {
    heap.free(ptr);
}

#[inline]
pub unsafe fn zero_alloc(size: uint) -> *mut u8 {
    if size == 0 {
        0 as *mut u8
    }
    else {
        let (ptr, sz) = heap.zero_alloc(size);
        if sz == 0 {
            out_of_memory();
        }
        ptr
    }
}

#[inline]
pub unsafe fn realloc_raw(ptr: *mut u8, size: uint) -> *mut u8 {
    if size == 0 {
        free(ptr);
        0 as *mut u8
    } else {
        let (ptr, sz) = heap.realloc(ptr, size);
        if sz == 0 {
            out_of_memory()
        }
        ptr
    }
}
