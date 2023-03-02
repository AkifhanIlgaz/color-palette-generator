use std::{collections::HashMap, fs};

// use rand::{self, Rng};
// use spinners::{Spinner, Spinners};
use image::{DynamicImage, GenericImageView, Rgb, Rgba};
use rand::Rng;
use spinners::{Spinner, Spinners};
use termion::{color, style};

mod cluster;
mod colors;
mod kmeans;

fn main() {
    let img = image::open("./akira-neo-tokyo-7.png")
        .unwrap()
        .thumbnail_exact(200, 200);

    let mut kms = kmeans::KmeansColor::new(3, &img);

    kms.run(&img);
}
