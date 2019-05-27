//! Boot entry-points which receive data from boot services and call the kernel entry point.

mod services

#[cfg(feature = "boot_bios")]
mod bios;

pub use services::BootServices;