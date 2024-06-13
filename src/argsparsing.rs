pub struct renderArgs {
  x_resolution : u32,
  y_resolution : u32,
}

impl renderArgs {
  pub fn get_xres(&self) -> u32 {return self.x_resolution;}
  pub fn get_yres(&self) -> u32 {return self.y_resolution;}
}