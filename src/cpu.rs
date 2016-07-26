use std::fmt;
use std::sync::Arc;
use std::cell::RefCell;

use super::mmu;

const NUM_GPR: usize = 8;
const REG_B: usize = 0b000;
const REG_C: usize = 0b001;
const REG_D: usize = 0b010;
const REG_E: usize = 0b011;
const REG_H: usize = 0b100;
const REG_L: usize = 0b101;
const REG_F: usize = 0b110;
const REG_A: usize = 0b111;

const REG_PAIR_BC: usize = 0b00;
const REG_PAIR_DE: usize = 0b01;
const REG_PAIR_HL: usize = 0b10;
const REG_PAIR_SP: usize = 0b11;

const FLAG_Z: usize = 0b10000000; // zero flag
const FLAG_N: usize = 0b01000000; // add/sub flag
const FLAG_H: usize = 0b00100000; // half carry flag
const FLAG_C: usize = 0b00010000; // carry flag

pub struct Cpu {
  // Contains the registers: A, F, B, C, D, E, H, L
  reg_gpr: [u8; NUM_GPR],

  reg_sp: u16, // Stack pointer
  reg_pc: u16, // Program counter

  cycles: u32, // Current number of clock cycles

  boot_rom: Box<[u8]>,
  cart_rom: Box<[u8]>,
  booting: bool,
}

