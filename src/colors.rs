use hsl;
use image::Rgba;
use lab;
use std::{
    fmt::{self, Display},
    ops::Add,
};
use termion::{color, style};

#[derive(Clone)]
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

    pub fn to_hsl_string(&self) -> String {
        let hsl = hsl::HSL::from_rgb(&self.rgb);
        let h = hsl.h.ceil();
        let s = hsl.s * 100.;
        let l = hsl.l * 100.;

        format!("hsl({}°,{:.1}%,{:.1}%)", h, s, l)
    }

    pub fn color_difference(&self, other_color: &Color) -> f32 {
        self.lab.squared_distance(&other_color.lab).sqrt()
    }

    pub fn average_color(&self, other_color: &Color) -> Color {
        let average_r = (((self.rgb[0] as f32).powi(2) + (other_color.rgb[0] as f32).powi(2)) / 2.)
            .sqrt() as u8;

        let average_g = (((self.rgb[1] as f32).powi(2) + (other_color.rgb[1] as f32).powi(2)) / 2.)
            .sqrt() as u8;

        let average_b = (((self.rgb[2] as f32).powi(2) + (other_color.rgb[2] as f32).powi(2)) / 2.)
            .sqrt() as u8;

        Color::new(average_r, average_g, average_b)
    }

    pub fn average_of_colors(&self, colors: &Vec<Color>) -> Color {
        colors.iter().fold(self.clone(), |centroid: Color, color| {
            centroid.average_color(color)
        })
    }
}

impl From<Rgba<u8>> for Color {
    fn from(rgba: Rgba<u8>) -> Self {
        let mut rgb: [u8; 3] = [0; 3];
        for i in 0..3 {
            rgb[i] = rgba.0[i];
        }
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
