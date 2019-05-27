#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod qemu;
pub mod serial;
pub mod vga;
pub mod interrupts;
pub mod test;
pub mod gdt;
pub mod cpu;
pub mod memory;

pub fn init() {
    gdt::init();
    interrupts::init_idt();

    // Initialize the PICs
    unsafe {
        interrupts::PICS.lock().initialize();
    }

    // Enable interrupts! Now we're a Real Boy^B^B^B OS
    x86_64::instructions::interrupts::enable();
}