#[allow(unused_imports)];
#[allow(dead_code)];

use core::mem::size_of;
use core::ptr::set_memory;
use core::option::Some;

use kernel::heap;
use kernel::memory::physical;
use kernel;

static CACHE:  u32 = 1 << 3;
static BUFFER: u32 = 1 << 2;

pub static SECTION: u32 = 0b10010;
pub static RW:      u32 = 1 << 10;
pub static USER:    u32 = 1 << 11;

#[packed]
struct Descriptor(u32);

#[packed]
struct PageTableCoarse {
    pages: [Descriptor, ..256]
}

#[packed]
pub struct PageDirectory {
    tables: [Descriptor, ..4096]
}

pub unsafe fn init() {
    let dir = physical::zero_alloc_frames(4) as *mut PageDirectory;

    (*dir).tables[0] = Descriptor::section(0, RW);
    (*dir).enable();
}

#[allow(unused_variable)]
pub unsafe fn map(page_ptr: *mut u8, flags: u32) {
    // TODO
}

impl Descriptor {
    fn section(base: u32, flags: u32) -> Descriptor {
        // make a section descriptor
        //                /permissions
        Descriptor(base | flags | SECTION)
    }
}

impl PageDirectory {
    pub unsafe fn enable(&self) {
        asm!("mov ip, 0
              mcr p15, 0, ip, c7, c5, 0     // invalidate I cache
              mcr p15, 0, ip, c7, c10, 4    // drain WB
              mcr p15, 0, r0, c2, c0, 0     // load page table pointer
              mcr p15, 0, ip, c8, c7, 0     // invalidate I & D TLBs"
            :: "{r0}"(self) : "ip")
    }
}
