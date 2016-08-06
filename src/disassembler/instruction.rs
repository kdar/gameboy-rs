use super::super::reg::Reg;
use super::super::flag::Flag;
use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum Instruction {
  Invalid(u8),
  InvalidCB(u8),

  // 0xCB instructions
  BIT_b_r(u8, Reg),
  RL_r(Reg),
  RR_r(Reg),
  SRL_r(Reg),

  ADC_A_·HL·,
  ADC_A_n(u8),
  ADC_A_r(Reg),
  ADD_A_·HL·,
  ADD_A_n(u8),
  ADD_HL_rr(Reg),
  AND_n(u8),
  AND_r(Reg),
  CALL_cc_nn(Flag, u16),
  CALL_nn(u16),
  CP_·HL·,
  CP_n(u8),
  DEC_r(Reg),
  DI,
  INC_r(Reg),
  INC_rr(Reg),
  JP_·HL·,
  JP_nn(u16),
  JR_cc_e(Flag, i8),
  JR_e(i8),
  LD_·0xFF00C·_A, // Moved: RET PO -> LD (FF00+n),A
  LD_·0xFF00n·_A(u8), // Moved: JP PO,nn -> LD (FF00+C),A
  LD_·BC·_A,
  LD_·DE·_A,
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
  OR_r(Reg),
  POP_rr(Reg),
  PUSH_rr(Reg),
  RET,
  RET_cc(Flag),
  RLA,
  RLCA,
  RRA,
  SUB_n(u8),
  SUB_r(Reg),
  XOR_n(u8),
  XOR_r(Reg),
}

impl fmt::Debug for Instruction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Instruction::Invalid(d) => write!(f, "DB ${:02x}", d),
      Instruction::InvalidCB(d) => write!(f, "DB ${:02x}", d),

      Instruction::BIT_b_r(b, r) => write!(f, "BIT {},{}", b, r),
      Instruction::RL_r(r) => write!(f, "RL {}", r),
      Instruction::RR_r(r) => write!(f, "RR {}", r),
      Instruction::SRL_r(r) => write!(f, "SRL {}", r),

      Instruction::ADC_A_·HL· => write!(f, "ADC A,(HL)"),
      Instruction::ADC_A_n(n) => write!(f, "ADC A,${:02x}", n),
      Instruction::ADC_A_r(r) => write!(f, "ADC A,{}", r),
      Instruction::ADD_A_·HL· => write!(f, "ADD A,(HL)"),
      Instruction::ADD_A_n(n) => write!(f, "ADD A,${:02x}", n),
      Instruction::ADD_HL_rr(rr) => write!(f, "ADD HL,{}", rr),
      Instruction::AND_n(n) => write!(f, "AND ${:02x}", n),
      Instruction::AND_r(r) => write!(f, "AND {}", r),
      Instruction::CALL_cc_nn(cc, nn) => write!(f, "CALL {},${:04x}", cc, nn),
      Instruction::CALL_nn(nn) => write!(f, "CALL ${:04x}", nn),
      Instruction::CP_·HL· => write!(f, "CP (HL)"),
      Instruction::CP_n(n) => write!(f, "CP ${:02x}", n),
      Instruction::DEC_r(r) => write!(f, "DEC {}", r),
      Instruction::DI => write!(f, "DI"),
      Instruction::INC_r(r) => write!(f, "INC {}", r),
      Instruction::INC_rr(rr) => write!(f, "INC {}", rr),
      Instruction::JP_·HL· => write!(f, "JP (HL)"),
      Instruction::JP_nn(nn) => write!(f, "JP ${:04x}", nn),
      Instruction::JR_cc_e(cc, e) => write!(f, "JR {},{}", cc, e),
      Instruction::JR_e(e) => write!(f, "JR {}", e),
      Instruction::LD_·0xFF00C·_A => write!(f, "LD (0xFF00+C),A"),
      Instruction::LD_·0xFF00n·_A(n) => write!(f, "LD (0xFF00+${:02x}),A", n),
      Instruction::LD_·BC·_A => write!(f, "LD (BC),A"),
      Instruction::LD_·DE·_A => write!(f, "LD (DE),A"),
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
      Instruction::OR_r(r) => write!(f, "OR {}", r),
      Instruction::POP_rr(rr) => write!(f, "POP {}", rr),
      Instruction::PUSH_rr(rr) => write!(f, "PUSH {}", rr),
      Instruction::RET => write!(f, "RET"),
      Instruction::RET_cc(cc) => write!(f, "RET {}", cc),
      Instruction::RLA => write!(f, "RLA"),
      Instruction::RLCA => write!(f, "RLCA"),
      Instruction::RRA => write!(f, "RRA"),
      Instruction::SUB_n(n) => write!(f, "SUB ${:02x}", n),
      Instruction::SUB_r(r) => write!(f, "SUB {}", r),
      Instruction::XOR_n(n) => write!(f, "XOR {}", n),
      Instruction::XOR_r(r) => write!(f, "XOR {}", r),
    }
  }
}
