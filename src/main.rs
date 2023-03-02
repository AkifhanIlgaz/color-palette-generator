use std::env;

use termion::{color, style};

mod cluster;
mod colors;
mod kmeans;

fn main() {
    let path = env::args().nth(1).unwrap();
    let k = env::args().nth(2).unwrap();
    print!(
        "{}{}Creating a palette of ",
        color::Fg(color::White),
        style::Bold
    );
    print!("{}{} ", color::Fg(color::Blue), k);
    print!("{}colors from ", color::Fg(color::White));
    println!("{}{}{}", color::Fg(color::Blue), path, style::Reset);
    // TODO: color palette of <path>
    let img = image::open(path).unwrap().thumbnail_exact(200, 200);

    let mut kms = kmeans::KmeansColor::new(k.parse().unwrap(), &img);

    kms.run(&img);
}
