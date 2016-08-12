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
  JR_cc_e(Flag, i8),
  JR_e(i8),
  LD_·0xFF00C·_A, // Moved: RET PO -> LD (FF00+n),A
  LD_·0xFF00n·_A(u8), // Moved: JP PO,nn -> LD (FF00+C),A
  LD_·BC·_A,
  LD_·DE·_A,
  LD_·HL·_n(u8),
  LD_·HL·_r(Reg),
  LD_·nn·_A(u16), // Moved: JP PE,nn -> LD (nn),A
  LD_·nn·_SP(u16), // Moved: EX AF,AF -> LD (nn),SP
  LD_A_·BC·,
  LD_A_·DE·,
  LD_A_·nn·(u16), // Moved: JP M,nn -> LD A,(nn)
  LD_A_·0xFF00n·(u8), // Moved: RET P -> LD A,(FF00+n)
  LD_dd_nn(Reg, u16),
  LD_r_·HL·(Reg),
  LD_r_n(Reg, u8),
  LD_r_r(Reg, Reg),
  LDI_A_·HL·, // Moved: LD HL,(nn) -> LDI A,(HL)
  LDD_·HL·_A, // Moved: LD (nn),A -> LDD (HL),A
  LDI_·HL·_A, // Moved: LD (nn),HL -> LDI (HL),A
  NOP,
  OR_A_·HL·,
  OR_r(Reg),
  POP_rr(Reg),
  PUSH_rr(Reg),
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
      Instruction::JR_cc_e(cc, e) => write!(f, "JR {},{}", cc, e),
      Instruction::JR_e(e) => write!(f, "JR {}", e),
      Instruction::LD_·0xFF00C·_A => write!(f, "LD (0xFF00+C),A"),
      Instruction::LD_·0xFF00n·_A(n) => write!(f, "LD (0xFF00+${:02x}),A", n),
      Instruction::LD_·BC·_A => write!(f, "LD (BC),A"),
      Instruction::LD_·DE·_A => write!(f, "LD (DE),A"),
      Instruction::LD_·HL·_n(n) => write!(f, "LD (HL),${:02x}", n),
      Instruction::LD_·HL·_r(r) => write!(f, "LD (HL),{}", r),
      Instruction::LD_·nn·_A(nn) => write!(f, "LD (${:04x}),A", nn),
      Instruction::LD_·nn·_SP(nn) => write!(f, "LD (${:04x}),SP", nn),
      Instruction::LD_A_·BC· => write!(f, "LD A,(BC)"),
      Instruction::LD_A_·DE· => write!(f, "LD A,(DE)"),
      Instruction::LD_A_·nn·(nn) => write!(f, "LD A,${:04x}", nn),
      Instruction::LD_A_·0xFF00n·(n) => write!(f, "LD A,(0xFF00+${:02x})", n),
      Instruction::LD_dd_nn(dd, nn) => write!(f, "LD {},${:04x}", dd, nn),
      Instruction::LD_r_·HL·(r) => write!(f, "LD {},(HL)", r),
      Instruction::LD_r_n(r, n) => write!(f, "LD {},${:02x}", r, n),
      Instruction::LD_r_r(r1, r2) => write!(f, "LD {},{}", r1, r2),
      Instruction::LDI_A_·HL· => write!(f, "LDI A,(HL)"),
      Instruction::LDD_·HL·_A => write!(f, "LDD (HL),A"),
      Instruction::LDI_·HL·_A => write!(f, "LDI (HL),A"),
      Instruction::NOP => write!(f, "NOP"),
      Instruction::OR_A_·HL· => write!(f, "OR A,(HL)"),
      Instruction::OR_r(r) => write!(f, "OR {}", r),
      Instruction::POP_rr(rr) => write!(f, "POP {}", rr),
      Instruction::PUSH_rr(rr) => write!(f, "PUSH {}", rr),
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
