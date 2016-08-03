pub trait Bit {
  fn has_bit(&self, position: usize) -> bool;
  fn set_bit(&mut self, position: usize);
}


impl Bit for u8 {
  fn has_bit(&self, position: usize) -> bool {
    (self & (0b00000001 << position as u8)) != 0
  }

  fn set_bit(&mut self, position: usize) {
    *self |= 0b00000001 << position as u8;
  }
}

impl Bit for u16 {
  fn has_bit(&self, position: usize) -> bool {
    (self & (0b0000000000000001 << position as u16)) != 0
  }

  fn set_bit(&mut self, position: usize) {
    *self |= 0b0000000000000001 << position as u16;
  }
}
