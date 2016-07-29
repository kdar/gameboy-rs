use std::io::Write;
use super::super::Reg;
use super::super::Flag;

use super::instruction::Instruction;
use super::super::mem;

macro_rules! try_o {
  ($expr:expr) => (match $expr {
    Some(val) => val,
    None => {
      return None;
    },
  })
}

fn to_hex(v: &[u8]) -> String {
  let mut f = vec![];
  for val in v {
    write!(f, "{:x}", val).unwrap();
  }

  String::from_utf8(f).unwrap()
}

pub struct Disassembler;

impl Disassembler {
  pub fn new() -> Disassembler {
    Disassembler
  }

  pub fn at(&self, m: &Box<mem::Memory>, addr: u16) -> Option<(Instruction, u16)> {
    let mut pc = 0u16;

    let op = try_o!(m.read_byte(addr + pc));
    pc += 1;

    if op == 0xCB {
      let op = try_o!(m.read_byte(addr + pc));
      pc += 1;
      match op {
        0x7C => Some((Instruction::BIT_b_r(7, Reg::H), pc)),
        _ => {
          panic!("instruction_at: 0xCB instruction not implemented: 0x{:02x}",
                 op)
        }
      }
    } else {
      if op & 0b11000111 == 0b00000100 {
        let r = op >> 3 & 0b111;
        Some((Instruction::INC_r(Reg::from(r)), pc))
      } else if op & 0b11111000 == 0b01110000 {
        let r = op & 0b111;
        Some((Instruction::LD_·HL·_r(Reg::from(r)), pc))
      } else if op & 0b11001111 == 0b00000001 {
        let r = op >> 4 & 0b11;
        let nn = try_o!(m.read_word(addr + pc));
        pc += 2;
        Some((Instruction::LD_dd_nn(Reg::from_pair(r), nn), pc))
      } else if op & 0b11000111 == 0b00000110 {
        let r = op >> 3 & 0b111;
        let n = try_o!(m.read_byte(addr + pc));
        pc += 1;
        Some((Instruction::LD_r_n(Reg::from(r), n), pc))
      } else if op & 0b11000000 == 0b01000000 {
        let r1 = op >> 3 & 0b111;
        let r2 = op & 0b111;
        Some((Instruction::LD_r_r(Reg::from(r1), Reg::from(r2)), pc))
      } else {
        match op {
          0x20 => {
            let e = try_o!(m.read_byte(addr + pc));

            pc += 1;
            Some((Instruction::JR_cc_e(Flag::NZ, e as i8), pc))
          }
          0x28 => {
            let e = try_o!(m.read_byte(addr + pc));
            pc += 1;
            Some((Instruction::JR_cc_e(Flag::Z, e as i8), pc))
          }
          0x30 => {
            let e = try_o!(m.read_byte(addr + pc));
            pc += 1;
            Some((Instruction::JR_cc_e(Flag::NC, e as i8), pc))
          }
          0x38 => {
            let e = try_o!(m.read_byte(addr + pc));
            pc += 1;
            Some((Instruction::JR_cc_e(Flag::C, e as i8), pc))
          }
          0xE2 => Some((Instruction::LD_0xFF00C_A, pc)),
          0xE0 => Some((Instruction::LD_0xFF00n_A, pc)),
          0x1A => Some((Instruction::LD_A_·DE·, pc)),
          0x32 => Some((Instruction::LDD_·HL·_A, pc)),
          0x00 => Some((Instruction::NOP, pc)),
          0xAF => Some((Instruction::XOR_r(Reg::A), pc)),

          _ => panic!("instruction_at: instruction not implemented: 0x{:02x}", op),
        }
      }
    }
  }

  pub fn print_all(&self, m: &Box<mem::Memory>) {
    let mut pc = 0u16;

    while let Some((ins, inc)) = self.at(m, pc) {
      // let hex = to_hex(&rom[(pc as usize)..(pc as usize) + inc as usize]);
      let hex = to_hex(m.read_vec(pc, inc).unwrap().as_slice());
      match ins {
        Instruction::JR_cc_e(_, e) => {
          println!("{:04x} {:12} {:20} ; Addr: {}",
                   pc,
                   hex,
                   format!("{:?}", ins),
                   (pc as i16) + (e as i16) + inc as i16)
        }
        _ => println!("{:04x} {:12} {:12?}", pc, hex, ins),
      }
      pc += inc;
    }
  }
}
