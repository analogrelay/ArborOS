pub fn halt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}