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
  FlagZ,
  FlagN,
  FlagH,
  FlagC,
  FlagNZ,
  FlagNC,
  Imm8(u8),
  Imm16(u16), /* Immi8(i8),
               * Immi16(i16), */
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
      Operand::FlagZ => write!(f, "Z"),
      Operand::FlagN => write!(f, "N"),
      Operand::FlagH => write!(f, "H"),
      Operand::FlagC => write!(f, "C"),
      Operand::FlagNZ => write!(f, "NZ"),
      Operand::FlagNC => write!(f, "NC"),
      Operand::Imm8(i) => write!(f, "${:02x}", i),
      Operand::Imm16(i) => write!(f, "${:04x}", i),
      // Operand::Immi8(i) => {
      //  if i < 0 {
      //    write!(f, "-${:02x}", i)
      //  } else {
      //    write!(f, "${:02x}", i)
      //  }
      // }
      // Operand::Immi16(i) => {
      //  if i < 0 {
      //    write!(f, "-${:04x}", i)
      //  } else {
      //    write!(f, "${:04x}", i)
      //  }
      // }
    }
  }
}
