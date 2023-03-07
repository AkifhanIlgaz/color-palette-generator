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
        self.centroid = self.calculate_new_centroid();
    }

    pub fn calculate_color_difference_with_centroid(&self, other_color: &Color) -> u32 {
        self.centroid.color_difference(other_color) as u32
    }

    pub fn calculate_new_centroid(&self) -> Color {
        let average_l = self
            .colors
            .iter()
            .map(|color| lab::Lab::from_rgb(&color.rgb).l)
            .sum::<f32>()
            / self.colors.len() as f32;
        let average_a = self
            .colors
            .iter()
            .map(|color| lab::Lab::from_rgb(&color.rgb).a)
            .sum::<f32>()
            / self.colors.len() as f32;
        let average_b = self
            .colors
            .iter()
            .map(|color| lab::Lab::from_rgb(&color.rgb).b)
            .sum::<f32>()
            / self.colors.len() as f32;

        let lab = lab::Lab {
            l: average_l,
            a: average_a,
            b: average_b,
        };
        Color::from(lab)
    }
}
