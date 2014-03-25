/* io::mod.rs */

use core::mem::volatile_store;
use kernel::sgash;

mod font;

/* http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0225d/BBABEGGE.html */
pub static UART0: *mut u32 = 0x101f1000 as *mut u32;

pub static UART0_IMSC: *mut u32 = (0x101f1000 + 0x038) as *mut u32;
#[allow(dead_code)]
pub static VIC_INTENABLE: *mut u32 = (0x10140000 + 0x010) as *mut u32;

// These store the current position of the cursor
pub static mut CURSOR_X: u32 = 0;
pub static mut CURSOR_Y: u32 = 0;
// These are set by the bitmaps in ./font.rs
pub static CURSOR_HEIGHT: u32 = 16;
pub static CURSOR_WIDTH: u32 = 8;
// Colors have their own setters
pub static mut CURSOR_COLOR: u32 = 0x00000000;
pub static mut FG_COLOR: u32 = 0x00000000;
pub static mut BG_COLOR: u32 = 0x00000000;
pub static mut CURSOR_BUFFER: [u32, ..8*16] = [0x00FF0000, ..8*16];
pub static mut SAVE_X: u32 = 0;
pub static mut SAVE_Y: u32 = 0;
pub static START_ADDR: u32 = 1024*1024;
pub static mut SCREEN_WIDTH: u32 = 0;
pub static mut SCREEN_HEIGHT: u32 = 0;

pub unsafe fn init(width: u32, height: u32)
{
    SCREEN_WIDTH = width;
    SCREEN_HEIGHT= height;
    sgash::init();
    
    /* For the following magic values, see 
     * http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0225d/CACHEDGD.html
     */

    // 800x600
    // See http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0225d/CACCCFBF.html
    if (SCREEN_WIDTH == 800 && SCREEN_HEIGHT == 600)
    {
    	ws(0x10000010, 0x2CAC);
    	ws(0x10120000, 0x1313A4C4);
    	ws(0x10120004, 0x0505F657);
    	ws(0x10120008, 0x071F1800);

	/* See http://forum.osdev.org/viewtopic.php?p=195000 */
	ws(0x10120010, START_ADDR);
	
	/* See http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0161e/I911024.html */
	ws(0x10120018, 0x82B);
    }

    // 640x480
    // See http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0225d/CACCCFBF.html
    else if (SCREEN_WIDTH == 640 && SCREEN_HEIGHT == 480)
    {
	ws(0x10000010, 0x2C77);
	ws(0x10120000, 0x3F1F3F9C);
	ws(0x10120004, 0x090B61DF);
	ws(0x10120008, 0x067F1800);

	/* See http://forum.osdev.org/viewtopic.php?p=195000 */
	ws(0x10120010, START_ADDR);

	/* See http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0161e/I911024.html */
	ws(0x10120018, 0x82B);

    }
    set_bg(0x222C38);
    set_fg(0xFAFCFF);
    set_cursor_color(0xFAFCFF);
    fill_bg();	
    draw_cursor();
}

pub unsafe fn write_char(c: char, address: *mut u32) {
    volatile_store(address, c as u32);
}

