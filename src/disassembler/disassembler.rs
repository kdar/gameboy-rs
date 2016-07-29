use std::io::Write;
use super::super::Reg;
use super::super::Flag;

use super::instruction::Instruction;

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

  pub fn print_all(&self, rom: &[u8]) {
    let mut pc = 0u16;

    while pc < rom.len() as u16 {
      let (ins, inc) = self.instruction_at(pc, rom);
      let hex = to_hex(&rom[(pc as usize)..(pc as usize) + inc as usize]);
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

  fn read_byte(&self, addr: u16, rom: &[u8]) -> u8 {
    rom[addr as usize]
  }

  fn read_word(&self, addr: u16, rom: &[u8]) -> u16 {
    let mut val: u16 = (self.read_byte(addr + 1, rom) as u16) << 8;
    val |= self.read_byte(addr, rom) as u16;
    val
  }

  pub fn instruction_at(&self, addr: u16, rom: &[u8]) -> (Instruction, u16) {
    let mut pc = 0u16;

    let op = self.read_byte(addr + pc, rom);
    pc += 1;

    if op == 0xCB {
      let op = self.read_byte(addr + pc, rom);
      pc += 1;
      match op {
        0x7C => (Instruction::BIT_b_r(7, Reg::H), pc),
        _ => {
          panic!("instruction_at: 0xCB instruction not implemented: 0x{:02x}",
                 op)
        }
      }
    } else {
      if op & 0b11000111 == 0b00000100 {
        let r = op >> 3 & 0b111;
        (Instruction::INC_r(Reg::from(r)), pc)
      } else if op & 0b11111000 == 0b01110000 {
        let r = op & 0b111;
        (Instruction::LD_·HL·_r(Reg::from(r)), pc)
      } else if op & 0b11001111 == 0b00000001 {
        let r = op >> 4 & 0b11;
        let nn = self.read_word(addr + pc, rom);
        pc += 2;
        (Instruction::LD_dd_nn(Reg::from_pair(r), nn), pc)
      } else if op & 0b11000111 == 0b00000110 {
        let r = op >> 3 & 0b111;
        let n = self.read_byte(addr + pc, rom);
        pc += 1;
        (Instruction::LD_r_n(Reg::from(r), n), pc)
      } else if op & 0b11000000 == 0b01000000 {
        let r1 = op >> 3 & 0b111;
        let r2 = op & 0b111;
        (Instruction::LD_r_r(Reg::from(r1), Reg::from(r2)), pc)
      } else {
        match op {
          0x20 => {
            let e = self.read_byte(addr + pc, rom);
            pc += 1;
            (Instruction::JR_cc_e(Flag::NZ, e as i8), pc)
          }
          0x28 => {
            let e = self.read_byte(addr + pc, rom);
            pc += 1;
            (Instruction::JR_cc_e(Flag::Z, e as i8), pc)
          }
          0x30 => {
            let e = self.read_byte(addr + pc, rom);
            pc += 1;
            (Instruction::JR_cc_e(Flag::NC, e as i8), pc)
          }
          0x38 => {
            let e = self.read_byte(addr + pc, rom);
            pc += 1;
            (Instruction::JR_cc_e(Flag::C, e as i8), pc)
          }
          0xE2 => (Instruction::LD_0xFF00C_A, pc),
          0xE0 => (Instruction::LD_0xFF00n_A, pc),
          0x1A => (Instruction::LD_A_·DE·, pc),
          0x32 => (Instruction::LDD_·HL·_A, pc),
          0x00 => (Instruction::NOP, pc),
          0xAF => (Instruction::XOR_r(Reg::A), pc),

          _ => panic!("instruction_at: instruction not implemented: 0x{:02x}", op),
        }
      }
    }
  }
}
