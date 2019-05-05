use core::ops;

use volatile::Volatile;

use crate::vga::{ScreenChar, BUFFER_HEIGHT, BUFFER_WIDTH};

pub struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl Buffer {
    pub unsafe fn from_address(addr: usize) -> &'static mut Buffer {
        &mut *(addr as *mut Buffer)
    }

    pub fn iter(&self) -> Iter {
        Iter {
            buffer: self,
            position: 0,
        }
    }
}

impl ops::Index<usize> for Buffer {
    type Output = [Volatile<ScreenChar>];

    fn index<'a>(&'a self, index: usize) -> &'a [Volatile<ScreenChar>] {
        &self.chars[index]
    }
}

impl ops::IndexMut<usize> for Buffer {
    fn index_mut(&mut self, index: usize) -> &mut [Volatile<ScreenChar>] {
        &mut self.chars[index]
    }
}

impl ops::Index<(usize, usize)> for Buffer {
    type Output = Volatile<ScreenChar>;

    fn index<'a>(&'a self, (row, col): (usize, usize)) -> &'a Volatile<ScreenChar> {
        &self.chars[row][col]
    }
}

impl ops::IndexMut<(usize, usize)> for Buffer {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Volatile<ScreenChar> {
        &mut self.chars[row][col]
    }
}

pub struct Iter<'a> {
    buffer: &'a Buffer,
    position: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a [Volatile<ScreenChar>];
    fn next(&mut self) -> Option<&'a [Volatile<ScreenChar>]> {
        if self.position < BUFFER_HEIGHT {
            let idx = self.position;
            self.position += 1;
            Some(&self.buffer.chars[idx])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{serial_print, serial_println, println, vga};

    #[test_case]
    fn test_println_simple() {
        serial_print!("test_println... ");
        println!("test_println_simple output");
        serial_println!("[ok]");
    }

    #[test_case]
    fn test_println_many() {
        serial_print!("test_println_many... ");
        for _ in 0..200 {
            println!("test_println_many output");
        }
        serial_println!("[ok]");
    }

    #[test_case]
    fn test_println_output() {
        serial_print!("test_println_output... ");

        let s = "Some test string that fits on a single line";
        println!("{}", s);
        for (i, c) in s.chars().enumerate() {
            let screen_char = vga::WRITER.lock().buffer.chars[vga::BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }

        serial_println!("[ok]");
    }
}