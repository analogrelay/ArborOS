use core::ops;

#[cfg(test)]
use std::string::String;

use volatile::Volatile;

use vga::{ScreenChar, BUFFER_HEIGHT, BUFFER_WIDTH};

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

    #[cfg(test)]
    pub fn new(fill_char: ScreenChar) -> Buffer {
        use array_init::array_init;
        Buffer {
            chars: array_init(|_| array_init(|_| Volatile::new(fill_char))),
        }
    }

    #[cfg(test)]
    pub fn get_row(&self, row: usize) -> String {
        use std::string::String;
        use std::vec::Vec;

        let mut v: Vec<u8> = Vec::with_capacity(BUFFER_WIDTH);
        for chr in self[row].iter() {
            v.push(chr.read().ascii_character);
        }

        String::from_utf8(v).unwrap()
    }

    #[cfg(test)]
    pub fn iter_rows(&self) -> RowIter {
        RowIter {
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
pub struct RowIter<'a> {
    buffer: &'a Buffer,
    position: usize,
}

#[cfg(test)]
impl<'a> Iterator for RowIter<'a> {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        if self.position < BUFFER_HEIGHT {
            let idx = self.position;
            self.position += 1;
            Some(self.buffer.get_row(idx))
        } else {
            None
        }
    }
}
