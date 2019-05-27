use crate::arch;

pub struct X64Cpu;

impl arch::Cpu for X64Cpu {
    fn halt(&self) -> ! {
        loop {
            x86_64::instructions::hlt();
        }
    }
}
