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

impl Default for Disassembler {
  fn default() -> Disassembler {
    Disassembler
  }
}

impl Disassembler {
  pub fn new() -> Disassembler {
    Disassembler::default()
  }

  pub fn at(&self, m: &Box<mem::Memory>, addr: u16) -> Result<(Instruction, u16), String> {
    let mut pc = 0u16;

    let op = try!(m.read_byte(addr + pc));
    pc += 1;

    if op == 0xCB {
      let op = try!(m.read_byte(addr + pc));
      pc += 1;
      match op {
        0x7C => Ok((Instruction::BIT_b_r(7, Reg::H), pc)),

        0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 => {
          let r = op & 0b111;
          Ok((Instruction::RL_r(Reg::from(r)), pc))
        }

        _ => {
          panic!("instruction_at: 0xCB instruction not implemented: 0x{:02x}",
                 op)
        }
      }
    } else {
      match op {
        0x86 => Ok((Instruction::ADD_A_·HL·, pc)),

        0xCD => {
          let nn = try!(m.read_word(addr + pc));
          pc += 2;
          Ok((Instruction::CALL_nn(nn), pc))
        }

        0xBE => Ok((Instruction::CP_·HL·, pc)),

        0xFE => {
          let n = try!(m.read_byte(addr + pc));
          pc += 1;
          Ok((Instruction::CP_n(n), pc))
        }

        0x5 | 0xd | 0x15 | 0x1d | 0x25 | 0x2d | 0x35 | 0x3d => {
          let r = op >> 3 & 0b11;
          Ok((Instruction::DEC_r(Reg::from(r)), pc))
        }

        0x4 | 0xc | 0x14 | 0x1c | 0x24 | 0x2c | 0x3c => {
          let r = op >> 3 & 0b111;
          Ok((Instruction::INC_r(Reg::from(r)), pc))
        }

        0x3 | 0x13 | 0x23 | 0x33 => {
          let ss = op >> 4 & 0b11;
          Ok((Instruction::INC_rr(Reg::from_pair(ss)), pc))
        }

        0xC3 => {
          let nn = try!(m.read_word(addr + pc));
          pc += 2;
          Ok((Instruction::JP_nn(nn), pc))
        }

        0x20 => {
          let e = try!(m.read_byte(addr + pc));
          pc += 1;
          Ok((Instruction::JR_cc_e(Flag::NZ, e as i8), pc))
        }
        0x28 => {
          let e = try!(m.read_byte(addr + pc));
          pc += 1;
          Ok((Instruction::JR_cc_e(Flag::Z, e as i8), pc))
        }
        0x30 => {
          let e = try!(m.read_byte(addr + pc));
          pc += 1;
          Ok((Instruction::JR_cc_e(Flag::NC, e as i8), pc))
        }
        0x38 => {
          let e = try!(m.read_byte(addr + pc));
          pc += 1;
          Ok((Instruction::JR_cc_e(Flag::C, e as i8), pc))
        }

        0x18 => {
          let e = try!(m.read_byte(addr + pc));
          pc += 1;
          Ok((Instruction::JR_e(e as i8), pc))
        }

        0xE2 => Ok((Instruction::LD_·0xFF00C·_A, pc)),
        0xE0 => {
          let n = try!(m.read_byte(addr + pc));
          pc += 1;
          Ok((Instruction::LD_·0xFF00n·_A(n), pc))
        }

        0x70...0x75 | 0x77 => {
          let r = op & 0b111;
          Ok((Instruction::LD_·HL·_r(Reg::from(r)), pc))
        }

        0xEA => {
          let nn = try!(m.read_word(addr + pc));
          pc += 2;
          Ok((Instruction::LD_·nn·_A(nn), pc))
        }

        0x1A => Ok((Instruction::LD_A_·DE·, pc)),

        0xF0 => {
          let n = try!(m.read_byte(addr + pc));
          pc += 1;
          Ok((Instruction::LD_A_·0xFF00n·(n), pc))
        }

        0x1 | 0x11 | 0x21 | 0x31 => {
          let r = op >> 4 & 0b11;
          let nn = try!(m.read_word(addr + pc));
          pc += 2;
          Ok((Instruction::LD_dd_nn(Reg::from_pair(r), nn), pc))
        }

        0x6 | 0xe | 0x16 | 0x1e | 0x26 | 0x2e | 0x3e => {
          let r = op >> 3 & 0b111;
          let n = try!(m.read_byte(addr + pc));
          pc += 1;
          Ok((Instruction::LD_r_n(Reg::from(r), n), pc))
        }

        0x40...0x45 | 0x47...0x4D | 0x4F...0x55 | 0x57...0x5D | 0x5F...0x65 | 0x67...0x6D |
        0x6F | 0x78...0x7D | 0x7F => {
          let r1 = op >> 3 & 0b111;
          let r2 = op & 0b111;
          Ok((Instruction::LD_r_r(Reg::from(r1), Reg::from(r2)), pc))
        }

        0x32 => Ok((Instruction::LDD_·HL·_A, pc)),
        0x22 => Ok((Instruction::LDI_·HL·_A, pc)),

        0xC1 => {
          let rr = op >> 4 & 0b11;
          Ok((Instruction::POP_rr(Reg::from_pair(rr)), pc))
        }

        0xc5 | 0xd5 | 0xe5 | 0xf5 => {
          let rr = op >> 4 & 0b11;
          Ok((Instruction::PUSH_rr(Reg::from_pair(rr)), pc))
        }

        0xC9 => Ok((Instruction::RET, pc)),

        0x17 => Ok((Instruction::RLA, pc)),

        0x90 | 0x91 | 0x92 | 0x93 | 0x94 | 0x95 | 0x96 | 0x97 => {
          let r = op & 0b111;
          Ok((Instruction::SUB_r(Reg::from(r)), pc))
        }

        0x00 => Ok((Instruction::NOP, pc)),
        0xAF => Ok((Instruction::XOR_r(Reg::A), pc)),

        _ => Ok((Instruction::Data(op), pc)),
        // _ => panic!("instruction_at: instruction not implemented: 0x{:02x}", op),
      }
    }
  }

  pub fn print_all(&self, m: &Box<mem::Memory>) {
    let mut pc = 0u16;

    while let Ok((ins, inc)) = self.at(m, pc) {
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
