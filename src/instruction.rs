use super::reg::Reg;
use super::flag::Flag;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Instruction {
  NOP,
  LD_hl_nn,
  LD_sp_nn,
  LDD_hl_a,
  XOR_r(Reg),
  JR_cc_e(Flag),

  BIT_b_r(u8, Reg),
}

impl Instruction {
  pub fn from(op: u8) -> Instruction {
    match op {
      0x00 => Instruction::NOP,
      0x20 => Instruction::JR_cc_e(Flag::NZ),
      0x21 => Instruction::LD_hl_nn,
      0x28 => Instruction::JR_cc_e(Flag::Z),
      0x30 => Instruction::JR_cc_e(Flag::NC),
      0x31 => Instruction::LD_sp_nn,
      0x32 => Instruction::LDD_hl_a,
      0x38 => Instruction::JR_cc_e(Flag::C),
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
