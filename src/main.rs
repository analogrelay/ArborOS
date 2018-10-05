// Disable std library (we're freestanding) and the default main method
#![no_std]
#![cfg_attr(not(test), no_main)]

// Silence warnings in tests
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

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
    println!("{}", info);
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    panic!("Kernel not yet implemented");
}