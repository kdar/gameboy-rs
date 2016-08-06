use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Reg {
  B,
  C,
  D,
  E,
  H,
  L,
  F,
  A,
  BC,
  DE,
  HL,
  AF,
  SP,
  PC,
}

impl fmt::Display for Reg {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Reg::B => write!(f, "B"),
      Reg::C => write!(f, "C"),
      Reg::D => write!(f, "D"),
      Reg::E => write!(f, "E"),
      Reg::H => write!(f, "H"),
      Reg::L => write!(f, "L"),
      Reg::F => write!(f, "F"),
      Reg::A => write!(f, "A"),
      Reg::BC => write!(f, "BC"),
      Reg::DE => write!(f, "DE"),
      Reg::HL => write!(f, "HL"),
      Reg::AF => write!(f, "AF"),
      Reg::SP => write!(f, "SP"),
      Reg::PC => write!(f, "PC"),
    }
  }
}

impl From<u8> for Reg {
  fn from(v: u8) -> Reg {
    match v {
      0b000 => Reg::B,
      0b001 => Reg::C,
      0b010 => Reg::D,
      0b011 => Reg::E,
      0b100 => Reg::H,
      0b101 => Reg::L,
      0b110 => Reg::F,
      0b111 => Reg::A,
      _ => panic!("reg.from unknown register: {}", v),
    }
  }
}

impl Reg {
  // Some instructions (PUSH rr and POP rr) map 0b11 to AF and others map to SP.
  // Setting use_af to true will map it to AF.
  pub fn from_pair(v: u8, use_af: bool) -> Reg {
    match v {
      0b00 => Reg::BC,
      0b01 => Reg::DE,
      0b10 => Reg::HL,
      0b11 => {
        if use_af {
          Reg::AF
        } else {
          Reg::SP
        }
      }
      _ => panic!("reg.from_raw_byte unknown register: {}", v),
    }
  }
}
