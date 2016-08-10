use std::fmt;

#[derive(Copy, Clone)]
pub enum Value {
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

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Value::A => write!(f, "A"),
      Value::F => write!(f, "F"),
      Value::B => write!(f, "B"),
      Value::C => write!(f, "C"),
      Value::D => write!(f, "D"),
      Value::E => write!(f, "E"),
      Value::H => write!(f, "H"),
      Value::L => write!(f, "L"),
      Value::AF => write!(f, "AF"),
      Value::BC => write!(f, "BC"),
      Value::DE => write!(f, "DE"),
      Value::HL => write!(f, "HL"),
      Value::SP => write!(f, "SP"),
      Value::PC => write!(f, "PC"),
      Value::_BC_ => write!(f, "(BC)"),
      Value::_DE_ => write!(f, "(DE)"),
      Value::_HL_ => write!(f, "(HL)"),
      Value::_SP_ => write!(f, "(SP)"),
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
