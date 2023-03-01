use image::Rgba;
use std::fmt::{self, Display};
use termion::{color, style};
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> RGB {
        RGB { r, g, b }
    }

    pub fn to_hex_string(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    pub fn to_rgb_string(&self) -> String {
        format!("rgb({},{},{})", self.r, self.g, self.b)
    }

    pub fn to_hsl_string(&self) -> String {
        todo!()
    }

    pub fn name(&self) -> String {
        todo!()
    }
}

impl From<Rgba<Vec<u8>>> for RGB {
    fn from(value: Rgba<Vec<u8>>) -> Self {
        todo!()
    }
}

impl Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}  {}   {}{} {}    {}",
            color::Bg(color::Rgb(self.r, self.g, self.b)),
            style::Reset,
            color::Fg(color::LightBlack),
            style::Blink,
            self.to_hex_string(),
            self.to_rgb_string()
        )
    }
}
