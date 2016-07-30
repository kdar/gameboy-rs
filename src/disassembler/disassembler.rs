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
      // Invalid,
      // BIT_b_r(u8, Reg),
      // CALL_nn(u16),
      // CP_n(u8),
      // INC_r(Reg),
      // INC_rr(Reg),
      // JR_cc_e(Flag, i8),
      // LD_0xFF00C_A, // Moved: RET PO -> LD (FF00+n),A
      // LD_0xFF00n_A, // Moved: JP PO,nn -> LD (FF00+C),A
      // LD_·HL·_r(Reg),
      // LD_A_·DE·,
      // LD_dd_nn(Reg, u16),
      // LD_r_n(Reg, u8),
      // LD_r_r(Reg, Reg),
      // LDD_·HL·_A, // Moved: LD (nn),A -> LDD (HL),A
      // LDI_·HL·_A, // Moved: LD (nn),HL -> LDI (HL),A
      // NOP,
      // XOR_r(Reg),

      match op {
        0xCD => {
          let nn = try_o!(m.read_word(addr + pc));
          pc += 2;
          Some((Instruction::CALL_nn(nn), pc))
        }

        0xFE => {
          let n = try_o!(m.read_byte(addr + pc));
          pc += 1;
          Some((Instruction::CP_n(n), pc))
        }

        0x4 | 0xc | 0x14 | 0x1c | 0x24 | 0x2c | 0x3c => {
          let r = op >> 3 & 0b111;
          Some((Instruction::INC_r(Reg::from(r)), pc))
        }

        0x3 | 0x13 | 0x23 | 0x33 => {
          let ss = op >> 4 & 0b11;
          Some((Instruction::INC_rr(Reg::from_pair(ss)), pc))
        }

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

        0x70...0x75 | 0x77 => {
          let r = op & 0b111;
          Some((Instruction::LD_·HL·_r(Reg::from(r)), pc))
        }

        0x1A => Some((Instruction::LD_A_·DE·, pc)),

        0x1 | 0x11 | 0x21 | 0x31 => {
          let r = op >> 4 & 0b11;
          let nn = try_o!(m.read_word(addr + pc));
          pc += 2;
          Some((Instruction::LD_dd_nn(Reg::from_pair(r), nn), pc))
        }

        0x6 | 0xe | 0x16 | 0x1e | 0x26 | 0x2e | 0x3e => {
          let r = op >> 3 & 0b111;
          let n = try_o!(m.read_byte(addr + pc));
          pc += 1;
          Some((Instruction::LD_r_n(Reg::from(r), n), pc))
        }

        0x40...0x45 | 0x47...0x4D | 0x4F...0x55 | 0x57...0x5D | 0x5F...0x65 | 0x67...0x6D |
        0x6F | 0x78...0x7D | 0x7F => {
          let r1 = op >> 3 & 0b111;
          let r2 = op & 0b111;
          Some((Instruction::LD_r_r(Reg::from(r1), Reg::from(r2)), pc))
        }

        0x32 => Some((Instruction::LDD_·HL·_A, pc)),
        0x22 => Some((Instruction::LDI_·HL·_A, pc)),
        0x00 => Some((Instruction::NOP, pc)),
        0xAF => Some((Instruction::XOR_r(Reg::A), pc)),

        _ => panic!("instruction_at: instruction not implemented: 0x{:02x}", op),
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
