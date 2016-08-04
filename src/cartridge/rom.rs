const ROM_BANK_SIZE: usize = 0x4000;

#[derive(Debug, PartialEq, NumFromPrimitive)]
#[allow(enum_variant_names, non_camel_case_types)]
pub enum CartRomSize {
  Rom32KB = 0x00, // 256kbit
  Rom64KB = 0x01, // 512kbit
  Rom128KB = 0x02, // 1mbit
  Rom256KB = 0x03, // 2mbit
  Rom512KB = 0x04, // 4mbit
  Rom1MB = 0x05, // 8mbit
  Rom2MB = 0x06, // 16mbit
  Rom4MB = 0x07, // 32mbit
  Rom1_1MB = 0x52,
  Rom1_2MB = 0x53,
  Rom1_5MB = 0x54,
}

impl CartRomSize {
  pub fn banks(&self) -> usize {
    match *self {
      CartRomSize::Rom32KB => 0,
      CartRomSize::Rom64KB => 4,
      CartRomSize::Rom128KB => 8,
      CartRomSize::Rom256KB => 16,
      CartRomSize::Rom512KB => 32,
      CartRomSize::Rom1MB => 64, // only 63 banks used by MBC1
      CartRomSize::Rom2MB => 128, // only 125 banks used by MBC1
      CartRomSize::Rom4MB => 256,
      CartRomSize::Rom1_1MB => 72,
      CartRomSize::Rom1_2MB => 80,
      CartRomSize::Rom1_5MB => 96,
    }
  }

  pub fn as_usize(&self) -> usize {
    ROM_BANK_SIZE * self.banks()
  }
}
