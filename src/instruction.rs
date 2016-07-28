use super::reg::Reg;
use super::flag::Flag;
use std::fmt;

#[allow(non_camel_case_types)]
pub enum Instruction {
  BIT_b_r(u8, Reg),
  INC_r(Reg),
  JR_cc_e(Flag),
  LD_0xFF00C_A,
  LD_0xFF00n_A,
  LD_·HL·_r(Reg),
  LD_A_·DE·,
  LD_dd_nn(Reg),
  LD_r_n(Reg),
  LDD_·HL·_A,
  NOP,
  XOR_r(Reg),
}

impl fmt::Debug for Instruction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Instruction::BIT_b_r(b, r) => write!(f, "BIT {},{}", b, r),
      Instruction::INC_r(r) => write!(f, "INC {}", r),
      Instruction::JR_cc_e(cc) => write!(f, "JR {},e", cc),
      Instruction::LD_0xFF00C_A => write!(f, "LD (0xFF00+C),A"),
      Instruction::LD_0xFF00n_A => write!(f, "LD (0xFF00+n),A"),
      Instruction::LD_·HL·_r(r) => write!(f, "LD (HL),{}", r),
      Instruction::LD_A_·DE· => write!(f, "LD A,(DE)"),
      Instruction::LD_dd_nn(dd) => write!(f, "LD {},nn", dd),
      Instruction::LD_r_n(r) => write!(f, "LD {},n", r),
      Instruction::LDD_·HL·_A => write!(f, "LDD (HL),A"),
      Instruction::NOP => write!(f, "NOP"),
      Instruction::XOR_r(r) => write!(f, "XOR {}", r),
    }
  }
}


impl Instruction {
  pub fn from(op: u8) -> Instruction {
    if op & 0b11000111 == 0b00000100 {
      let r = op >> 3 & 0b111;
      Instruction::INC_r(Reg::from(r))
    } else if op & 0b11111000 == 0b01110000 {
      let r = op & 0b111;
      Instruction::LD_·HL·_r(Reg::from(r))
    } else if op & 0b11001111 == 0b00000001 {
      let r = op >> 4 & 0b11;
      Instruction::LD_dd_nn(Reg::from_pair(r))
    } else if op & 0b11000111 == 0b00000110 {
      let r = op >> 3 & 0b111;
      Instruction::LD_r_n(Reg::from(r))
    } else {
      match op {
        0x20 => Instruction::JR_cc_e(Flag::NZ),
        0x28 => Instruction::JR_cc_e(Flag::Z),
        0x30 => Instruction::JR_cc_e(Flag::NC),
        0x38 => Instruction::JR_cc_e(Flag::C),
        0xE2 => Instruction::LD_0xFF00C_A,
        0xE0 => Instruction::LD_0xFF00n_A,
        0x1A => Instruction::LD_A_·DE·,
        0x32 => Instruction::LDD_·HL·_A,
        0x00 => Instruction::NOP,
        0xAF => Instruction::XOR_r(Reg::A),

        _ => panic!("instruction.from instruction not implemented: 0x{:02x}", op),
      }
    }
  }

  pub fn from_cb(op: u8) -> Instruction {
    match op {
      0x7C => Instruction::BIT_b_r(7, Reg::H),
      _ => {
        panic!("instruction.from_cb instruction not implemented: 0x{:02x}",
               op)
      }
    }
  }
}
