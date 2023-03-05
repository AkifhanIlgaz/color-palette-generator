use crate::colors::Color;

pub struct Cluster {
    pub centroid: Color,
    pub colors: Vec<Color>,
}

impl Cluster {
    pub fn new(centroid: Color) -> Cluster {
        Cluster {
            centroid,
            colors: vec![centroid],
        }
    }

    pub fn add_new_color(&mut self, new_color: &Color) {
        self.colors.push(*new_color);
        self.set_new_centroid();
    }

    pub fn set_new_centroid(&mut self) {
        self.centroid = self.average_of_colors();
    }

    pub fn calculate_color_difference_with_centroid(&self, other_color: &Color) -> u32 {
        self.centroid.color_difference(other_color) as u32
    }

    pub fn average_of_colors(&self) -> Color {
        let average_r = (self
            .colors
            .iter()
            .map(|color| color.rgb[0] as u32)
            .sum::<u32>()
            / self.colors.len() as u32) as u8;
        let average_g = (self
            .colors
            .iter()
            .map(|color| color.rgb[1] as u32)
            .sum::<u32>()
            / self.colors.len() as u32) as u8;
        let average_b = (self
            .colors
            .iter()
            .map(|color| color.rgb[2] as u32)
            .sum::<u32>()
            / self.colors.len() as u32) as u8;

        Color::new(average_r, average_g, average_b)
    }
}
