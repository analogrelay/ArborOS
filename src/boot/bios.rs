//! Bios boot entry-point

use bootloader::{entry_point, BootInfo};

entry_point!(bios_entry);

/// BIOS entry point. This is what the BIOS-based bootloader calls first!
pub fn bios_entry(boot_info: &'static BootInfo) -> ! {
    kernel_main(boot_info);
}