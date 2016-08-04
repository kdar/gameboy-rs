#[derive(Debug, PartialEq, NumFromPrimitive)]
#[allow(enum_variant_names)]
pub enum CartRamSize {
  None = 0x00,
  Ram2KB = 0x01,
  Ram8KB = 0x02,
  Ram32KB = 0x03,
  Ram128KB = 0x04,
}

impl CartRamSize {
  pub fn as_usize(&self) -> usize {
    match *self {
      CartRamSize::None => 0,
      CartRamSize::Ram2KB => 2048,
      CartRamSize::Ram8KB => 8192,
      CartRamSize::Ram32KB => 32768,
      CartRamSize::Ram128KB => 131072,
    }
  }
}
