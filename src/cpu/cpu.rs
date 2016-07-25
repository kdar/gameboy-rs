use std::fmt;
use std::sync::Arc;
use std::cell::RefCell;

use super::super::system;

pub struct Cpu {
  reg_af: u16, // Accumulator and flags
  reg_bc: u16, // General purpose
  reg_de: u16, // General purpose
  reg_hl: u16, // General purpose

  reg_sp: u16, // Stack pointer
  reg_pc: u16, // Program counter

  cycles: u32, // Current number of clock cycles

  system: Arc<RefCell<system::System>>,
}

impl fmt::Debug for Cpu {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    try!(write!(f, "\nCPU Registers:"));
    try!(write!(f, "\nAF: 0x{:04x}", self.reg_af));
    try!(write!(f, "\nBC: 0x{:04x}", self.reg_bc));
    try!(write!(f, "\nDE: 0x{:04x}", self.reg_de));
    try!(write!(f, "\nHL: 0x{:04x}", self.reg_hl));
    try!(write!(f, "\nSP: 0x{:04x}", self.reg_sp));
    try!(write!(f, "\nPC: 0x{:04x}", self.reg_pc));
    try!(write!(f, "\n\nCycles: {}", self.cycles));
    write!(f, "")
  }
}

impl Cpu {
  pub fn new(system: Arc<RefCell<system::System>>) -> Cpu {
    Cpu {
      reg_af: 0,
      reg_bc: 0,
      reg_de: 0,
      reg_hl: 0,
      reg_sp: 0,
      reg_pc: 0,
      cycles: 0,
      system: system,
    }
  }

  pub fn step(&mut self) {
    let mut cycles = 0;

    let opcode = self.read_pc_byte();

    cycles += self.execute_instruction(opcode);

    println!("{:?}", self);
  }

  fn execute_instruction(&mut self, opcode: u8) -> usize {
    if opcode == 0xCB {
      let opcode = self.read_pc_byte();
      match opcode {
        _ => panic!("CB instruction not implemented: {}", opcode),
      }
    } else {
      match opcode {
        0x00 => self.inst_nop(),
        0x31 => self.inst_ld_dd_nn(opcode),
        _ => panic!("instruction not implemented: {}", opcode),
      }
    }
  }

  fn inst_nop(&self) -> usize {
    4
  }

  // LD dd,nn
  // 00dd0001 nnnnnnnn nnnnnnnn
  // Page 120
  fn inst_ld_dd_nn(&mut self, opcode: u8) -> usize {
    let register = opcode >> 4 & 0x3;
    match register {
      0x3 => {
        self.reg_sp = self.read_pc_word();
      }
      _ => {
        panic!("ld_dd_nn unknown register: {}", register);
      }
    }
    12
  }

  fn read_pc_byte(&mut self) -> u8 {
    let d = self.system.borrow_mut().read_byte(self.reg_pc);
    self.reg_pc += 1;
    d
  }

  fn read_pc_word(&mut self) -> u16 {
    let d = self.system.borrow_mut().read_word(self.reg_pc);
    self.reg_pc += 2;
    d
  }
}
