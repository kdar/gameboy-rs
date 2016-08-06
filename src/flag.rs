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

impl From<u8> for Flag {
  fn from(v: u8) -> Flag {
    match v {
      0b000 => Flag::NZ,
      0b001 => Flag::Z,
      0b010 => Flag::NC,
      0b011 => Flag::C,
      _ => panic!("flag.from unknown flag: {}", v),
    }
  }
}
