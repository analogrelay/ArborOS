use core::fmt;

use spin::Mutex;
use lazy_static::lazy_static;

use crate::arch;

use color::ColorCode;
use buffer::Buffer;

pub use color::Color;
pub use writer::Writer;

mod color;
mod writer;
mod buffer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const VGA_FRAMEBUFFER_ADDRESS: usize = 0xb8000;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new(
        Color::Yellow,
        Color::Black,
        unsafe { Buffer::from_address(VGA_FRAMEBUFFER_ADDRESS) }
    ));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::devices::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n"), $($arg)*));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    // Disable interrupts while we run
    arch::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}
