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
      _ => panic!("reg.from_raw_byte unknown register: {}", v),
    }
  }
}

impl Reg {
  pub fn from_pair(v: u8) -> Reg {
    match v {
      0b00 => Reg::BC,
      0b01 => Reg::DE,
      0b10 => Reg::HL,
      0b11 => Reg::SP,
      _ => panic!("reg.from_raw_byte unknown register: {}", v),
    }
  }
}
