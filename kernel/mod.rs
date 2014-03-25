#[allow(unused_imports)];

use core::option::{Option, Some, None};
use core::fail::out_of_memory;

use platform::{cpu, io, drivers};
use cpu::interrupt;

use self::memory::Allocator;

pub mod ptr;
pub mod memory;
pub mod sgash;

#[cfg(target_word_size = "32")]
pub mod rt;
static START_ADDRESS : *mut u8 = 0x100_000 as *mut u8;
static MEMORY_ORDER : uint = 17;

pub static mut heap: memory::Alloc = memory::Alloc {
    base: START_ADDRESS as *mut u8,
    el_size: 0,
    parent: memory::BuddyAlloc {
        order: MEMORY_ORDER,
        tree: memory::Bitv { storage: START_ADDRESS as memory::BitvStorage }
    }
};

pub static mut int_table: Option<interrupt::Table> = None;

#[lang="start"]
#[no_mangle]
pub fn main() {
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
