use core::fmt;

use crate::vga::{Buffer, Color, ColorCode, ScreenChar, BUFFER_HEIGHT, BUFFER_WIDTH};

pub struct Writer {
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,

    // Visible in crate for testing.
    pub(crate) buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new(fg: Color, bg: Color, buffer: &'static mut Buffer) -> Writer {
        Writer {
            column_position: 0,
            row_position: 0,
            color_code: ColorCode::new(fg, bg),
            buffer: buffer,
        }
    }

    pub fn fg(&self) -> Color {
        self.color_code.fg()
    }

    pub fn bg(&self) -> Color {
        self.color_code.bg()
    }

    pub fn set_fg(&mut self, fg: Color) {
        self.color_code.set_fg(fg);
    }

    pub fn set_bg(&mut self, bg: Color) {
        self.color_code.set_bg(bg);
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer[(self.row_position, col)].write(ScreenChar {
                    ascii_character: byte,
                    color_code: color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn clear(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
        self.row_position = 0;
        self.column_position = 0;
    }

    fn new_line(&mut self) {
        self.column_position = 0;
        self.row_position += 1;
        if self.row_position == BUFFER_HEIGHT {
            // Scroll the window
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let character = self.buffer[(row, col)].read();
                    self.buffer[(row - 1, col)].write(character);
                }
            }

            self.clear_row(BUFFER_HEIGHT - 1);
            self.row_position = BUFFER_HEIGHT - 1;
        }
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer[(row, col)].write(blank);
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20...0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}