pub unsafe fn scrollup()
{
    let mut i = CURSOR_HEIGHT*SCREEN_WIDTH;
    while i < (SCREEN_WIDTH*SCREEN_HEIGHT)
    {
	*((START_ADDR + ((i-16*SCREEN_WIDTH)*4)) as *mut u32) = *((START_ADDR+(i*4)) as *u32); 
	i += 1;
    }
    i = 4*(SCREEN_WIDTH*SCREEN_HEIGHT - CURSOR_HEIGHT*SCREEN_WIDTH);
    while i < 4*SCREEN_WIDTH*SCREEN_HEIGHT
    {
	*((START_ADDR + (i as u32)) as *mut u32) = BG_COLOR;
	i += 4;
    }
    CURSOR_X = 0x0u32;
    CURSOR_Y -= CURSOR_HEIGHT;
}
pub unsafe fn draw_char(c: char)
{
    while CURSOR_X+(SCREEN_WIDTH*CURSOR_Y) >= SCREEN_WIDTH*SCREEN_HEIGHT
    {
	scrollup();
    }
    let font_offset = (c as u8) - 0x20;
    let map = font::bitmaps[font_offset];

    let mut i = -1;
    let mut j = 0;
    let mut addr = START_ADDR + 4*(CURSOR_X + CURSOR_WIDTH + 1 + SCREEN_WIDTH*CURSOR_Y);
    while j < CURSOR_HEIGHT
    {
	while i < CURSOR_WIDTH
	{
	    //let addr = START_ADDR + 4*(CURSOR_X + CURSOR_WIDTH - i + SCREEN_WIDTH*(CURSOR_Y + j));
	    //let addr = START_ADDR + 4*(CURSOR_X + CURSOR_WIDTH + SCREEN_WIDTH*CURSOR_Y) - 4*i + 4*SCREEN_WIDTH*j
	    if ((map[j] >> 4*i) & 1) == 1
	    {
		*(addr as *mut u32) = FG_COLOR;
	    }
	    else
	    {
		*(addr as *mut u32) = BG_COLOR;
	    }
	    
	    addr-= 4;
	    i += 1;
	}
	addr += 4*(i+SCREEN_WIDTH);
	i = 0;
	j += 1;
    }
}


pub unsafe fn backup()
{
    while CURSOR_X+(SCREEN_WIDTH*CURSOR_Y) >= SCREEN_WIDTH*SCREEN_HEIGHT
    {
	scrollup();
    }
    let mut i = 0;
    let mut j = 0;
    while j < CURSOR_HEIGHT
    {
	while i < CURSOR_WIDTH
	{
	    let addr = START_ADDR + 4*(CURSOR_X + i + SCREEN_WIDTH*(CURSOR_Y + j));
	    CURSOR_BUFFER[i + j*8] = *(addr as *mut u32);
	    i += 1;
	}
	i = 0;
	j += 1;
    }
    SAVE_X = CURSOR_X;
    SAVE_Y = CURSOR_Y;
}

pub unsafe fn restore()
{
    let mut i = 0;
    let mut j = 0;
    while j < CURSOR_HEIGHT
    {
	while i < CURSOR_WIDTH
	{
	    let addr = START_ADDR + 4*(SAVE_X + i + SCREEN_WIDTH*(SAVE_Y + j));
	    *(addr as *mut u32) = CURSOR_BUFFER[i + j*8];
	    i += 1;
	}
	i = 0;
	j += 1;
    }
}

pub unsafe fn draw_cursor()
{

    let mut i = 0;
    let mut j = 0;

    while j < CURSOR_HEIGHT
    {
	while i < CURSOR_WIDTH
	{
	    let addr = START_ADDR + 4*(CURSOR_X + i + SCREEN_WIDTH*(CURSOR_Y + j));
	    *(addr as *mut u32) = CURSOR_COLOR;
	    i += 1;
	}
	i = 0;
	j += 1;
    }

}

pub unsafe fn paint(color: u32)
{
    let mut i = 0; 
    while i < SCREEN_WIDTH*SCREEN_HEIGHT
    {
	*((START_ADDR as u32 + i*4) as *mut u32) = color;
	i+=1;
    }
}

pub unsafe fn fill_bg()
{
    paint(BG_COLOR);
}

#[allow(dead_code)]
pub unsafe fn read(addr: u32)	->	u32
{
    *(addr as *mut u32)
}

pub unsafe fn ws(addr: u32, value: u32)
{
    *(addr as *mut u32) = *(addr as *mut u32) | value;
}

#[allow(dead_code)]
pub unsafe fn wh(addr: u32, value: u32)
{
    *(addr as *mut u32) = value;
}

pub unsafe fn set_fg(color: u32)
{
    FG_COLOR = color;
}

pub unsafe fn set_bg(color: u32)
{
    BG_COLOR = color;
}

pub unsafe fn set_cursor_color(color: u32)
{
    CURSOR_COLOR = color;
}
