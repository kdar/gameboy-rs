use std::fmt;

#[derive(Copy, Clone)]
pub enum Operand {
  A,
  F,
  B,
  C,
  D,
  E,
  H,
  L,
  AF,
  BC,
  DE,
  HL,
  SP,
  PC,
  _BC_,
  _DE_,
  _HL_,
  _SP_,
}

impl fmt::Display for Operand {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Operand::A => write!(f, "A"),
      Operand::F => write!(f, "F"),
      Operand::B => write!(f, "B"),
      Operand::C => write!(f, "C"),
      Operand::D => write!(f, "D"),
      Operand::E => write!(f, "E"),
      Operand::H => write!(f, "H"),
      Operand::L => write!(f, "L"),
      Operand::AF => write!(f, "AF"),
      Operand::BC => write!(f, "BC"),
      Operand::DE => write!(f, "DE"),
      Operand::HL => write!(f, "HL"),
      Operand::SP => write!(f, "SP"),
      Operand::PC => write!(f, "PC"),
      Operand::_BC_ => write!(f, "(BC)"),
      Operand::_DE_ => write!(f, "(DE)"),
      Operand::_HL_ => write!(f, "(HL)"),
      Operand::_SP_ => write!(f, "(SP)"),
    }
  }
}
