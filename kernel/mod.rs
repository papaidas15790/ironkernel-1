use core::option::{Option, Some, None};

use platform::{cpu, io, drivers};
use cpu::interrupt;

pub mod util;
pub mod mm;
pub mod sgash;
pub mod heap;

pub static mut int_table: Option<interrupt::Table> = None;

#[lang="start"]
#[no_mangle]
pub fn main() {
    heap::init();
    mm::physical::init();

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
