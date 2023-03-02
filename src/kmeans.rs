use crate::cluster::Cluster;
use crate::colors::Color;
use image::{DynamicImage, GenericImageView};
use rand::{self, Rng};
use spinners::{Spinner, Spinners};
use termion::{color, style};
pub struct KmeansColor {
    clusters: Vec<Cluster>,
}

impl KmeansColor {
    pub fn new(k: usize, img: &DynamicImage) -> KmeansColor {
        // let img = img.thumbnail_exact(200, 200);
        let mut clusters = vec![];
        let mut random = rand::thread_rng();

        for _ in 0..k {
            let random_centroid =
                Color::from(img.get_pixel(random.gen_range(0..200), random.gen_range(0..200)));
            let cluster = Cluster::new(random_centroid);
            clusters.push(cluster);
        }

        KmeansColor { clusters }
    }

    pub fn run(&mut self, img: &DynamicImage) {


        let mut sp = Spinner::new(Spinners::Arc, String::default());

        for (_, _, rgb_color) in img.pixels().step_by(2) {
            let new_color = Color::from(rgb_color);
            self.add_new_color(&new_color);
        }

        sp.stop();
        println!();
        self.print_dominant_colors();
        println!(
            "{}{}✓ Success!{}",
            color::Fg(color::Green),
            style::Bold,
            style::Reset,
        );
    }

    pub fn add_new_color(&mut self, new_color: &Color) {
        let nearest_cluster = self
            .clusters
            .iter_mut()
            .min_by_key(|cluster| cluster.calculate_color_difference_with_centroid(new_color))
            .unwrap();

        nearest_cluster.add_new_color(new_color);
    }

    pub fn print_dominant_colors(&self) {
        for cluster in &self.clusters {
            let color = cluster.centroid;
            print!("{}  ", color);
            print!(
                "{}{}{}  ",
                color::Fg(color::LightWhite),
                style::Bold,
                color.to_hex_string()
            );
            print!(
                "{}{}{}  ",
                color::Fg(color::LightWhite),
                style::Bold,
                color.to_rgb_string()
            );
            println!(
                "{}{}%{}  {}",
                color::Fg(color::LightWhite),
                style::Bold,
                (cluster.colors.len() as f32 / 400.),
                style::Reset
            )
        }
    }
}
