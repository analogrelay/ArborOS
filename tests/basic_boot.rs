#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(arbor_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate arbor_os;

use core::panic::PanicInfo;

use arbor_os::{serial_print, serial_println, println};

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

fn test_runner(tests: &[&dyn Fn()]) {
    unimplemented!();
}

#[test_case]
fn test_println() {
    serial_print!("test_println... ");
    println!("test_println output");
    serial_println!("[ok]");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    arbor_os::test_panic_handler(info)
}