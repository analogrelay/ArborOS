// Disable std library (we're freestanding) and the default main method
#![no_std]
#![cfg_attr(not(test), no_main)]

// Allow dead code because we write helpers before we use them sometimes :)
#![allow(dead_code)]

// Silence warnings in tests
#![cfg_attr(test, allow(unused_macros, unused_imports))]

// Bring in the std library in tests
#[cfg(test)]
extern crate std;

// Dependencies
extern crate bootloader_precompiled;
extern crate volatile;
extern crate spin;

#[macro_use]
extern crate lazy_static;

// Test-only dependencies
#[cfg(test)]
extern crate array_init;

use core::panic::PanicInfo;

#[macro_use]
mod vga;

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga::WRITER.lock().set_fg(vga::Color::LightRed);
    vga::WRITER.lock().set_bg(vga::Color::Black);
    println!("{}", info);
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    panic!("Kernel not yet implemented");
}