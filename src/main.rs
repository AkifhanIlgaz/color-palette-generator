use std::env;
use termion::{
    color::*,
    style::{self, *},
};

mod cluster;
mod colors;
mod kmeans;

fn main() {
    /*
        TODO

        - Use clap for argument parsing
        - Unit testing
        - Optimize k-means
        - Accept URL
     
    */

    let args: Vec<String> = env::args().skip(1).collect();
    let [path, k, ..] = args.as_slice() else {
        panic!("Not enough arguments")
    };

    print!("{}{}Creating a palette of ", Fg(White), Bold);
    print!("{}{} ", Fg(LightGreen), k);
    print!("{}colors from ", Fg(White));
    println!("{}{}{}", Fg(Green), path, style::Reset);

    let img = image::open(path).unwrap().thumbnail_exact(200, 200);

    let mut kms = kmeans::KmeansColor::new(k.parse().unwrap(), &img);

    kms.cluster_image(&img);
    kms.print_dominant_colors();

    println!(
        "{}{}✓ Color palette successfully created!{}",
        Fg(Blue),
        style::Bold,
        style::Reset,
    );
}
