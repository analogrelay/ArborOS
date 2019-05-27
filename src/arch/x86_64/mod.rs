mod gdt;
mod cpu;
mod interrupts;
mod vmem;

use crate::arch;

pub use interrupts::without_interrupts;

/// Contains information about the current architecture for use at runtime.
pub const ARCHITECTURE_INFO: arch::ArchitectureInfo = arch::ArchitectureInfo {
    name: "x86_64"
};

/// Provides control over the CPU itself.
pub const CPU: cpu::X64Cpu = cpu::X64Cpu;

/// Initializes Architecture-specific logic.
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