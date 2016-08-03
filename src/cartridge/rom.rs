
#[derive(Debug, PartialEq)]
enum RomSize {
  Kbyte32 = 0x00,
  Kbyte64 = 0x01,
  Kbyte128 = 0x02,
  Kbyte256 = 0x03,
  Kbyte512 = 0x04,
  Mbyte1 = 0x05,
  Mbyte2 = 0x06,
  Mbyte4 = 0x07,
  Mbyte1_1 = 0x52,
  Mbyte1_2 = 0x53,
  Mbyte1_5 = 0x54,
}

impl RomSize {
  pub fn banks(&self) -> usize {
    match *self {
      RomSize::Kbyte32 => 0,
      RomSize::Kbyte64 => 4,
      RomSize::Kbyte128 => 8,
      RomSize::Kbyte256 => 16,
      RomSize::Kbyte512 => 32,
      RomSize::Mbyte1 => 64, // only 63 banks used by MBC1
      RomSize::Mbyte2 => 128, // only 125 banks used by MBC1
      RomSize::Mbyte4 => 256,
      RomSize::Mbyte1_1 => 72,
      RomSize::Mbyte1_2 => 80,
      RomSize::Mbyte1_5 => 96,
    }
  }
}
