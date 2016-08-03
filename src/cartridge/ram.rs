#[derive(Debug, PartialEq)]
enum RamSize {
  None = 0x00,
  Kbyte2 = 0x01,
  Kbyte8 = 0x02,
  Kbyte32 = 0x03,
}
