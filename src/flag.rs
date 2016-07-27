#[derive(Debug)]
pub enum Flag {
  Z = 0b10000000, // zero flag
  N = 0b01000000, // add/sub flag
  H = 0b00100000, // half carry flag
  C = 0b00010000, // carry flag
  NZ,
  NC,
}

// impl Flag {
//   fn pos(&self) -> u8 {
//     match *self {
//       Flag::NZ => 0b10000000,
//       Flag::Z => 0b10000000,
//       Flag::N => 0b01000000,
//       Flag::H => 0b00100000,
//       Flag::NC => 0b00010000,
//       Flag::C => 0b00010000,
//     }
//   }
// }
