mod lib;
use lib::*;

use std::env;
use std::ops::{AddAssign, DivAssign};
use std::path::Path;
use image::{ImageBuffer, RgbImage};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Get source path and number of samples from args
    let path = Path::new(args[1].as_str());
    let samples: u32 = args[2].parse().unwrap();

    // Create a cubemap from files in the source path
    let in_cubemap = CubeMap::open(
        path.join("top.png"),
        path.join("bottom.png"),
        path.join("left.png"),
        path.join("right.png"),
        path.join("front.png"),
        path.join("back.png")
    );

    // Create an empty cubemap
    let mut out_cubemap = CubeMap::new(in_cubemap.up.height());

    // Diffuse input map onto output map
    in_cubemap.diffuse_to(&mut out_cubemap, samples);

    // Write to file
    out_cubemap.save();
}
