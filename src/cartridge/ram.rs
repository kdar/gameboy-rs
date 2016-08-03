#[derive(Debug, PartialEq)]
enum Ram {
  RamNone = 0x00,
  Ram2KByte = 0x01,
  Ram8KByte = 0x02,
  Ram32KByte = 0x03,
  Ram,
}
