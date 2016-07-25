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
    try!(write!(f, "\nAF: {}", self.reg_af));
    try!(write!(f, "\nBC: {}", self.reg_bc));
    try!(write!(f, "\nDE: {}", self.reg_de));
    try!(write!(f, "\nHL: {}", self.reg_hl));
    try!(write!(f, "\nSP: {}", self.reg_sp));
    try!(write!(f, "\nPC: {}", self.reg_pc));
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
    println!("{:?}", self);

    cycles += self.execute_instruction(opcode);
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
        0x31 => self.inst_ld_nn(),
        _ => panic!("instruction not implemented: {}", opcode),
      }
    }
  }

  fn inst_nop(&self) -> usize {
    4
  }

  fn inst_ld_nn(&mut self) -> usize {
    let nn = self.read_pc_byte();
    20
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
