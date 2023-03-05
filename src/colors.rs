use image::Rgba;
use std::fmt::{self, Display};
use termion::{color, style};

#[derive(Clone, Copy, PartialEq)]
pub struct Color {
    pub rgb: [u8; 3],
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { rgb: [r, g, b] }
    }

    pub fn to_hex_string(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.rgb[0], self.rgb[1], self.rgb[2])
    }

    pub fn to_rgb_string(&self) -> String {
        format!("rgb({},{},{})", self.rgb[0], self.rgb[1], self.rgb[2])
    }

    pub fn color_difference(&self, other_color: &Color) -> f32 {
        let self_lab = lab::Lab::from_rgb(&self.rgb);
        let other_color_lab = lab::Lab::from_rgb(&other_color.rgb);
        self_lab.squared_distance(&other_color_lab).sqrt()
    }
}

impl From<Rgba<u8>> for Color {
    fn from(rgba: Rgba<u8>) -> Self {
        let mut rgb: [u8; 3] = [0; 3];
        rgb.copy_from_slice(&rgba.0[..3]);
        Self { rgb }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}  {}",
            color::Bg(color::Rgb(self.rgb[0], self.rgb[1], self.rgb[2])),
            style::Reset
        )
    }
}

