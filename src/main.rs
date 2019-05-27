// Disable std library (we're freestanding) and the default main method
#![no_std]
#![no_main]

// Allow dead code because we write helpers before we use them sometimes :)
#![allow(dead_code)]

// Enable custom test runner
#![feature(custom_test_frameworks)]
#![test_runner(arbor_os::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};

use arbor_os::{cpu, memory, println};

entry_point!(kernel_main);

pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello world!");

    // Initialize the core OS
    arbor_os::init();

    use x86_64::VirtAddr;
    use x86_64::structures::paging::MapperAllSizes;

    // new: initialize a mapper
    let mapper = unsafe { memory::init(boot_info.physical_memory_offset) };

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x20010a,
        // some stack page
        0x57ac_001f_fe48,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        // new: use the `mapper.translate_addr` method
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    // Launch test (when built as a test)
    #[cfg(test)]
    test_main();

    cpu::halt()
}

/// This function is called on panic in the real OS.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    arbor_os::vga::WRITER.lock().set_fg(arbor_os::vga::Color::LightRed);
    arbor_os::vga::WRITER.lock().set_bg(arbor_os::vga::Color::Black);
    println!("{}", info);
    cpu::halt()
}

/// This function is called on panic in tests.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    arbor_os::test::test_panic_handler(info)
}