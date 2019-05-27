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

use arbor_os::{cpu, memory, println};

entry_point!(kernel_main);

pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello world!");

    // Initialize the core OS
    arbor_os::init();

    use x86_64::structures::paging::Page;
    use x86_64::VirtAddr;

    let mut mapper = unsafe { memory::init(boot_info.physical_memory_offset) };
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // map a previously unmapped page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    // Launch test (when built as a test)
    #[cfg(test)]
    test_main();

    cpu::halt()
}

/// This function is called on panic in the real OS.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    arbor_os::vga::WRITER
        .lock()
        .set_fg(arbor_os::vga::Color::LightRed);
    arbor_os::vga::WRITER
        .lock()
        .set_bg(arbor_os::vga::Color::Black);
    println!("{}", info);
    cpu::halt()
}

/// This function is called on panic in tests.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    arbor_os::test::test_panic_handler(info)
}
