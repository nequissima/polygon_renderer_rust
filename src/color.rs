#[derive(Clone)]
#[derive(Copy)]
pub struct Color {
  r: u8,
  g: u8,
  b: u8,
}

impl Color {
  pub fn get_r (&self) -> u8 {return self.r}
  pub fn get_g (&self) -> u8 {return self.g}
  pub fn get_b (&self) -> u8 {return self.b}

  pub fn set_r (&mut self, new_r : u8) -> () {self.r = new_r;}
  pub fn set_g (&mut self, new_g : u8) -> () {self.g = new_g;}
  pub fn set_b (&mut self, new_b : u8) -> () {self.g = new_b;}

  pub fn new(r : u8, g : u8, b : u8) -> Color {return Color{r, g, b}}
}