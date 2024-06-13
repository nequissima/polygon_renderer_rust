pub mod color;
pub mod vector;
pub mod argsparsing;
pub mod renderer;
pub mod fileio;

const XRES : u32 = 200;
const YRES : u32 = 200;

use std::env;

fn main() {

    let args : Vec<String> = env::args().collect();
    let filename : &str;

    match args.get(1) {
      Some(x) => filename = x,
      None=> panic!("you have to provide an argument!!")
    }

    println!("The filename entered was {filename}")


}