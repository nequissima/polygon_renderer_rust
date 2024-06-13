pub mod color;
pub mod vector;
pub mod argsparsing;
pub mod renderer;
pub mod fileio;

const XRES : usize = 200;
const YRES : usize = 200;

use std::env;
use color::Color;
use fileio::*;
use renderer::*;

fn main() {

    let args : Vec<String> = env::args().collect();
    let filename : &str;

    match args.get(1) {
      Some(x) => filename = x,
      None=> panic!("you have to provide an argument!!")
    }

    println!("The filename entered was {filename}");

    let testgrid = white_testgrid();

    write_pixel_grid_to_file(&testgrid, (XRES,YRES), filename).expect("something fucked up");
}