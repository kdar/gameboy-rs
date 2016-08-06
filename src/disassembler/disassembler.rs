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

        0x38 | 0x39 | 0x3a | 0x3b | 0x3c | 0x3d | 0x3e | 0x3f => {
          let r = op & 0b111;
          Ok((Instruction::SRL_r(Reg::from(r)), pc))
        }

        _ => {
          panic!("instruction_at: 0xCB instruction not implemented: 0x{:02x}",
                 op)
        }
      }
    } else {
      match op {
        0x86 => Ok((Instruction::ADD_A_·HL·, pc)),

        0xc6 => {
          let n = try!(m.read_byte(addr + pc));
          pc += 1;
          Ok((Instruction::ADD_A_n(n), pc))
        }

        0x9 | 0x19 | 0x29 | 0x39 => {
          let rr = op >> 4 & 0b11;
          Ok((Instruction::ADD_HL_rr(Reg::from_pair(rr, false)), pc))
        }

        0xe6 => {
          let n = try!(m.read_byte(addr + pc));
          pc += 1;
          Ok((Instruction::AND_n(n), pc))
        }

        0xa0 | 0xa1 | 0xa2 | 0xa3 | 0xa4 | 0xa5 | 0xa6 | 0xa7 => {
          let r = op & 0b111;
          Ok((Instruction::AND_r(Reg::from(r)), pc))
        }

        0xc4 | 0xcc | 0xd4 | 0xdc | 0xe4 | 0xec | 0xf4 | 0xfc => {
          let cc = op >> 3 & 0b111;
          let nn = try!(m.read_word(addr + pc));
          pc += 2;
          Ok((Instruction::CALL_cc_nn(Flag::from(cc), nn), pc))
        }

        0xcd => {
          let nn = try!(m.read_word(addr + pc));
          pc += 2;
          Ok((Instruction::CALL_nn(nn), pc))
        }

        0xbe => Ok((Instruction::CP_·HL·, pc)),

        0xfe => {
          let n = try!(m.read_byte(addr + pc));
          pc += 1;
          Ok((Instruction::CP_n(n), pc))
        }

        0x5 | 0xd | 0x15 | 0x1d | 0x25 | 0x2d | 0x35 | 0x3d => {
          let r = op >> 3 & 0b11;
          Ok((Instruction::DEC_r(Reg::from(r)), pc))
        }

        0xf3 => Ok((Instruction::DI, pc)),

        0x4 | 0xc | 0x14 | 0x1c | 0x24 | 0x2c | 0x3c => {
          let r = op >> 3 & 0b111;
          Ok((Instruction::INC_r(Reg::from(r)), pc))
        }

        0x3 | 0x13 | 0x23 | 0x33 => {
          let ss = op >> 4 & 0b11;
          Ok((Instruction::INC_rr(Reg::from_pair(ss, false)), pc))
        }

        0xc3 => {
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

        0xe2 => Ok((Instruction::LD_·0xFF00C·_A, pc)),
        0xe0 => {
          let n = try!(m.read_byte(addr + pc));
          pc += 1;
          Ok((Instruction::LD_·0xFF00n·_A(n), pc))
        }

        0x02 => Ok((Instruction::LD_·BC·_A, pc)),
        0x12 => Ok((Instruction::LD_·DE·_A, pc)),

        0x70...0x75 | 0x77 => {
          let r = op & 0b111;
          Ok((Instruction::LD_·HL·_r(Reg::from(r)), pc))
        }

        0xea => {
          let nn = try!(m.read_word(addr + pc));
          pc += 2;
          Ok((Instruction::LD_·nn·_A(nn), pc))
        }

        0x08 => {
          let nn = try!(m.read_word(addr + pc));
          pc += 2;
          Ok((Instruction::LD_·nn·_SP(nn), pc))
        }

        0x0a => Ok((Instruction::LD_A_·BC·, pc)),

        0x1a => Ok((Instruction::LD_A_·DE·, pc)),

        0xfa => {
          let nn = try!(m.read_word(addr + pc));
          pc += 2;
          Ok((Instruction::LD_A_·nn·(nn), pc))
        }

        0xf0 => {
          let n = try!(m.read_byte(addr + pc));
          pc += 1;
          Ok((Instruction::LD_A_·0xFF00n·(n), pc))
        }

        0x1 | 0x11 | 0x21 | 0x31 => {
          let r = op >> 4 & 0b11;
          let nn = try!(m.read_word(addr + pc));
          pc += 2;
          Ok((Instruction::LD_dd_nn(Reg::from_pair(r, false), nn), pc))
        }

        0x46 | 0x4e | 0x56 | 0x5e | 0x66 | 0x6e | 0x76 | 0x7e => {
          let r = op >> 3 & 0b111;
          Ok((Instruction::LD_r_·HL·(Reg::from(r)), pc))
        }

        0x6 | 0xe | 0x16 | 0x1e | 0x26 | 0x2e | 0x3e => {
          let r = op >> 3 & 0b111;
          let n = try!(m.read_byte(addr + pc));
          pc += 1;
          Ok((Instruction::LD_r_n(Reg::from(r), n), pc))
        }

        0x40 | 0x41 | 0x42 | 0x43 | 0x44 | 0x45 | 0x47 | 0x48 | 0x49 | 0x4a | 0x4b | 0x4c |
        0x4d | 0x4f | 0x50 | 0x51 | 0x52 | 0x53 | 0x54 | 0x55 | 0x57 | 0x58 | 0x59 | 0x5a |
        0x5b | 0x5c | 0x5d | 0x5f | 0x60 | 0x61 | 0x62 | 0x63 | 0x64 | 0x65 | 0x67 | 0x68 |
        0x69 | 0x6a | 0x6b | 0x6c | 0x6d | 0x6f | 0x78 | 0x79 | 0x7a | 0x7b | 0x7c | 0x7d |
        0x7f => {
          let r1 = op >> 3 & 0b111;
          let r2 = op & 0b111;
          Ok((Instruction::LD_r_r(Reg::from(r1), Reg::from(r2)), pc))
        }

        0x2a => Ok((Instruction::LDI_A_·HL·, pc)),
        0x32 => Ok((Instruction::LDD_·HL·_A, pc)),
        0x22 => Ok((Instruction::LDI_·HL·_A, pc)),

        0xb0 | 0xb1 | 0xb2 | 0xb3 | 0xb4 | 0xb5 | 0xb6 | 0xb7 => {
          let r = op & 0b111;
          Ok((Instruction::OR_r(Reg::from(r)), pc))
        }

        0xc1 | 0xd1 | 0xe1 | 0xf1 => {
          let rr = op >> 4 & 0b11;
          Ok((Instruction::POP_rr(Reg::from_pair(rr, true)), pc))
        }

        0xc5 | 0xd5 | 0xe5 | 0xf5 => {
          let rr = op >> 4 & 0b11;
          Ok((Instruction::PUSH_rr(Reg::from_pair(rr, true)), pc))
        }

        0xc9 => Ok((Instruction::RET, pc)),

        0x17 => Ok((Instruction::RLA, pc)),
        0x07 => Ok((Instruction::RLCA, pc)),

        0xd6 => {
          let n = try!(m.read_byte(addr + pc));
          pc += 1;
          Ok((Instruction::SUB_n(n), pc))
        }

        0x90 | 0x91 | 0x92 | 0x93 | 0x94 | 0x95 | 0x96 | 0x97 => {
          let r = op & 0b111;
          Ok((Instruction::SUB_r(Reg::from(r)), pc))
        }

        0x00 => Ok((Instruction::NOP, pc)),
        0xa8 | 0xa9 | 0xaa | 0xab | 0xac | 0xad | 0xae | 0xaf => {
          let r = op & 0b111;
          Ok((Instruction::XOR_r(Reg::from(r)), pc))
        }

        _ => Ok((Instruction::Invalid(op), pc)),
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
