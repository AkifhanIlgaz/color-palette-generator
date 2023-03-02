use image::Rgba;
use std::fmt::{self, Display};
use termion::{color, style};

#[derive(Clone, Copy)]
pub struct Color {
    pub rgb: [u8; 3],
    pub lab: lab::Lab,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            rgb: [r, g, b],
            lab: lab::Lab::from_rgb(&[r, g, b]),
        }
    }

    pub fn to_hex_string(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.rgb[0], self.rgb[1], self.rgb[2])
    }

    pub fn to_rgb_string(&self) -> String {
        format!("rgb({},{},{})", self.rgb[0], self.rgb[1], self.rgb[2])
    }

    pub fn color_difference(&self, other_color: &Color) -> f32 {
        self.lab.squared_distance(&other_color.lab).sqrt()
    }

    pub fn average_of_colors(&self, colors: &Vec<Color>) -> Color {
        let average_r = (colors.iter().map(|color| color.rgb[0] as u32).sum::<u32>()
            / colors.len() as u32) as u8;
        let average_g = (colors.iter().map(|color| color.rgb[1] as u32).sum::<u32>()
            / colors.len() as u32) as u8;
        let average_b = (colors.iter().map(|color| color.rgb[2] as u32).sum::<u32>()
            / colors.len() as u32) as u8;

        Self::new(average_r, average_g, average_b)
    }
}

impl From<Rgba<u8>> for Color {
    fn from(rgba: Rgba<u8>) -> Self {
        let mut rgb: [u8; 3] = [0; 3];
        rgb.copy_from_slice(&rgba.0[..3]);
        Self {
            rgb,
            lab: lab::Lab::from_rgba(&rgba.0),
        }
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_creating_color_from_rgb_values() {
        let color = Color::new(27, 41, 53);
        assert_eq!([27, 41, 53], color.rgb);
        assert_eq!(15.88, (color.lab.l * 1000.).round() / 1000.);
        assert_eq!(-2.018, (color.lab.a * 1000.).round() / 1000.);
        assert_eq!(-9.631, (color.lab.b * 1000.).round() / 1000.);
    }

    #[test]
    fn test_creating_color_from_rgba() {
        let rgba = Rgba::from([27_u8, 41, 53, 1]);
        let color = Color::from(rgba);
        assert_eq!([27, 41, 53], color.rgb);
        assert_eq!(15.88, (color.lab.l * 1000.).round() / 1000.);
        assert_eq!(-2.018, (color.lab.a * 1000.).round() / 1000.);
        assert_eq!(-9.631, (color.lab.b * 1000.).round() / 1000.);
    }

    #[test]
    fn test_hex_string() {
        let color = Color::new(27, 41, 53);
        assert_eq!("#1B2935".to_string(), color.to_hex_string());
    }

    #[test]
    fn test_rgb_string() {
        let color = Color::new(27, 41, 53);
        assert_eq!("rgb(27,41,53)".to_string(), color.to_rgb_string());
    }

    #[test]
    fn test_color_difference() {
        let color = Color::new(21, 45, 78);
        let color2 = Color::new(42, 186, 210);

        assert_eq!(
            60.221,
            (color.color_difference(&color2) * 1000.).round() / 1000.
        )
    }
}
