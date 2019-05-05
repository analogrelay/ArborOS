// Disable std library (we're freestanding) and the default main method
#![no_std]
#![no_main]

// Allow dead code because we write helpers before we use them sometimes :)
#![allow(dead_code)]

// Enable custom test runner
#![feature(custom_test_frameworks)]
#![test_runner(arbor_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate arbor_os;

use core::panic::PanicInfo;
use arbor_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world!");

    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic in the real OS.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    arbor_os::vga::WRITER.lock().set_fg(arbor_os::vga::Color::LightRed);
    arbor_os::vga::WRITER.lock().set_bg(arbor_os::vga::Color::Black);
    println!("{}", info);
    loop {}
}

/// This function is called on panic in tests.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    arbor_os::test_panic_handler(info)
}