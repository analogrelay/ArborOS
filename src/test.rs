use core::panic::PanicInfo;

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

use crate::{qemu, serial_println};

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    qemu::exit(qemu::ExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    qemu::exit(qemu::ExitCode::Failure);
}

#[cfg(test)]
entry_point!(test_kernel_main);

#[cfg(test)]
pub fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    use crate::cpu;

    crate::init();
    crate::test_main();
    cpu::halt();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
