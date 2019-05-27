// All architecture-specific code goes here.
// We automatically pull in the right submodule based on the current architecture

// Common stuff for all architectures
pub struct ArchitectureInfo {
    name: &'static str
}

pub trait Cpu {
    fn halt(&self) -> !;
}

// Import arch-specific modules
#[cfg(target_arch="x86_64")]
mod x86_64;
#[cfg(target_arch="x86_64")]
pub use self::x86_64::*;