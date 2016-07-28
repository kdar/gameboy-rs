use super::reg::Reg;
use super::flag::Flag;

macro_rules! bitmask {
  ($x:ident, $y:expr) => {

  }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
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

impl Instruction {
  pub fn from(op: u8) -> Instruction {
    match op {
      0x04 if op & 0b11000111 == 0b00000100 => {
        let r = op >> 3 & 0b111;
        Instruction::INC_r(Reg::from(r))
      }
      0x20 => Instruction::JR_cc_e(Flag::NZ),
      0x28 => Instruction::JR_cc_e(Flag::Z),
      0x30 => Instruction::JR_cc_e(Flag::NC),
      0x38 => Instruction::JR_cc_e(Flag::C),
      0xE2 => Instruction::LD_0xFF00C_A,
      0xE0 => Instruction::LD_0xFF00n_A,
      0x70 if op & 0b11111000 == 0b01110000 => {
        let r = op & 0b111;
        Instruction::LD_·HL·_r(Reg::from(r))
      }
      0x1A => Instruction::LD_A_·DE·,
      0x21 if op & 0b11001111 == 0b00000001 => {
        let r = op >> 4 & 0b11;
        Instruction::LD_dd_nn(Reg::from_pair(r))
      }
      0x06 if op & 0b11000111 == 0b00000110 => {
        let r = op >> 3 & 0b111;
        Instruction::LD_r_n(Reg::from(r))
      }
      0x32 => Instruction::LDD_·HL·_A,
      0x00 => Instruction::NOP,
      0xAF => Instruction::XOR_r(Reg::A),

      _ => panic!("instruction.from instruction not implemented: 0x{:02x}", op),
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
