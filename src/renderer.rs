use crate::argsparsing::*;
use crate::color::*;
use std::vec;
use crate::vector::triangle_2d;

pub fn render_scene(args : &mut render_args) -> () {

  // the grid of pixels to be written out to the image file
  // TODO: implement custom resolution in args
  // TODO: write some more elegant error handling code for the u32 -> usize conversion
  let mut pixelgrid = vec![vec![Color::new(0,0,0); args.get_yres().try_into().unwrap()] ; args.get_xres().try_into().unwrap()];

}

pub fn white_testgrid() -> Vec<Vec<Color>> {

  let mut testgrid : Vec<Vec<Color>> = vec![];
    testgrid.resize_with(200,|| {
      let mut newvec = vec![];
      newvec.resize_with(200, || Color::new(255,255,255));
      newvec
      });

    return testgrid;

}

pub fn draw_triangle(grid : &Vec<Vec<Color>>, triangle : &triangle_2d<i64>) -> () {

}