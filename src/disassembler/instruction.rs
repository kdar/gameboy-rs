use super::super::reg::Reg;
use super::super::flag::Flag;
use super::super::operand::Operand;
use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum Instruction {
  Invalid(u8),
  InvalidCB(u8),

  // 0xCB instructions
  BIT(Operand, Operand),
  RL(Operand),
  RR(Operand),
  SRL(Operand),
  SWAP(Operand),

  ADC(Operand, Operand),
  ADD8(Operand, Operand),
  ADD16(Operand, Operand),
  AND(Operand),
  CALL(Operand),
  CALL_cc(Operand, Operand),
  CP(Operand),
  DEC8(Operand),
  DEC16(Operand),
  DI,
  EI,
  HALT,
  INC8(Operand),
  INC16(Operand),
  JP(Operand),
  JP_cc(Operand, Operand),
  JR(Operand),
  JR_cc(Operand, Operand),
  LD8(Operand, Operand),
  LD16(Operand, Operand),
  LDD(Operand, Operand),
  LDI(Operand, Operand),
  NOP,
  OR(Operand, Operand),
  POP16(Operand),
  PUSH16(Operand),
  RET,
  RET_cc(Flag),
  RLA,
  RLCA,
  RRA,
  RST_t(u8),
  SUB_n(u8),
  SUB_r(Reg),
  XOR_·HL·,
  XOR_n(u8),
  XOR_r(Reg),
}

impl fmt::Debug for Instruction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Instruction::Invalid(d) => write!(f, "DB ${:02x}", d),
      Instruction::InvalidCB(d) => write!(f, "DB ${:02x}", d),

      Instruction::BIT(o1, o2) => write!(f, "BIT {},{}", o1, o2),
      Instruction::RL(o) => write!(f, "RL {}", o),
      Instruction::RR(o) => write!(f, "RR {}", o),
      Instruction::SRL(o) => write!(f, "SRL {}", o),
      Instruction::SWAP(o) => write!(f, "SWAP {}", o),

      Instruction::ADC(o1, o2) => write!(f, "ADC {},{}", o1, o2),
      Instruction::ADD8(o1, o2) => write!(f, "ADD {},{}", o1, o2),
      Instruction::ADD16(o1, o2) => write!(f, "ADD {},{}", o1, o2),
      Instruction::AND(o) => write!(f, "AND {}", o),
      Instruction::CALL_cc(o1, o2) => write!(f, "CALL {},{}", o1, o2),
      Instruction::CALL(o) => write!(f, "CALL {}", o),
      Instruction::CP(o) => write!(f, "CP {}", o),
      Instruction::DEC8(o) => write!(f, "DEC {}", o),
      Instruction::DEC16(o) => write!(f, "DEC {}", o),
      Instruction::DI => write!(f, "DI"),
      Instruction::EI => write!(f, "EI"),
      Instruction::HALT => write!(f, "HALT"),
      Instruction::INC8(o) => write!(f, "INC {}", o),
      Instruction::INC16(o) => write!(f, "INC {}", o),
      Instruction::JP(o) => write!(f, "JP {}", o),
      Instruction::JP_cc(o1, o2) => write!(f, "JP {},{}", o1, o2),
      Instruction::JR(o) => write!(f, "JR {}", o),
      Instruction::JR_cc(o1, o2) => write!(f, "JR {},{}", o1, o2),
      Instruction::LD8(o1, o2) => write!(f, "LD {},{}", o1, o2),
      Instruction::LD16(o1, o2) => write!(f, "LD {},{}", o1, o2),
      Instruction::LDD(o1, o2) => write!(f, "LDD {},{}", o1, o2),
      Instruction::LDI(o1, o2) => write!(f, "LDI {},{}", o1, o2),
      Instruction::NOP => write!(f, "NOP"),
      Instruction::OR(o1, o2) => write!(f, "OR {},{}", o1, o2),
      Instruction::POP16(o) => write!(f, "POP {}", o),
      Instruction::PUSH16(o) => write!(f, "PUSH {}", o),
      Instruction::RET => write!(f, "RET"),
      Instruction::RET_cc(cc) => write!(f, "RET {}", cc),
      Instruction::RLA => write!(f, "RLA"),
      Instruction::RLCA => write!(f, "RLCA"),
      Instruction::RRA => write!(f, "RRA"),
      Instruction::RST_t(t) => write!(f, "RST ${:02x}", t),
      Instruction::SUB_n(n) => write!(f, "SUB ${:02x}", n),
      Instruction::SUB_r(r) => write!(f, "SUB {}", r),
      Instruction::XOR_·HL· => write!(f, "XOR (HL)"),
      Instruction::XOR_n(n) => write!(f, "XOR ${:02x}", n),
      Instruction::XOR_r(r) => write!(f, "XOR {}", r),
    }
  }
}
