use gameboy::reg::Reg;
use gameboy::flag::Flag;
use std::fmt;

#[allow(non_camel_case_types)]
pub enum Instruction {
  BIT_b_r(u8, Reg),
  INC_r(Reg),
  JR_cc_e(Flag, i8),
  LD_0xFF00C_A,
  LD_0xFF00n_A,
  LD_·HL·_r(Reg),
  LD_A_·DE·,
  LD_dd_nn(Reg, u16),
  LD_r_n(Reg, u8),
  LDD_·HL·_A,
  NOP,
  XOR_r(Reg),
}

impl fmt::Debug for Instruction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Instruction::BIT_b_r(b, r) => write!(f, "BIT {},{}", b, r),
      Instruction::INC_r(r) => write!(f, "INC {}", r),
      Instruction::JR_cc_e(cc, e) => write!(f, "JR {},{}", cc, e),
      Instruction::LD_0xFF00C_A => write!(f, "LD (0xFF00+C),A"),
      Instruction::LD_0xFF00n_A => write!(f, "LD (0xFF00+n),A"),
      Instruction::LD_·HL·_r(r) => write!(f, "LD (HL),{}", r),
      Instruction::LD_A_·DE· => write!(f, "LD A,(DE)"),
      Instruction::LD_dd_nn(dd, nn) => write!(f, "LD {},${:04x}", dd, nn),
      Instruction::LD_r_n(r, n) => write!(f, "LD {},${:02x}", r, n),
      Instruction::LDD_·HL·_A => write!(f, "LDD (HL),A"),
      Instruction::NOP => write!(f, "NOP"),
      Instruction::XOR_r(r) => write!(f, "XOR {}", r),
    }
  }
}
