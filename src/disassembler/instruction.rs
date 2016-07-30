use super::super::reg::Reg;
use super::super::flag::Flag;
use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum Instruction {
  Invalid,
  BIT_b_r(u8, Reg),
  CALL_nn(u16),
  CP_n(u8),
  DEC_r(Reg),
  INC_r(Reg),
  INC_rr(Reg),
  JR_cc_e(Flag, i8),
  JR_e(i8),
  LD_·0xFF00C·_A, // Moved: RET PO -> LD (FF00+n),A
  LD_·0xFF00n·_A(u8), // Moved: JP PO,nn -> LD (FF00+C),A
  LD_·HL·_r(Reg),
  LD_·nn·_A(u16), // Moved: JP PE,nn => LD (nn),A
  LD_A_·DE·,
  LD_A_·0xFF00n·(u8), // Moved: RET P -> LD A,(FF00+n)
  LD_dd_nn(Reg, u16),
  LD_r_n(Reg, u8),
  LD_r_r(Reg, Reg),
  LDD_·HL·_A, // Moved: LD (nn),A -> LDD (HL),A
  LDI_·HL·_A, // Moved: LD (nn),HL -> LDI (HL),A
  PUSH_rr(Reg),
  NOP,
  XOR_r(Reg),
}

impl fmt::Debug for Instruction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Instruction::BIT_b_r(b, r) => write!(f, "BIT {},{}", b, r),
      Instruction::CALL_nn(nn) => write!(f, "CALL {}", nn),
      Instruction::CP_n(n) => write!(f, "CP ${:02x}", n),
      Instruction::DEC_r(r) => write!(f, "DEC {}", r),
      Instruction::INC_r(r) => write!(f, "INC {}", r),
      Instruction::INC_rr(r) => write!(f, "INC {}", r),
      Instruction::JR_cc_e(cc, e) => write!(f, "JR {},{}", cc, e),
      Instruction::JR_e(e) => write!(f, "JR {}", e),
      Instruction::LD_·0xFF00C·_A => write!(f, "LD (0xFF00+C),A"),
      Instruction::LD_·0xFF00n·_A(n) => write!(f, "LD (0xFF00+{}),A", n),
      Instruction::LD_·HL·_r(r) => write!(f, "LD (HL),{}", r),
      Instruction::LD_·nn·_A(nn) => write!(f, "LD (${:#04x}),A", nn),
      Instruction::LD_A_·DE· => write!(f, "LD A,(DE)"),
      Instruction::LD_A_·0xFF00n·(n) => write!(f, "LD A,(0xFF00+{})", n),
      Instruction::LD_dd_nn(dd, nn) => write!(f, "LD {},${:#04x}", dd, nn),
      Instruction::LD_r_n(r, n) => write!(f, "LD {},${:#02x}", r, n),
      Instruction::LD_r_r(r1, r2) => write!(f, "LD {},{}", r1, r2),
      Instruction::LDD_·HL·_A => write!(f, "LDD (HL),A"),
      Instruction::LDI_·HL·_A => write!(f, "LDI (HL),A"),
      Instruction::PUSH_rr(rr) => write!(f, "PUSH {}", rr),
      Instruction::NOP => write!(f, "NOP"),
      Instruction::XOR_r(r) => write!(f, "XOR {}", r),
      Instruction::Invalid => write!(f, "INVALID"),
    }
  }
}
