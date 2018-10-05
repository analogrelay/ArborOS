use core::fmt;

use vga::{Buffer, Color, ColorCode, ScreenChar, BUFFER_HEIGHT, BUFFER_WIDTH};

pub struct Writer {
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
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

#[cfg(test)]
mod test {
    use std::format;
    use std::vec::Vec;

    use super::*;
    use volatile::Volatile;

    #[test]
    fn write_byte_sets_character_in_next_buffer_slot() {
        let mut writer = construct_writer();
        writer.write_byte(b'X');
        writer.write_byte(b'Y');

        for (i, row) in writer.buffer.iter_rows().enumerate() {
            if i == 0 {
                assert_eq!("XY", row.trim());
            } else {
                assert_eq!("", row.trim());
            }
        }
    }

    #[test]
    fn write_macro_can_write_to_vga_buffer() {
        use core::fmt::Write;

        let mut writer = construct_writer();
        writeln!(&mut writer, "a").unwrap();
        writeln!(&mut writer, "b{}", "c").unwrap();

        for (i, row) in writer.buffer.iter_rows().enumerate() {
            if i == 0 {
                assert_eq!("a", row.trim());
            } else if i == 1 {
                assert_eq!("bc", row.trim());
            } else {
                assert_eq!("", row.trim());
            }
        }
    }

    #[test]
    fn clear_sets_buffer_to_all_blanks() {
        use core::fmt::Write;

        let mut writer = construct_writer();

        // Fill the buffer
        for i in 0..BUFFER_HEIGHT {
            writeln!(&mut writer, "Line {}", i);
        }

        // Clear it
        writer.clear();

        // Verify the rows
        for row in writer.buffer.iter_rows() {
            assert_eq!("", row.trim());
        }
    }

    #[test]
    fn buffer_scrolls_when_filled() {
        use core::fmt::Write;

        let mut writer = construct_writer();

        // Fill the buffer
        for i in 0..BUFFER_HEIGHT {
            writeln!(&mut writer, "Line {}", i);
        }

        // Write two more lines
        writeln!(&mut writer, "Extra Line 1");
        writeln!(&mut writer, "Extra Line 2");

        // Verify the rows
        for (i, row) in writer.buffer.iter_rows().enumerate() {
            if i == BUFFER_HEIGHT - 3 {
                assert_eq!("Extra Line 1", row.trim());
            } else if i == BUFFER_HEIGHT - 2 {
                assert_eq!("Extra Line 2", row.trim());
            } else if i == BUFFER_HEIGHT - 1 {
                assert_eq!("", row.trim());
            } else {
                assert_eq!(format!("Line {}", i + 3), row.trim());
            }
        }
    }

    #[test]
    fn clear_resets_cursor_position() {
        use core::fmt::Write;

        let mut writer = construct_writer();

        // Fill the buffer
        for i in 0..BUFFER_HEIGHT {
            writeln!(&mut writer, "Line {}", i);
        }

        // Clear it
        writer.clear();

        // Write a new line
        writeln!(&mut writer, "Line 1");

        // Verify the rows
        for (i, row) in writer.buffer.iter_rows().enumerate() {
            if i == 0 {
                assert_eq!("Line 1", row.trim());
            } else {
                assert_eq!("", row.trim());
            }
        }
    }

    #[test]
    fn write_byte_includes_current_colors() {
        use core::fmt::Write;

        let mut writer = construct_writer();
        writer.set_fg(Color::Yellow);
        writer.set_bg(Color::LightCyan);
        writer.write_byte(b'X');
        writer.set_fg(Color::Red);
        writer.set_bg(Color::Magenta);
        writer.write_byte(b'Y');

        assert_eq!(Color::Yellow, writer.buffer[(0, 0)].read().color_code.fg());
        assert_eq!(
            Color::LightCyan,
            writer.buffer[(0, 0)].read().color_code.bg()
        );
        assert_eq!(Color::Red, writer.buffer[(0, 1)].read().color_code.fg());
        assert_eq!(
            Color::Magenta,
            writer.buffer[(0, 1)].read().color_code.bg()
        );
    }

    #[test]
    fn color_values_scroll() {
        use core::fmt::Write;

        let mut writer = construct_writer();
        writer.set_fg(Color::Yellow);
        writer.set_bg(Color::White);

        // Fill the buffer
        for i in 0..BUFFER_HEIGHT {
            writeln!(&mut writer, "Line {}", i);
        }

        // Change the colors
        writer.set_fg(Color::Red);
        writer.set_bg(Color::Blue);

        // Write another lines
        writeln!(&mut writer, "1");
        writeln!(&mut writer, "2");

        // Check the color of the new lines
        assert_eq!(Color::Red, writer.buffer[BUFFER_HEIGHT - 1][0].read().color_code.fg());
        assert_eq!(Color::Blue, writer.buffer[BUFFER_HEIGHT - 1][0].read().color_code.bg());
        assert_eq!(Color::Red, writer.buffer[BUFFER_HEIGHT - 2][0].read().color_code.fg());
        assert_eq!(Color::Blue, writer.buffer[BUFFER_HEIGHT - 2][0].read().color_code.bg());

        // Check the color of the old lines
        assert_eq!(Color::Yellow, writer.buffer[0][0].read().color_code.fg());
        assert_eq!(Color::White, writer.buffer[0][0].read().color_code.bg());
    }

    fn construct_writer() -> Writer {
        use std::boxed::Box;

        Writer::new(
            Color::Blue,
            Color::Magenta,
            Box::leak(Box::new(Buffer::new(empty_char()))),
        )
    }

    fn empty_char() -> ScreenChar {
        ScreenChar {
            ascii_character: b' ',
            color_code: ColorCode::new(Color::Green, Color::Brown),
        }
    }
}
