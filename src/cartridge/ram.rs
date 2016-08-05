pub const RAM_BANK_SIZE: usize = 0x2000;

#[derive(Debug, PartialEq, NumFromPrimitive)]
#[allow(enum_variant_names)]
pub enum RamSize {
  None = 0x00,
  Ram2KB = 0x01,
  Ram8KB = 0x02,
  Ram32KB = 0x03,
  Ram128KB = 0x04,
}

impl RamSize {
  pub fn as_usize(&self) -> usize {
    match *self {
      RamSize::None => 0,
      RamSize::Ram2KB => 2048,
      RamSize::Ram8KB => 8192,
      RamSize::Ram32KB => 32768,
      RamSize::Ram128KB => 131072,
    }
  }
}
