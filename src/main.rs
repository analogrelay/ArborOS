// Disable std library (we're freestanding) and the default main method
#![no_std]
#![no_main]
// Allow dead code because we write helpers before we use them sometimes :)
#![allow(dead_code)]
// Enable custom test runner
#![feature(custom_test_frameworks)]
#![test_runner(arbor_os::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

use arbor_os::{
    arch::{self, Cpu}, 
    devices::vga,
    println
};

#[cfg(test)]
use arbor_os::test;

entry_point!(kernel_main);

pub fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    println!("Hello world!");

    // Initialize the core OS
    arbor_os::init();

    // Launch test (when built as a test)
    #[cfg(test)]
    test_main();

    arch::CPU.halt();
}

/// This function is called on panic in the real OS.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga::WRITER
        .lock()
        .set_fg(vga::Color::LightRed);
    vga::WRITER
        .lock()
        .set_bg(vga::Color::Black);
    println!("{}", info);
    arch::CPU.halt();
}

/// This function is called on panic in tests.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test::test_panic_handler(info)
}