impl fmt::Debug for Cpu {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    try!(write!(f, "\nCPU Registers:"));
    try!(write!(f, "\nA: 0x{:02x}", self.reg_gpr[REG_A]));
    try!(write!(f, "\nF: 0x{:02x}", self.reg_gpr[REG_F]));
    try!(write!(f, "\nB: 0x{:02x}", self.reg_gpr[REG_B]));
    try!(write!(f, "\nC: 0x{:02x}", self.reg_gpr[REG_C]));
    try!(write!(f, "\nD: 0x{:02x}", self.reg_gpr[REG_D]));
    try!(write!(f, "\nE: 0x{:02x}", self.reg_gpr[REG_E]));
    try!(write!(f, "\nH: 0x{:02x}", self.reg_gpr[REG_H]));
    try!(write!(f, "\nL: 0x{:02x}", self.reg_gpr[REG_L]));
    try!(write!(f, "\nSP: 0x{:04x}", self.reg_sp));
    try!(write!(f, "\nPC: 0x{:04x}", self.reg_pc));
    try!(write!(f, "\n\nCycles: {}", self.cycles));
    write!(f, "")
  }
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      reg_gpr: [0; NUM_GPR],

      reg_sp: 0,
      reg_pc: 0,
      cycles: 0,
      boot_rom: Box::new([]),
      cart_rom: Box::new([]),
      booting: false,
    }
  }

  pub fn set_boot_rom(&mut self, rom: Box<[u8]>) {
    self.booting = true;
    self.boot_rom = rom;
  }

  pub fn set_cart_rom(&mut self, rom: Box<[u8]>) {
    self.cart_rom = rom;
  }

  pub fn read_word(&self, addr: u16) -> u16 {
    let mut val: u16 = (self.read_byte(addr + 1) as u16) << 8;
    val |= self.read_byte(addr) as u16;
    val
  }

  pub fn read_byte(&self, addr: u16) -> u8 {
    let mapped = mmu::memory_map(addr);
    match mapped {
      mmu::Addr::Rom00(offset) => {
        if self.booting {
          self.boot_rom[offset as usize]
        } else {
          self.cart_rom[offset as usize]
        }
      }
      mmu::Addr::Rom01(offset) => panic!("not implemented: {:?}", mapped),
      mmu::Addr::VideoRam(offset) => panic!("not implemented: {:?}", mapped),
      mmu::Addr::ExternalRam(offset) => panic!("not implemented: {:?}", mapped),
      mmu::Addr::WorkRam00(offset) => panic!("not implemented: {:?}", mapped),
      mmu::Addr::WorkRam01(offset) => panic!("not implemented: {:?}", mapped),
      mmu::Addr::SpriteTable(offset) => panic!("not implemented: {:?}", mapped),
      mmu::Addr::IoPorts(offset) => panic!("not implemented: {:?}", mapped),
      mmu::Addr::HighRam(offset) => panic!("not implemented: {:?}", mapped),
      mmu::Addr::InterruptRegister => panic!("not implemented: {:?}", mapped),
    }
  }

  // pub fn reset(&mut self) {
  //   self.reg_gpr = [0; NUM_GPR];
  //   self.reg_sp = 0;
  //   self.reg_pc = 0;
  //   self.cycles = 0;
  // }

  pub fn step(&mut self) {
    let opcode = self.read_pc_byte();
    self.execute(opcode);

    println!("{:?}", self);
  }

  fn execute(&mut self, opcode: u8) {
    let cycles = if opcode == 0xCB {
      let opcode = self.read_pc_byte();
      match opcode {
        _ => panic!("CB instruction not implemented: 0x{:02x}", opcode),
      }
    } else {
      match opcode {
        0x00 => self.inst_nop(),
        0x21 => self.inst_ld_hl_nn(),
        0x31 => self.inst_ld_sp_nn(),
        0xAF => self.inst_xor_a(),
        _ => panic!("instruction not implemented: 0x{:02x}", opcode),
      }
    };

    self.cycles += cycles;
  }

  // 0x00
  // NOP
  fn inst_nop(&self) -> u32 {
    4
  }

  // 0x21
  // LD HL,nn
  fn inst_ld_hl_nn(&mut self) -> u32 {
    let h = self.read_pc_byte();
    let l = self.read_pc_byte();
    self.write_reg_gpr(REG_H, h);
    self.write_reg_gpr(REG_L, l);
    12
  }

  // 0x31
  // LD SP,nn
  // Page 120
  fn inst_ld_sp_nn(&mut self) -> u32 {
    self.reg_sp = self.read_pc_word();
    12
  }

  fn xor(&mut self, register: usize) -> u32 {
    let register = self.read_reg_gpr(register);
    let mut accumulator = self.read_reg_gpr(REG_A);
    accumulator = accumulator ^ register;
    self.write_reg_gpr(REG_A, accumulator);

    if accumulator == 0 {
      self.write_flag(FLAG_Z, true);
    } else {
      self.write_flag(FLAG_Z, false);
    }

    self.write_flag(FLAG_N, false);
    self.write_flag(FLAG_C, false);

    4
  }

  // 0xAF
  // XOR A
  fn inst_xor_a(&mut self) -> u32 {
    self.xor(REG_A)
  }

  pub fn read_reg_gpr(&self, register: usize) -> u8 {
    match register {
      0b000...0b111 => {
        return self.reg_gpr[register as usize];
      } // A
      _ => panic!("get_byte_register unknown register: {}", register),
    }
  }

  pub fn write_reg_gpr(&mut self, register: usize, value: u8) {
    self.reg_gpr[register as usize] = value;
  }

  fn read_pc_byte(&mut self) -> u8 {
    let d = self.read_byte(self.reg_pc);
    self.reg_pc += 1;
    d
  }

  fn read_pc_word(&mut self) -> u16 {
    let d = self.read_word(self.reg_pc);
    self.reg_pc += 2;
    d
  }

  fn write_flag(&mut self, flag: usize, value: bool) {
    let mut d = self.read_reg_gpr(REG_F);
    if value {
      d |= flag as u8;
    } else {
      d &= !flag as u8;
    }
    self.write_reg_gpr(REG_F, d);
  }

  fn read_flag(&self, flag: usize) -> bool {
    let mut d = self.read_reg_gpr(REG_F);
    d & flag as u8 > 0
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use super::{REG_A, FLAG_Z, FLAG_N, FLAG_H, FLAG_C};

  // #[test]
  // fn test_inst_ld_dd_nn() {
  //   let mut test_cpu = Cpu::new();
  //
  //   assert_eq!(test_cpu.cycles, 0);
  //   assert_eq!(test_cpu.read_reg_gpr(REG_A), 0b00000000);
  //
  //   test_cpu.execute(0xAF);
  //
  //   assert_eq!(test_cpu.read_reg_gpr(REG_A), 0b0);
  //   assert_eq!(test_cpu.cycles, 4);
  //
  //   assert_eq!(test_cpu.read_flag(FLAG_Z), true);
  //   assert_eq!(test_cpu.read_flag(FLAG_N), false);
  //   assert_eq!(test_cpu.read_flag(FLAG_H), false);
  //   assert_eq!(test_cpu.read_flag(FLAG_C), false);
  // }
  //
  // #[test]
  // fn test_inst_nop() {
  //   let mut test_cpu = Cpu::new();
  //
  //   assert_eq!(test_cpu.cycles, 0);
  //
  //
  //   test_cpu.execute(0x00);
  //
  //   assert_eq!(test_cpu.cycles, 4);
  //
  //   assert_eq!(test_cpu.read_flag(FLAG_Z), true);
  //   assert_eq!(test_cpu.read_flag(FLAG_N), false);
  //   assert_eq!(test_cpu.read_flag(FLAG_H), false);
  //   assert_eq!(test_cpu.read_flag(FLAG_C), false);
  // }
  // #[test]
  // fn test_inst_ld_hl_nn() {
  //
  // }
  // #[test]
  // fn test_inst_ld_sp_nn() {
  //
  // }
  // #[test]
  // fn test_inst_xor_a() {
  //
  // }
  //
  // #[test]
  // fn test_inst_xor_s() {
  //   let mut test_cpu = Cpu::new();
  //
  //   test_cpu.write_reg_gpr(REG_A, 0b11010110);
  //   assert_eq!(test_cpu.cycles, 0);
  //   assert_eq!(test_cpu.read_reg_gpr(REG_A), 0b11010110);
  //
  //   test_cpu.execute(0xAF);
  //
  //   assert_eq!(test_cpu.read_reg_gpr(REG_A), 0b0);
  //   assert_eq!(test_cpu.cycles, 4);
  //
  //   assert_eq!(test_cpu.read_flag(FLAG_Z), true);
  //   assert_eq!(test_cpu.read_flag(FLAG_N), false);
  //   assert_eq!(test_cpu.read_flag(FLAG_H), false);
  //   assert_eq!(test_cpu.read_flag(FLAG_C), false);
  // }
}
