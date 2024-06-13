use crate::color::*;
use std::io::Error;
use std::fs;

// takes a reference to the pixel grid and a filepath and tries writing
// the grid to the file in PPM format
pub fn writePixelGridToFile(grid: &Vec<Vec<Color>>, filepath : &str) -> Result<(), Error> {

  //open a file for writing

  for x in 0..grid.len() {
    for y in 0..grid.len() {
      grid[x][y];
    }
  }

  return Ok(());
}