pub mod piston;

pub trait Display {
  fn set_pixel(&mut self, x: u32, y: u32, color: [u8; 4]);
  fn swap(&mut self);
}

pub struct NullDisplay;

impl Display for NullDisplay {
  fn set_pixel(&mut self, x: u32, y: u32, color: [u8; 4]) {}
  fn swap(&mut self) {}
}

impl Default for NullDisplay {
  fn default() -> NullDisplay {
    NullDisplay
  }
}
