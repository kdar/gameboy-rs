
#[derive(Debug, PartialEq)]
enum RomSize {
  Rom32KByte = 0x00,
  Rom64KByte = 0x01,
  Rom128KByte = 0x02,
  Rom256KByte = 0x03,
  Rom512KByte = 0x04,
  Rom1MByte = 0x05,
  Rom2MByte = 0x06,
  Rom4MByte = 0x07,
  Rom1_1MByte = 0x52,
  Rom1_2MByte = 0x53,
  Rom1_5MByte = 0x54,
}

impl RomSize {
  pub fn banks(&self) -> usize {
    match *self {
      RomSize::Rom32KByte => 0,
      RomSize::Rom64KByte => 4,
      RomSize::Rom128KByte => 8,
      RomSize::Rom256KByte => 16,
      RomSize::Rom512KByte => 32,
      RomSize::Rom1MByte => 64, // only 63 banks used by MByteC1
      RomSize::Rom2MByte => 128, // only 125 banks used by MByteC1
      RomSize::Rom4MByte => 256,
      RomSize::Rom1_1MByte => 72,
      RomSize::Rom1_2MByte => 80,
      RomSize::Rom1_5MByte => 96,
    }
  }
}
