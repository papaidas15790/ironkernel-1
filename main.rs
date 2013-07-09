#[allow(ctypes)];
#[no_std];
#[no_core];

mod zero;

#[inline]
pub fn size_of_val<T>(_val: *mut T) -> uint {
    unsafe { zero::size_of::<T>() }
}

#[packed]
struct idt_reg {
    size: u16,
    addr: *mut [idt_entry, ..256],
}

static Present: u8 = 1 << 7;
static PM32Bit: u8 = 1 << 3;

#[packed]
struct idt_entry {
    addr_lo: u16,
    sel: u16,
    zero: u8,
    flags: u8,
    addr_hi: u16
}

fn idt_entry(proc: u32, sel: u16, flags: u8) -> idt_entry {
    idt_entry {
        addr_lo: (proc & 0xffff) as u16,
        sel: sel,
        zero: 0,
        flags: flags | 0b110,
        addr_hi: (proc >> 16) as u16
    }
}

enum Color {
    Black       = 0,
    Blue        = 1,
    Green       = 2,
    Cyan        = 3,
    Red         = 4,
    Pink        = 5,
    Brown       = 6,
    LightGray   = 7,
    DarkGray    = 8,
    LightBlue   = 9,
    LightGreen  = 10,
    LightCyan   = 11,
    LightRed    = 12,
    LightPink   = 13,
    Yellow      = 14,
    White       = 15,
}

fn range(lo: uint, hi: uint, it: &fn(uint)) {
    let mut iter = lo;
    while iter < hi {
        it(iter);
        iter += 1;
    }
}

unsafe fn clear_screen(background: Color) {
    range(0, 80*25, |i| {
        *((0xb8000 + i * 2) as *mut u16) = (background as u16) << 12;
    });
}

#[no_mangle]
pub unsafe fn main() {
    clear_screen(LightRed);
}
