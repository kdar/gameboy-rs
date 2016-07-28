#[derive(Debug)]
pub enum Flag {
  Z, // zero flag
  N, // add/sub flag
  H, // half carry flag
  C, // carry flag
  NZ, // non-zero (uses zero flag)
  NC, // non-carry (uses carry flag)
}

// impl Flag {
//   pub fn pos(&self) -> u8 {
//     match *self {
//       // Flag::NZ => 0b10000000,
//       Flag::Z => 0b10000000,
//       Flag::N => 0b01000000,
//       Flag::H => 0b00100000,
//       // Flag::NC => 0b00010000,
//       Flag::C => 0b00010000,
//       _ => panic!("flag.pos unknown flag"),
//     }
//   }
// }
