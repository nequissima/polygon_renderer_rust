pub struct renderArgs {
  xResolution : u32,
  yResolution : u32,
}

impl renderArgs {
  pub fn get_xres(&self) -> u32 {return self.xResolution;}
  pub fn get_yres(&self) -> u32 {return self.yResolution;}
}