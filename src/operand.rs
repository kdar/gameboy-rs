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


// impl Value {
//  fn from_reg8(v: u8) -> Reg {
//    match v {
//      0b000 => Reg::B,
//      0b001 => Reg::C,
//      0b010 => Reg::D,
//      0b011 => Reg::E,
//      0b100 => Reg::H,
//      0b101 => Reg::L,
//      0b110 => Reg::F,
//      0b111 => Reg::A,
//      _ => panic!("value.from_reg8 unknown register: {}", v),
//    }
//  }
//
//  // Some instructions (PUSH rr and POP rr) map 0b11 to AF and others map to SP.
//  // Setting use_af to true will map it to AF.
//  pub fn from_reg16(v: u8, use_af: bool) -> Reg {
//    match v {
//      0b00 => Reg::BC,
//      0b01 => Reg::DE,
//      0b10 => Reg::HL,
//      0b11 => {
//        if use_af {
//          Reg::AF
//        } else {
//          Reg::SP
//        }
//      }
//      _ => panic!("value.from_reg16 unknown register: {}", v),
//    }
//  }
// }
