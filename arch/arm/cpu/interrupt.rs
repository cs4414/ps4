use core::mem::{volatile_store, transmute};
use core::ptr::offset;

use platform::io;

static VIC_INT_ENABLE: *mut u32 = (0x10140000 + 0x010) as *mut u32;
static UART0_IRQ: u8 = 12;
static VT: *u32 = 0 as *u32;

#[repr(u8)]
pub enum Int {
    RESET = 0,
    UNDEF,
    SWI, // software interrupt
    PREFETCH_ABORT,
    DATA_ABORT,
    IRQ = 6,
    FIQ
}

fn set_word(vector: u8, instruction: u32) {
    unsafe {
        volatile_store(offset(VT, vector as int) as *mut u32, instruction);
    }
}

fn branch(rel: u32) -> u32 {
    // b isr ; branch instruction [1]
    /* 
	 * See
	 * http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0225d/I1042232.html
	 * and pczarn's comment at 
	 * https://github.com/wbthomason/ironkernel/commit/4b199b502b2fc5d42b7f1571b52dd1b0c657e77b#arch-arm-cpu-interrupt-rs-P6
	 */
    0xea000000 | (((rel - 8) >> 2) & 0xffffff)
}

pub struct Table;

impl Table {
    pub fn new() -> Table {
        Table
    }

    pub fn enable(&self, which: Int, isr: unsafe fn()) {
        // Installing exception handlers into the vectors directly [1]
        let vector: u8 = unsafe { transmute(which) };
        set_word(vector, branch(isr as u32 - (vector as u32 * 4)));
    }

    pub fn load(&self) {
        let mut i = 0;
        while i < 10 {
            // make every handler loop indefinitely
            set_word(i, branch(0));
            i += 1;
        }

        self.enable(RESET, unsafe { transmute(start) });
        // breakpoints use an UND opcode to trigger UNDEF. [7]
        self.enable(UNDEF, debug);

        unsafe {
            // Enable IRQs [5]
            asm!("mov r2, sp
              mrs r0, cpsr      // get Program Status Register
              bic r1, r0, #0x1F // go in IRQ mode
              orr r1, r1, #0x12
              msr cpsr, r1
              mov sp, 0x19000   // set IRQ stack
              bic r0, r0, #0x80 // Enable IRQs
              msr cpsr, r0      // go back in Supervisor mode
              mov sp, r2"
            ::: "r0", "r1", "r2", "cpsr");

            // enable UART0 IRQ [4]
            *VIC_INT_ENABLE = 1 << UART0_IRQ;
            // enable RXIM interrupt
            /*
             * See
             * http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0183f/I54603.html
             */
            *io::UART0_IMSC = 1 << 4;
        }
    }
}

extern {
    fn start();
}

#[no_mangle]
pub unsafe fn debug() {
    asm!("movs pc, lr")
}

/*
#[lang="fail_"]
#[fixed_stack_segment]
pub fn fail(expr: *u8, file: *u8, line: uint) -> ! {
    unsafe { zero::abort(); }
}

#[lang="fail_bounds_check"]
#[fixed_stack_segment]
pub fn fail_bounds_check(file: *u8, line: uint, index: uint, len: uint) {
    unsafe { zero::abort(); }
}
*/
