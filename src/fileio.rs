use crate::color::*;
use std::io::Error;
use std::fs::write;
use std::str;

// takes a reference to the pixel grid and a filepath and tries writing
// the grid to the file in PPM format
pub fn write_pixel_grid_to_file(grid: &Vec<Vec<Color>>, grid_dimension : (usize, usize), filepath : &str) -> Result<(), Error> {

  // create string to be written
  let mut output = format!("P3\n{} {}\n255\n", grid_dimension.0, grid_dimension.1);

  // writing each color in the grid to the string
  for x in 0..grid.len() {
    for y in 0..grid.len() {
      let str = format!("{} {} {} ", grid[x][y].get_r(),
                                            grid[x][y].get_g(),
                                            grid[x][y].get_b());
      output.push_str(&str);
    }
    output.push('\n');
  }

  // attempting to write the string to a file
  let write_result = write(filepath, &output);

  // returning the result
  return write_result;
}