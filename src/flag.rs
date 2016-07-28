use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Flag {
  Z, // zero flag
  N, // add/sub flag
  H, // half carry flag
  C, // carry flag
  NZ, // non-zero (uses zero flag)
  NC, // non-carry (uses carry flag)
}

impl fmt::Display for Flag {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Flag::Z => write!(f, "Z"),
      Flag::N => write!(f, "N"),
      Flag::H => write!(f, "H"),
      Flag::C => write!(f, "C"),
      Flag::NZ => write!(f, "NZ"),
      Flag::NC => write!(f, "NC"),
    }
  }
}

// impl Flag {
//   pub fn from(&self, v: u8) -> Flag {
//     match v {
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
