mod colors;

#[cfg(test)]
mod tests {
    use image::Rgba;

    use super::*;

    #[test]
    fn test_creating_color_from_rgb_values() {
        let color = colors::Color::new(27, 41, 53);
        assert_eq!([27, 41, 53], color.rgb);
        assert_eq!(15.88, (color.lab.l * 1000.).round() / 1000.);
        assert_eq!(-2.018, (color.lab.a * 1000.).round() / 1000.);
        assert_eq!(-9.631, (color.lab.b * 1000.).round() / 1000.);
    }

    #[test]
    fn test_creating_color_from_rgba() {
        let rgba = Rgba::from([27_u8, 41, 53, 1]);
        let color = colors::Color::from(rgba);
        assert_eq!([27, 41, 53], color.rgb);
        assert_eq!(15.88, (color.lab.l * 1000.).round() / 1000.);
        assert_eq!(-2.018, (color.lab.a * 1000.).round() / 1000.);
        assert_eq!(-9.631, (color.lab.b * 1000.).round() / 1000.);
    }

    #[test]
    fn test_hex_string() {
        let color = colors::Color::new(27, 41, 53);
        assert_eq!("#1B2935".to_string(), color.to_hex_string());
    }

    #[test]
    fn test_rgb_string() {
        let color = colors::Color::new(27, 41, 53);
        assert_eq!("rgb(27,41,53)".to_string(), color.to_rgb_string());
    }

    #[test]
    fn test_color_difference() {
        let color = colors::Color::new(21, 45, 78);
        let color2 = colors::Color::new(42, 186, 210);

        assert_eq!(
            60.221,
            (color.color_difference(&color2) * 1000.).round() / 1000.
        )
    }
}
