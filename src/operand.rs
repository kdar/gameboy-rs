use std::fmt;

use super::flag::Flag;
use super::reg::Reg;

#[derive(Copy, Clone)]
pub enum Addr {
  BC,
  DE,
  HL,
  SP,
  Imm16(u16),
  OffsetReg(Imm, Reg),
  OffsetImm8(Imm, Imm),
}

impl fmt::Display for Addr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Addr::BC => write!(f, "(BC)"),
      Addr::DE => write!(f, "(DE)"),
      Addr::HL => write!(f, "(HL)"),
      Addr::SP => write!(f, "(SP)"),
      Addr::Imm16(i) => write!(f, "(${:04x})", i),
      Addr::OffsetReg(o, r) => write!(f, "({}+{})", o, r),
      Addr::OffsetImm8(o, i) => write!(f, "({}+{})", o, i),
    }
  }
}

#[derive(Copy, Clone)]
pub enum Imm {
  Imm8(u8),
  Imm16(u16),
}

impl fmt::Display for Imm {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Imm::Imm8(i) => write!(f, "${:02x}", i),
      Imm::Imm16(i) => write!(f, "${:04x}", i),
    }
  }
}

#[derive(Copy, Clone)]
pub enum Operand {
  Reg(Reg),
  Addr(Addr),
  Flag(Flag),
  Imm(Imm), /* Immi8(i8),
             * Immi16(i16),
             * _Imm16_(u16),
             * _OffsetReg_(u16, Operand),
             * IoPortC,
             * IoPortImm16(u16), */
}

impl fmt::Display for Operand {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Operand::Reg(r) => write!(f, "{}", r),
      Operand::Addr(a) => write!(f, "{}", a),
      Operand::Flag(fl) => write!(f, "{}", fl),
      Operand::Imm(i) => write!(f, "{}", i),
      // Operand::_Imm16_(i) => write!(f, "(${:04x})", i),
      // Operand::_OffsetReg_(o, r) => write!(f, "(${:04x}+{})", o, r),
      // Operand::IoPortImm16(i) => write!(f, "($FF00+${:04x})", i),
      //
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
