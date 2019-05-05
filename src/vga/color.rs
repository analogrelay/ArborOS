#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }

    pub fn bg(&self) -> Color {
        use core::mem::transmute;
        unsafe {
            transmute(self.0 >> 4)
        }
    }

    pub fn fg(&self) -> Color {
        use core::mem::transmute;
        unsafe {
            transmute(self.0 & 0x0F)
        }
    }

    pub fn set_bg(&mut self, bg: Color) {
        self.0 = (self.0 & 0x0F) | ((bg as u8) << 4);
    }

    pub fn set_fg(&mut self, fg: Color) {
        self.0 = (self.0 & 0xF0) | (fg as u8);
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     pub fn bg_retrieves_background_part() {
//         assert_eq!(Color::Green, ColorCode::new(Color::Red, Color::Green).bg());
//     }

//     #[test]
//     pub fn fg_retrieves_background_part() {
//         assert_eq!(Color::Red, ColorCode::new(Color::Red, Color::Green).fg());
//     }

//     #[test]
//     pub fn set_bg_modifies_background_part() {
//         let mut color = ColorCode::new(Color::Red, Color::Green);
//         color.set_bg(Color::Yellow);
//         assert_eq!(Color::Red, color.fg());
//         assert_eq!(Color::Yellow, color.bg());
//     }

//     #[test]
//     pub fn set_fg_modifies_foreground_part() {
//         let mut color = ColorCode::new(Color::Red, Color::Green);
//         color.set_fg(Color::Yellow);
//         assert_eq!(Color::Yellow, color.fg());
//         assert_eq!(Color::Green, color.bg());
//     }
// }
