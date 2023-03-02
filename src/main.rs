use std::{collections::HashMap, fs};

// use rand::{self, Rng};
// use spinners::{Spinner, Spinners};
use image::{DynamicImage, GenericImageView, Rgb, Rgba};
use termion::{color, style};

mod colors;
mod kmeans;

fn main() {
    // // // Arc,
    // // let mut sp = Spinner::new(Spinners::Arc, "Loading things into memory...".into());

    // // sp.stop_with_message("All things loaded into memory succesfully ~".to_string());
    // let mut colors: Vec<(u8, u8, u8)> = vec![];
    // let mut rng = rand::thread_rng();

    // for _ in 0..5 {
    //     let r = rng.gen_range(0..=255);
    //     let g = rng.gen_range(0..=255);
    //     let b = rng.gen_range(0..=255);
    //     colors.push((r, g, b));
    // }

    // print!(
    //     "{}{}Creating a palette of ",
    //     color::Fg(color::White),
    //     style::Bold
    // );
    // // TODO: Take number of colors as an argument, default 5
    // print!("{}5 ", color::Fg(color::Blue));
    // println!("{}{}colors", color::Fg(color::White), style::Bold);

    // for color in colors {
    //     println!(
    //         "{}  {}",
    //         color::Bg(color::Rgb(color.0, color.1, color.2)),
    //         style::Reset
    //     );
    // }

    let img = image::open("./samurai.jpg").unwrap();

    let mut color_count = HashMap::new();
    let mut max = 0;
    let mut dominant: Option<Rgba<u8>> = None;
    for (_, _, rgb_color) in img.pixels() {
        *color_count.entry(rgb_color).or_insert(0) += 1
    }
}
