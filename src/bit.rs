pub trait Bit {
  fn has_bit(&self, mask: usize) -> bool;
  fn set_bit(&mut self, mask: usize);
}


impl Bit for u8 {
  fn has_bit(&self, mask: usize) -> bool {
    (self & mask as u8) != 0
  }

  fn set_bit(&mut self, mask: usize) {
    *self |= mask as u8;
  }
}

impl Bit for u16 {
  fn has_bit(&self, mask: usize) -> bool {
    (self & mask as u16) != 0
  }

  fn set_bit(&mut self, mask: usize) {
    *self |= mask as u16;
  }
}
