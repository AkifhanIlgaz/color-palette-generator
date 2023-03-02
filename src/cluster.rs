use crate::colors::Color;

pub struct Cluster {
    pub centroid: Color,
    pub colors: Vec<Color>,
}

impl Cluster {
    pub fn new(centroid: Color) -> Cluster {
        Cluster {
            centroid,
            colors: vec![],
        }
    }

    pub fn add_new_color(&mut self, new_color: &Color) {
        self.colors.push(*new_color);
        self.set_new_centroid();
    }

    pub fn set_new_centroid(&mut self) {
        self.centroid = self.centroid.average_of_colors(&self.colors);
    }

    pub fn calculate_color_difference_with_centroid(&self, other_color: &Color) -> u32 {
        self.centroid.color_difference(other_color) as u32
    }
}
