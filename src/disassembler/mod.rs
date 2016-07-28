#![feature(non_ascii_idents)]

use gameboy::reg::Reg;
use gameboy::flag::Flag;

mod instruction;
use self::instruction::Instruction;

pub struct Disassembler {
  rom: Box<[u8]>,
}

impl Disassembler {
  pub fn new(rom: Box<[u8]>) -> Disassembler {
    Disassembler { rom: rom }
  }

  pub fn print_all(&self) {
    let mut pc = 0u16;

    while pc < self.rom.len() as u16 {
      let (ins, inc) = self.instruction_at(pc);
      println!("{:?}", ins);
      pc += inc;
    }
  }

  fn read_byte(&self, addr: u16) -> u8 {
    self.rom[addr as usize]
  }

  fn read_word(&self, addr: u16) -> u16 {
    let mut val: u16 = (self.read_byte(addr + 1) as u16) << 8;
    val |= self.read_byte(addr) as u16;
    val
  }

  pub fn instruction_at(&self, addr: u16) -> (Instruction, u16) {
    let mut pc = 0u16;

    let op = self.read_byte(addr + pc);
    pc += 1;

    if op == 0xCB {
      let op = self.read_byte(addr + pc);
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
        let nn = self.read_word(addr + pc);
        pc += 2;
        (Instruction::LD_dd_nn(Reg::from_pair(r), nn), pc)
      } else if op & 0b11000111 == 0b00000110 {
        let r = op >> 3 & 0b111;
        let n = self.read_byte(addr + pc);
        pc += 1;
        (Instruction::LD_r_n(Reg::from(r), n), pc)
      } else {
        match op {
          0x20 => {
            let e = self.read_byte(addr + pc);
            pc += 1;
            (Instruction::JR_cc_e(Flag::NZ, e as i8), pc)
          }
          0x28 => {
            let e = self.read_byte(addr + pc);
            pc += 1;
            (Instruction::JR_cc_e(Flag::Z, e as i8), pc)
          }
          0x30 => {
            let e = self.read_byte(addr + pc);
            pc += 1;
            (Instruction::JR_cc_e(Flag::NC, e as i8), pc)
          }
          0x38 => {
            let e = self.read_byte(addr + pc);
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
