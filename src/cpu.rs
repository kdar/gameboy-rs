use std::fmt;
use std::default::Default;
use std::cmp::PartialEq;

use super::operand::Operand;
use super::disassembler::Instruction;
use super::disassembler::Disassembler;
use super::system::{System, SystemCtrl};

#[inline]
fn high_byte(value: u16) -> u8 {
  (value >> 8) as u8
}

#[inline]
fn low_byte(value: u16) -> u8 {
  value as u8 & 0b11111111
}

#[derive(Debug, Clone, Copy)]
pub enum Flag {
  Z, // zero flag
  N, // add/sub flag
  H, // half carry flag
  C, // carry flag
  NZ, // non-zero (uses zero flag)
  NC, // non-carry (uses carry flag)
}

#[derive(Debug, Clone, Copy)]
pub enum Reg {
  B,
  C,
  D,
  E,
  H,
  L,
  F,
  A,
  BC,
  DE,
  HL,
  AF,
  SP,
  PC,
}

#[derive(PartialEq)]
enum ImeState {
  Enabling,
  Enabled,
  Disabled,
}

#[derive(PartialEq)]
struct Ime {
  state: ImeState,
}

impl Default for Ime {
  fn default() -> Ime {
    Ime { state: ImeState::Disabled }
  }
}

impl Ime {
  fn step(&mut self) {
    if self.state == ImeState::Enabling {
      self.state = ImeState::Enabled;
    }
  }

  fn set_enabling(&mut self) {
    self.state = ImeState::Enabling;
  }

  fn set_enabled(&mut self, v: bool) {
    if v {
      self.state = ImeState::Enabled;
    } else {
      self.state = ImeState::Disabled;
    }
  }

  fn enabled(&self) -> bool {
    self.state == ImeState::Enabled
  }
}

pub struct Cpu {
  reg_af: u16, // Accumulator and flags
  reg_bc: u16, // B and C general purpose
  reg_de: u16, // D and E general purpose
  reg_hl: u16, // H and L general purpose

  reg_sp: u16, // Stack pointer
  reg_pc: u16, // Program counter

  machine_cycles: u32, // 1 machine cycle = 4 clock cycles
  ime: Ime,
  halt: bool,

  pub system: Box<SystemCtrl + Send>,
  disasm: Disassembler,
}

impl PartialEq for Cpu {
  fn eq(&self, x: &Cpu) -> bool {
    self.reg_af == x.reg_af && self.reg_bc == x.reg_bc && self.reg_de == x.reg_de &&
    self.reg_hl == x.reg_hl && self.reg_sp == x.reg_sp && self.reg_pc == x.reg_pc &&
    self.machine_cycles == x.machine_cycles
  }
}

impl Default for Cpu {
  fn default() -> Cpu {
    Cpu {
      reg_af: 0,
      reg_bc: 0,
      reg_de: 0,
      reg_hl: 0,
      reg_sp: 0,
      reg_pc: 0,
      machine_cycles: 0,
      ime: Ime::default(),
      halt: false,
      system: Box::new(System::default()),
      disasm: Disassembler::new(),
    }
  }
}

impl fmt::Debug for Cpu {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    try!(write!(f, "\nAF:      {0:#06x} [{0:016b}]", self.reg_af));
    try!(write!(f, "\nBC:      {0:#06x} [{0:016b}]", self.reg_bc));
    try!(write!(f, "\nDE:      {0:#06x} [{0:016b}]", self.reg_de));
    try!(write!(f, "\nHL:      {0:#06x} [{0:016b}]", self.reg_hl));
    try!(write!(f, "\nSP:      {0:#06x} [{0:016b}]", self.reg_sp));
    try!(write!(f, "\nPC:      {0:#06x} [{0:016b}]", self.reg_pc));
    try!(write!(f, "\nFlags:   "));
    if self.read_flag(Flag::Z) {
      try!(write!(f, "Z"));
    }
    if self.read_flag(Flag::N) {
      try!(write!(f, "N"));
    }
    if self.read_flag(Flag::H) {
      try!(write!(f, "H"));
    }
    if self.read_flag(Flag::C) {
      try!(write!(f, "C"));
    }
    try!(write!(f, "\nMachine cycles: {}", self.machine_cycles));
    write!(f, "\n")
  }
}

impl Cpu {
  pub fn new(system: Box<SystemCtrl + Send>) -> Cpu {
    let mut c = Cpu::default();
    c.system = system;
    c
  }

  // Sets the system state as if the bootloader was run.
  pub fn bootstrap(&mut self) {
    // set booting flag to false
    self.system.write_u8(0xff50, 1).unwrap();

    self.reg_af = 0x01b0;
    self.reg_bc = 0x0013;
    self.reg_de = 0x00d8;
    self.reg_hl = 0x014d;
    self.reg_sp = 0xfffe;
    self.reg_pc = 0x100;

    self.write_u8(0xff10, 0x80);
    self.write_u8(0xff11, 0xbf);
    self.write_u8(0xff12, 0xf3);
    self.write_u8(0xff14, 0xbf);
    self.write_u8(0xff16, 0x3f);
    self.write_u8(0xff19, 0xbf);
    self.write_u8(0xff1a, 0x7f);
    self.write_u8(0xff1b, 0xff);
    self.write_u8(0xff1c, 0x9f);
    self.write_u8(0xff1e, 0xbf);
    self.write_u8(0xff20, 0xff);
    self.write_u8(0xff23, 0xbf);
    self.write_u8(0xff24, 0x77);
    self.write_u8(0xff25, 0xf3);
    self.write_u8(0xff26, 0xf1);
    self.write_u8(0xff40, 0x91);
    self.write_u8(0xff47, 0xfc);
    self.write_u8(0xff48, 0xff);
    self.write_u8(0xff49, 0xff);
  }

  fn mcycle(&mut self, machine_cycles: u32) {
    // step the system by the amount of clock cycles
    for _ in 0..(machine_cycles * 4) {
      self.system.step();
    }

    self.machine_cycles = self.machine_cycles.wrapping_add(machine_cycles);
  }

  pub fn read_operand_u8(&mut self, operand: Operand) -> u8 {
    match operand {
      Operand::RegA => high_byte(self.reg_af),
      Operand::RegF => low_byte(self.reg_af),
      Operand::RegB => high_byte(self.reg_bc),
      Operand::RegC => low_byte(self.reg_bc),
      Operand::RegD => high_byte(self.reg_de),
      Operand::RegE => low_byte(self.reg_de),
      Operand::RegH => high_byte(self.reg_hl),
      Operand::RegL => low_byte(self.reg_hl),
      Operand::AddrBC => {
        let bc = self.reg_bc;
        self.read_u8(bc)
      }
      Operand::AddrDE => {
        let de = self.reg_de;
        self.read_u8(de)
      }
      Operand::AddrHL => {
        let hl = self.reg_hl;
        self.read_u8(hl)
      }
      Operand::AddrSP => {
        let sp = self.reg_sp;
        self.read_u8(sp)
      }
      Operand::FlagZ => if 0b10000000 & self.reg_af != 0 { 1 } else { 0 },
      Operand::FlagN => if 0b01000000 & self.reg_af != 0 { 1 } else { 0 },
      Operand::FlagH => if 0b00100000 & self.reg_af != 0 { 1 } else { 0 },
      Operand::FlagC => if 0b00010000 & self.reg_af != 0 { 1 } else { 0 },
      Operand::FlagNZ => if 0b10000000 & self.reg_af == 0 { 1 } else { 0 },
      Operand::FlagNC => if 0b00010000 & self.reg_af == 0 { 1 } else { 0 },
      Operand::Imm8(i) => i,
      Operand::AddrImm16(i) => self.read_u8(i),
      Operand::AddrIoPortC => {
        let c = self.read_reg_u8(Reg::C);
        self.read_u8(0xff00 + c as u16)
      }
      _ => panic!("cpu.read_operand_u8: unrecognized operand: {:?}", operand),
    }
  }

  pub fn read_operand_u16(&self, operand: Operand) -> u16 {
    match operand {
      Operand::RegAF => self.reg_af,
      Operand::RegBC => self.reg_bc,
      Operand::RegDE => self.reg_de,
      Operand::RegHL => self.reg_hl,
      Operand::RegSP => self.reg_sp,
      Operand::RegPC => self.reg_pc,
      Operand::Imm16(i) => i,
      _ => panic!("cpu.read_operand_u16: unrecognized operand: {}", operand),
    }
  }

  pub fn write_operand_u8(&mut self, operand: Operand, value: u8) {
    match operand {
      Operand::RegA => self.reg_af = (value as u16) << 8 | low_byte(self.reg_af) as u16,
      Operand::RegF => self.reg_af = (high_byte(self.reg_af) as u16) << 8 | (value as u16 & 0xf0),
      Operand::RegB => self.reg_bc = (value as u16) << 8 | low_byte(self.reg_bc) as u16,
      Operand::RegC => self.reg_bc = (high_byte(self.reg_bc) as u16) << 8 | value as u16,
      Operand::RegD => self.reg_de = (value as u16) << 8 | low_byte(self.reg_de) as u16,
      Operand::RegE => self.reg_de = (high_byte(self.reg_de) as u16) << 8 | value as u16,
      Operand::RegH => self.reg_hl = (value as u16) << 8 | low_byte(self.reg_hl) as u16,
      Operand::RegL => self.reg_hl = (high_byte(self.reg_hl) as u16) << 8 | value as u16,
      Operand::AddrBC => {
        let bc = self.reg_bc;
        self.write_u8(bc, value)
      }
      Operand::AddrDE => {
        let de = self.reg_de;
        self.write_u8(de, value)
      }
      Operand::AddrHL => {
        let hl = self.reg_hl;
        self.write_u8(hl, value)
      }
      Operand::AddrSP => {
        let sp = self.reg_sp;
        self.write_u8(sp, value)
      }
      Operand::AddrImm16(i) => self.write_u8(i, value),
      Operand::AddrIoPortC => {
        let c = self.read_reg_u8(Reg::C);
        self.write_u8(0xff00 + c as u16, value);
      }
      _ => panic!("cpu.write_operand_u8: unrecognized operand: {}", operand),
    }
  }

  pub fn write_operand_u16(&mut self, operand: Operand, value: u16) {
    match operand {
      Operand::RegAF => self.reg_af = value & 0xfff0,
      Operand::RegBC => self.reg_bc = value,
      Operand::RegDE => self.reg_de = value,
      Operand::RegHL => self.reg_hl = value,
      Operand::RegSP => self.reg_sp = value,
      Operand::RegPC => self.reg_pc = value,
      Operand::AddrImm16(i) => self.write_u16(i, value),
      _ => panic!("cpu.write_operand_u16: unrecognized operand: {}", operand),
    }
  }

  pub fn read_reg_u8(&self, register: Reg) -> u8 {
    match register {
      Reg::B => high_byte(self.reg_bc),
      Reg::C => low_byte(self.reg_bc),
      Reg::D => high_byte(self.reg_de),
      Reg::E => low_byte(self.reg_de),
      Reg::H => high_byte(self.reg_hl),
      Reg::L => low_byte(self.reg_hl),
      Reg::A => high_byte(self.reg_af),
      Reg::F => low_byte(self.reg_af),
      _ => panic!("read_u8_register unknown register: {:?}", register),
    }
  }

  fn read_reg_u16(&mut self, register: Reg) -> u16 {
    match register {
      Reg::BC => self.reg_bc,
      Reg::DE => self.reg_de,
      Reg::HL => self.reg_hl,
      Reg::AF => self.reg_af,
      Reg::SP => self.reg_sp,
      Reg::PC => self.reg_pc,
      _ => panic!("read_reg_u16 unknown register: {:?}", register),
    }
  }

  pub fn write_reg_u16(&mut self, register: Reg, value: u16) {
    match register {
      Reg::BC => self.reg_bc = value,
      Reg::DE => self.reg_de = value,
      Reg::HL => self.reg_hl = value,
      Reg::AF => self.reg_af = value,
      Reg::SP => self.reg_sp = value,
      Reg::PC => self.reg_pc = value,
      _ => panic!("write_reg_u16 unknown register: {:?}", register),
    }
  }

  pub fn write_reg_u8(&mut self, register: Reg, value: u8) {
    // self.reg_gpr[register as usize] = value;
    match register {
      Reg::B => self.reg_bc = (value as u16) << 8 | low_byte(self.reg_bc) as u16,
      Reg::C => self.reg_bc = (high_byte(self.reg_bc) as u16) << 8 | value as u16,
      Reg::D => self.reg_de = (value as u16) << 8 | low_byte(self.reg_de) as u16,
      Reg::E => self.reg_de = (high_byte(self.reg_de) as u16) << 8 | value as u16,
      Reg::H => self.reg_hl = (value as u16) << 8 | low_byte(self.reg_hl) as u16,
      Reg::L => self.reg_hl = (high_byte(self.reg_hl) as u16) << 8 | value as u16,
      Reg::A => self.reg_af = (value as u16) << 8 | low_byte(self.reg_af) as u16,
      Reg::F => self.reg_af = (high_byte(self.reg_af) as u16) << 8 | value as u16,
      _ => panic!("write_reg_u8 unknown register: {:?}", register),
    }
  }

  pub fn read_u8_safe(&mut self, addr: u16) -> Result<u8, String> {
    self.mcycle(1);
    self.system.read_u8(addr)
  }

  pub fn read_u8(&mut self, addr: u16) -> u8 {
    self.mcycle(1);
    let val = self.system.read_u8(addr);
    match val {
      Ok(v) => v,
      Err(e) => panic!("cpu.read_u8: {}\n{:?}", e, self),
    }
  }

  fn read_u16(&mut self, addr: u16) -> u16 {
    self.mcycle(2);
    let val = self.system.read_u16(addr);
    match val {
      Ok(v) => v,
      Err(e) => panic!("cpu.read_u16: {}\n{:?}", e, self),
    }
  }

  fn write_u8(&mut self, addr: u16, value: u8) {
    self.mcycle(1);
    match self.system.write_u8(addr, value) {
      Ok(v) => v,
      Err(e) => panic!("cpu.write_u8: {}\n{:?}", e, self),
    }
  }

  fn write_u16(&mut self, addr: u16, value: u16) {
    self.mcycle(2);
    match self.system.write_u16(addr, value) {
      Ok(v) => v,
      Err(e) => panic!("cpu.write_u16: {}\n{:?}", e, self),
    }
  }

  fn write_flag(&mut self, flag: Flag, mut value: bool) {
    let mut d = self.read_reg_u8(Reg::F);

    let pos = match flag {
      Flag::Z => 0b10000000,
      Flag::N => 0b01000000,
      Flag::H => 0b00100000,
      Flag::C => 0b00010000,
      Flag::NZ => {
        value = !value;
        0b10000000
      }
      Flag::NC => {
        value = !value;
        0b00010000
      }
    };

    if value {
      d |= pos;
    } else {
      d &= !pos;
    }

    self.write_reg_u8(Reg::F, d);
  }

  fn read_flag(&self, flag: Flag) -> bool {
    let val = self.read_reg_u8(Reg::F);

    match flag {
      Flag::Z => 0b10000000 & val != 0,
      Flag::N => 0b01000000 & val != 0,
      Flag::H => 0b00100000 & val != 0,
      Flag::C => 0b00010000 & val != 0,
      Flag::NZ => 0b10000000 & val == 0,
      Flag::NC => 0b00010000 & val == 0,
    }
  }

  // pub fn reset(&mut self) {
  //   self.reg_gpr = [0; NUM_GPR];
  //   self.reg_sp = 0;
  //   self.reg_pc = 0;
  //   self.cycles = 0;
  // }

  pub fn pc(&self) -> u16 {
    self.reg_pc
  }

  pub fn peek_at(&self, pc: u16) -> Instruction {
    match self.disasm.at(self.system.as_memoryio(), pc) {
      Ok((inst, _)) => inst,
      Err(e) => {
        panic!("cpu.peek: {}", e);
      }
    }
  }

  pub fn step(&mut self) -> (Instruction, u16) {
    if self.halt {
      if self.system.has_interrupt() {
        self.halt = false;
      } else {
        self.machine_cycles += 1;
        self.system.step();
        return (Instruction::HALT, 0);
      }
    }

    match self.disasm.at(self.system.as_memoryio(), self.reg_pc) {
      Ok((inst, inc)) => {
        let pc_at_inst = self.reg_pc;
        self.reg_pc += inc;

        // use std::time::Instant;
        // let n = Instant::now();
        self.execute_instruction(inst);
        // println!("{:?}", n.elapsed());
        self.handle_interrupts();
        self.ime.step();
        self.system.step();

        (inst, pc_at_inst)
      }
      Err(e) => {
        panic!("cpu.step: {}", e);
      }
    }
  }

  fn handle_interrupts(&mut self) {
    if self.ime.enabled() {
      if let Some(int) = self.system.next_interrupt() {
        self.halt = false;
        self.ime.set_enabled(false);
        let pc = self.reg_pc;
        self.push_u16(pc);
        self.mcycle(6);
        self.reg_pc = int.addr();
      };
    }
  }

  fn execute_instruction(&mut self, ins: Instruction) {
    match ins {
      Instruction::Invalid(d) => {
        panic!("execute_instruction: Invalid instruction encountered: {:#04x}\n{:?}",
               d,
               self);
      }
      Instruction::InvalidCB(d) => {
        panic!("execute_instruction: Invalid CB instruction encountered: {:#04x}\n{:?}",
               d,
               self);
      }

      // 0xCB instructions
      Instruction::BIT(o1, o2) => self.inst_BIT(o1, o2),
      Instruction::RES(o1, o2) => self.inst_RES(o1, o2),
      Instruction::RL(o) => self.inst_RL(o),
      Instruction::RLC(o) => self.inst_RLC(o),
      Instruction::RR(o) => self.inst_RR(o),
      Instruction::RRC(o) => self.inst_RRC(o),
      Instruction::RLA => self.inst_RLA(),
      Instruction::SET(o1, o2) => self.inst_SET(o1, o2),
      Instruction::SLA(o) => self.inst_SLA(o),
      Instruction::SRA(o) => self.inst_SRA(o),
      Instruction::SRL(o) => self.inst_SRL(o),
      Instruction::SWAP(v) => self.inst_SWAP(v),

      Instruction::ADC(o1, o2) => self.inst_ADC(o1, o2),
      Instruction::ADD8(o1, o2) => self.inst_ADD8(o1, o2),
      Instruction::ADD_HL(o) => self.inst_ADD_HL(o),
      Instruction::ADD_SP(o) => self.inst_ADD_SP(o),
      Instruction::AND(o) => self.inst_AND(o),
      Instruction::CALL_cc(o1, o2) => self.inst_CALL_cc(o1, o2),
      Instruction::CALL(o) => self.inst_CALL(o),
      Instruction::CCF => self.inst_CCF(),
      Instruction::CP(o) => self.inst_CP(o),
      Instruction::CPL => self.inst_CPL(),
      Instruction::DAA => self.inst_DAA(),
      Instruction::DEC8(o) => self.inst_DEC8(o),
      Instruction::DEC16(o) => self.inst_DEC16(o),
      Instruction::DI => self.inst_DI(),
      Instruction::EI => self.inst_EI(),
      Instruction::HALT => self.inst_HALT(),
      Instruction::INC8(o) => self.inst_INC8(o),
      Instruction::INC16(o) => self.inst_INC16(o),
      Instruction::JP(o) => self.inst_JP(o),
      Instruction::JP_cc(o1, o2) => self.inst_JP_cc(o1, o2),
      Instruction::JR(o) => self.inst_JR(o),
      Instruction::JR_cc(o1, o2) => self.inst_JR_cc(o1, o2),
      Instruction::LD8(o1, o2) => self.inst_LD8(o1, o2),
      Instruction::LD16(o1, o2) => self.inst_LD16(o1, o2),
      Instruction::LDD(o1, o2) => self.inst_LDD(o1, o2),
      Instruction::LD_HL(o1, o2) => self.inst_LD_HL(o1, o2),
      Instruction::LDI(o1, o2) => self.inst_LDI(o1, o2),
      Instruction::NOP => self.inst_NOP(),
      Instruction::OR(o1, o2) => self.inst_OR(o1, o2),
      Instruction::POP16(o) => self.inst_POP16(o),
      Instruction::PUSH16(o) => self.inst_PUSH16(o),
      Instruction::RET => self.inst_RET(),
      Instruction::RET_cc(o) => self.inst_RET_cc(o),
      Instruction::RETI => self.inst_RETI(),
      Instruction::RLCA => self.inst_RLCA(),
      Instruction::RRA => self.inst_RRA(),
      Instruction::RRCA => self.inst_RRCA(),
      Instruction::RST(o) => self.inst_RST(o),
      Instruction::SBC(o) => self.inst_SBC(o),
      Instruction::SCF => self.inst_SCF(),
      Instruction::STOP => self.inst_STOP(),
      Instruction::SUB(o1, o2) => self.inst_SUB(o1, o2),
      Instruction::XOR(o1, o2) => self.inst_XOR(o1, o2),

      // _ => panic!("instruction not implemented: {:?}\n{:?}", ins, self),
    };
  }

  fn pop_u16(&mut self) -> u16 {
    let sp = self.reg_sp;
    let val = self.read_u16(sp);
    self.reg_sp = self.reg_sp.wrapping_add(2);
    val
  }

  fn push_u16(&mut self, val: u16) {
    self.reg_sp = self.reg_sp.wrapping_sub(2);
    let sp = self.reg_sp;
    self.write_u16(sp, val);
  }

  // BIT b,r
  //   Opcode: 0xcb 0x47 | 0x4f | 0x57 | 0x5f | 0x67 | 0x6f | 0x77 | 0x7f |
  //                0x40 | 0x48 | 0x50 | 0x58 | 0x60 | 0x68 | 0x70 | 0x78 |
  //                0x41 | 0x49 | 0x51 | 0x59 | 0x61 | 0x69 | 0x71 | 0x79 |
  //                0x42 | 0x4a | 0x52 | 0x5a | 0x62 | 0x6a | 0x72 | 0x7a |
  //                0x43 | 0x4b | 0x53 | 0x5b | 0x63 | 0x6b | 0x73 | 0x7b |
  //                0x44 | 0x4c | 0x54 | 0x5c | 0x64 | 0x6c | 0x74 | 0x7c |
  //                0x45 | 0x4d | 0x55 | 0x5d | 0x65 | 0x6d | 0x75 | 0x7d
  // BIT b,(HL)
  //   Opcode: 0xcb 0x46 | 0x4e | 0x56 | 0x5e | 0x66 | 0x6e | 0x76 | 0x7e
  // Page: 242
  #[allow(non_snake_case)]
  fn inst_BIT(&mut self, o1: Operand, o2: Operand) {
    let val1 = self.read_operand_u8(o1);
    let val2 = self.read_operand_u8(o2);

    self.write_flag(Flag::Z, val2 & (1 << val1) == 0);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, true);
  }

  // RES b,r
  //   Opcode: 0xcb 0x87 | 0x8f | 0x97 | 0x9f | 0xa7 | 0xaf | 0xb7 | 0xbf |
  //                0x80 | 0x88 | 0x90 | 0x98 | 0xa0 | 0xa8 | 0xb0 | 0xb8 |
  //                0x81 | 0x89 | 0x91 | 0x99 | 0xa1 | 0xa9 | 0xb1 | 0xb9 |
  //                0x82 | 0x8a | 0x92 | 0x9a | 0xa2 | 0xaa | 0xb2 | 0xba |
  //                0x83 | 0x8b | 0x93 | 0x9b | 0xa3 | 0xab | 0xb3 | 0xbb |
  //                0x84 | 0x8c | 0x94 | 0x9c | 0xa4 | 0xac | 0xb4 | 0xbc |
  //                0x85 | 0x8d | 0x95 | 0x9d | 0xa5 | 0xad | 0xb5 | 0xbd
  // RES b,(HL)
  //   Opcode: 0xcb 0x86 | 0x8e | 0x96 | 0x9e | 0xa6 | 0xae | 0xb6 | 0xbe
  #[allow(non_snake_case)]
  fn inst_RES(&mut self, o1: Operand, o2: Operand) {
    let val1 = self.read_operand_u8(o1);
    let val2 = self.read_operand_u8(o2);
    let result = val2 & !(1 << val1);

    self.write_operand_u8(o2, result);
  }

  // RL r
  //   Opcode: 0xcb 0x17 | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15
  // RL (HL)
  //   Opcode: 0xcb 0x16
  // Page: 220
  #[allow(non_snake_case)]
  fn inst_RL(&mut self, o: Operand) {
    let mut val = self.read_operand_u8(o);
    let prev_carry = self.read_flag(Flag::C);
    let carry = val & (1 << 7) != 0;

    val <<= 1;

    if prev_carry {
      val |= 1; // set bit 0 to 1
    } else {
      val &= !1; // set bit 0 to 0
    }

    self.write_operand_u8(o, val);
    self.write_flag(Flag::Z, val == 0);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, false);
    self.write_flag(Flag::C, carry);
  }

  // RLC r
  //   Opcode: 0xcb 0x07 | 0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05
  // RLC (HL)
  //   Opcode: 0xcb 0x06
  #[allow(non_snake_case)]
  fn inst_RLC(&mut self, o: Operand) {
    let val = self.read_operand_u8(o);
    let carry = val & 0x80 != 0;
    let result = val.rotate_left(1);

    self.write_operand_u8(o, result);
    self.write_flag(Flag::Z, result == 0);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, false);
    self.write_flag(Flag::C, carry);
  }

  // RR r
  //   Opcode: 0xcb 0x1f | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d
  // RR (HL)
  //   Opcode: 0xcb 0x1e
  // Page: 226
  // Opcode incorrect in z80undocumented manual
  #[allow(non_snake_case)]
  fn inst_RR(&mut self, o: Operand) {
    let mut val = self.read_operand_u8(o);
    let prev_carry = self.read_flag(Flag::C);
    let carry = val & 1 != 0;

    val >>= 1;

    if prev_carry {
      val |= 0b10000000; // set bit 7 to 1
    } else {
      val &= !0b10000000; // set bit 7 to 0
    }

    self.write_operand_u8(o, val);
    self.write_flag(Flag::Z, val == 0);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, false);
    self.write_flag(Flag::C, carry);
  }

  // RRC r
  //  Opcode: 0xcb 0x0f | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d
  // RRC (HL)
  //  Opcode: 0xcb 0x0e
  #[allow(non_snake_case)]
  fn inst_RRC(&mut self, o: Operand) {
    let val = self.read_operand_u8(o);
    let carry = val & 0x1 != 0;
    let result = val.rotate_right(1);

    self.write_operand_u8(o, result);
    self.write_flag(Flag::Z, result == 0);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, false);
    self.write_flag(Flag::C, carry);
  }

  // SET b,r
  //   Opcode: 0xcb 0xc7 | 0xcf | 0xd7 | 0xdf | 0xe7 | 0xef | 0xf7 |
  //                0xff | 0xc0 | 0xc8 | 0xd0 | 0xd8 | 0xe0 | 0xe8 |
  //                0xf0 | 0xf8 | 0xc1 | 0xc9 | 0xd1 | 0xd9 | 0xe1 |
  //                0xe9 | 0xf1 | 0xf9 | 0xc2 | 0xca | 0xd2 | 0xda |
  //                0xe2 | 0xea | 0xf2 | 0xfa | 0xc3 | 0xcb | 0xd3 |
  //                0xdb | 0xe3 | 0xeb | 0xf3 | 0xfb | 0xc4 | 0xcc |
  //                0xd4 | 0xdc | 0xe4 | 0xec | 0xf4 | 0xfc | 0xc5 |
  //                0xcd | 0xd5 | 0xdd | 0xe5 | 0xed | 0xf5 | 0xfd
  // SET b,(HL)
  //   Opcode: 0xcb 0xc6 | 0xce | 0xd6 | 0xde | 0xe6 | 0xee | 0xf6 | 0xfe
  #[allow(non_snake_case)]
  fn inst_SET(&mut self, o1: Operand, o2: Operand) {
    let val1 = self.read_operand_u8(o1);
    let val2 = self.read_operand_u8(o2);
    let result = val2 | (1 << val1);

    self.write_operand_u8(o2, result);
  }

  // SLA r
  //   Opcode: 0xcb 0x27 | 0x20 | 0x21 | 0x22 | 0x23 | 0x24 | 0x25
  // SLA (HL)
  //   Opcode: 0xcb 0x26
  #[allow(non_snake_case)]
  fn inst_SLA(&mut self, o: Operand) {
    let val = self.read_operand_u8(o);
    let carry = val & 0x80 != 0;
    let result = val << 1;

    self.write_operand_u8(o, result);
    self.write_flag(Flag::Z, result == 0);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, false);
    self.write_flag(Flag::C, carry);
  }

  // SRA r
  //   Opcode: 0xcb 0x2f | 0x28 | 0x29 | 0x2a | 0x2b | 0x2c | 0x2d
  // SRA (HL)
  //   Opcode: 0xcb 0x2e
  #[allow(non_snake_case)]
  fn inst_SRA(&mut self, o: Operand) {
    let val = self.read_operand_u8(o);
    let carry = val & 0x1 != 0;
    let bit7 = val & 0x80;
    let result = (val >> 1) | bit7;

    self.write_operand_u8(o, result);
    self.write_flag(Flag::Z, result == 0);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, false);
    self.write_flag(Flag::C, carry);
  }

  // SRL r
  //   Opcode: 0xcb 0x3f | 0x38 | 0x39 | 0x3a | 0x3b | 0x3c | 0x3d
  // SRL (HL)
  //   Opcode: 0xcb 0x3e
  // Page: 235
  #[allow(non_snake_case)]
  fn inst_SRL(&mut self, o: Operand) {
    let val = self.read_operand_u8(o);
    let carry = val & 0x1 != 0;

    let val = val.wrapping_shr(1);

    self.write_operand_u8(o, val);
    self.write_flag(Flag::Z, val == 0);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, false);
    self.write_flag(Flag::C, carry);
  }

  // SWAP r
  //   Opcode: 0xcb 0x37 | 0x30 | 0x31 | 0x32 | 0x33 | 0x34 | 0x35
  // SWAP (HL)
  //   Opcode: 0xcb 0x36
  #[allow(non_snake_case)]
  fn inst_SWAP(&mut self, o: Operand) {
    let val = self.read_operand_u8(o);
    let result = val << 4 | val >> 4;

    self.write_operand_u8(o, result);
    self.write_flag(Flag::Z, result == 0);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, false);
    self.write_flag(Flag::C, false);
  }

  // ADC A,(HL)
  //   Opcode: 0x8e
  // ADC A,n
  //   Opcode: 0xce
  // ADC A,r
  //   Opcode: 0x88 | 0x89 | 0x8a | 0x8b | 0x8c | 0x8d | 0x8f
  // Page: 164
  #[allow(non_snake_case)]
  fn inst_ADC(&mut self, o1: Operand, o2: Operand) {
    let val1 = self.read_operand_u8(o1);
    let val2 = self.read_operand_u8(o2);

    let c = if self.read_flag(Flag::C) { 1 } else { 0 };

    let (result, carry1) = val1.overflowing_add(val2);
    let (result, carry2) = result.overflowing_add(c);

    self.write_operand_u8(o1, result);
    self.write_flag(Flag::Z, result == 0);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, (val1 & 0x0f) + (val2 & 0x0f) + c > 0x0f);
    self.write_flag(Flag::C, carry1 || carry2);
  }

  // ADD A,n
  //   Opcode: 0xc6
  // ADD A,(HL)
  //   Opcode: 0x86
  #[allow(non_snake_case)]
  fn inst_ADD8(&mut self, o1: Operand, o2: Operand) {
    let val1 = self.read_operand_u8(o1);
    let val2 = self.read_operand_u8(o2);

    let (result, carry) = val1.overflowing_add(val2);

    self.write_reg_u8(Reg::A, result);
    self.write_flag(Flag::Z, result == 0);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, ((result ^ val1 ^ val2) & 0x10) > 0);
    self.write_flag(Flag::C, carry);
  }

  // ADD HL,rr
  //   Opcode: 0x09 | 0x19 | 0x29 | 0x39
  #[allow(non_snake_case)]
  fn inst_ADD_HL(&mut self, o: Operand) {
    let val1 = self.read_reg_u16(Reg::HL);
    let val2 = self.read_operand_u16(o);

    let (result, carry) = val1.overflowing_add(val2);

    self.write_reg_u16(Reg::HL, result);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, (result ^ val1 ^ val2) & 0x1000 != 0);
    self.write_flag(Flag::C, carry);

    self.mcycle(1);
  }

  // ADD SP,n
  //   Opcode: 0xe8
  #[allow(non_snake_case)]
  fn inst_ADD_SP(&mut self, o: Operand) {
    let val1 = self.read_reg_u16(Reg::SP);
    let val2 = self.read_operand_u8(o) as i8 as u16;
    let carry = (((val1 & 0xFF) + (val2 & 0xFF)) & 0x100) > 0;

    let result = val1.wrapping_add(val2);

    self.write_reg_u16(Reg::SP, result);
    self.write_flag(Flag::Z, false);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, (((val1 & 0xF) + (val2 & 0xF)) & 0x10) > 0);
    self.write_flag(Flag::C, carry);

    self.mcycle(2);
  }

  // AND n
  //   Opcode: 0xe6
  // AND (HL)
  //   Opcode: 0xa6
  // AND r
  //   Opcode: 0xa7 | 0xa0 | 0xa1 | 0xa2 | 0xa3 | 0xa4 | 0xa5
  // Page: 170
  #[allow(non_snake_case)]
  fn inst_AND(&mut self, o: Operand) {
    let val = self.read_operand_u8(o);
    let result = self.read_reg_u8(Reg::A) & val;
    self.write_reg_u8(Reg::A, result);

    self.write_flag(Flag::Z, result == 0);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, true);
    self.write_flag(Flag::C, false);
  }

  // CALL cc,nn
  //   Opcode: 0xc4 | 0xcc | 0xd4 | 0xdc
  // Page: 275
  #[allow(non_snake_case)]
  fn inst_CALL_cc(&mut self, o1: Operand, o2: Operand) {
    if self.read_operand_u8(o1) != 0 {
      let nn = self.read_operand_u16(o2);

      let pc = self.reg_pc;
      self.push_u16(pc);
      self.reg_pc = nn;

      self.mcycle(2);
    }
  }

  // CALL nn
  //   Opcode: 0xcd
  // Page: 273
  #[allow(non_snake_case)]
  fn inst_CALL(&mut self, o: Operand) {
    let nn = self.read_operand_u16(o);

    let pc = self.reg_pc;
    self.push_u16(pc);
    self.reg_pc = nn;

    self.mcycle(1);
  }

  // CCF
  //   Opcode: 0x3f
  #[allow(non_snake_case)]
  fn inst_CCF(&mut self) {
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, false);
    let c = self.read_flag(Flag::C);
    self.write_flag(Flag::C, !c);
  }

  // CP n
  //   Opcode: 0xfe
  // CP r
  //   Opcode: 0xbf | 0xb8 | 0xb9 | 0xba | 0xbb | 0xbc | 0xbd
  // CP (HL)
  //   Opcode: 0xbe
  // Page: 176
  #[allow(non_snake_case)]
  fn inst_CP(&mut self, o: Operand) {
    let val = self.read_operand_u8(o);
    let a = self.read_reg_u8(Reg::A);
    let (result, carry) = a.overflowing_sub(val);

    self.write_flag(Flag::Z, result == 0);
    self.write_flag(Flag::N, true);
    self.write_flag(Flag::H, a & 0x0F < val & 0x0F);
    self.write_flag(Flag::C, carry);
  }

  // CPL
  //   Opcode: 0x2f
  #[allow(non_snake_case)]
  fn inst_CPL(&mut self) {
    let val = self.read_reg_u8(Reg::A) ^ 0xff;
    self.write_reg_u8(Reg::A, val);
    self.write_flag(Flag::N, true);
    self.write_flag(Flag::H, true);
  }

  // DAA
  //   Opcode: 0x27
  #[allow(non_snake_case)]
  fn inst_DAA(&mut self) {
    // Based on http://forums.nesdev.com/viewtopic.php?t=9088

    let mut val = self.read_reg_u8(Reg::A) as usize;

    if !self.read_flag(Flag::N) {
      if self.read_flag(Flag::H) || (val & 0xf) > 9 {
        val = val.wrapping_add(0x06);
      }

      if self.read_flag(Flag::C) || (val > 0x9f) {
        val = val.wrapping_add(0x60);
      }
    } else {
      if self.read_flag(Flag::H) {
        val = (val.wrapping_sub(6)) & 0xff;
      }

      if self.read_flag(Flag::C) {
        val = val.wrapping_sub(0x60);
      }
    }

    if (val & 0x100) == 0x100 {
      self.write_flag(Flag::C, true);
    }

    let val = (val & 0xff) as u8;

    self.write_reg_u8(Reg::A, val);
    self.write_flag(Flag::Z, val == 0);
    self.write_flag(Flag::H, false);
  }

  // DEC (HL)
  //   Opcode: 0x35
  // DEC r
  //   Opcode: 0x05 | 0x0d | 0x15 | 0x1d | 0x25 | 0x2d | 0x3d
  #[allow(non_snake_case)]
  fn inst_DEC8(&mut self, o: Operand) {
    let val = self.read_operand_u8(o);
    let new_val = val.wrapping_sub(1);

    self.write_operand_u8(o, new_val);
    self.write_flag(Flag::Z, new_val == 0);
    self.write_flag(Flag::N, true);
    self.write_flag(Flag::H, val & 0xf == 0);
  }

  // DEC rr
  //   Opcode: 0x0b | 0x1b | 0x2b | 0x3b
  // Page: 205
  #[allow(non_snake_case)]
  fn inst_DEC16(&mut self, o: Operand) {
    let val = self.read_operand_u16(o);
    let val = val.wrapping_sub(1);
    self.write_operand_u16(o, val);

    self.mcycle(1);
  }

  // DI
  // Opcode: 0xf3
  // Page: 192
  #[allow(non_snake_case)]
  fn inst_DI(&mut self) {
    self.ime.set_enabled(false);
  }

  // EI
  // Opcode: 0xfb
  // Page: 193
  #[allow(non_snake_case)]
  fn inst_EI(&mut self) {
    self.ime.set_enabling();
  }

  // HALT
  // Opcode: 0x76
  #[allow(non_snake_case)]
  fn inst_HALT(&mut self) {
    self.halt = true;
    // println!("halted @ {:#06x}!", self.reg_pc);
  }

  // INC r
  //   Opcode: 0x04 | 0x0c | 0x14 | 0x1c | 0x24 | 0x2c | 0x3c
  // Page: 178
  #[allow(non_snake_case)]
  fn inst_INC8(&mut self, o: Operand) {
    let val = self.read_operand_u8(o);
    let new_val = val.wrapping_add(1);

    self.write_operand_u8(o, new_val);
    self.write_flag(Flag::Z, new_val == 0);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, val & 0xf == 0xf);
  }

  // INC rr
  //   Opcode: 0x03 | 0x13 | 0x23 | 0x33
  // Page: 202
  // Originally called INC ss
  #[allow(non_snake_case)]
  fn inst_INC16(&mut self, o: Operand) {
    let val = self.read_operand_u16(o);
    let val = val.wrapping_add(1);
    self.write_operand_u16(o, val);

    self.mcycle(1);
  }

  // JP HL
  //   Opcode: 0xe9
  // JP nn
  //   Opcode: 0xc3
  #[allow(non_snake_case)]
  fn inst_JP(&mut self, o: Operand) {
    let val = self.read_operand_u16(o);
    self.reg_pc = val;
  }

  // JP cc, nn
  // Opcode: 0xc2 | 0xca | 0xd2 | 0xda
  // Page: 257
  #[allow(non_snake_case)]
  fn inst_JP_cc(&mut self, o1: Operand, o2: Operand) {
    if self.read_operand_u8(o1) != 0 {
      let val = self.read_operand_u16(o2);
      self.reg_pc = val;
      self.mcycle(1);
    }
  }

  // JR e
  //   Opcode: 0x18
  // Page: 259
  #[allow(non_snake_case)]
  fn inst_JR(&mut self, o: Operand) {
    let val = self.read_operand_u8(o) as i8;
    // signed addition (can jump back)
    self.reg_pc = ((self.reg_pc as i16) + (val as i16)) as u16;
    self.mcycle(1);
  }

  // JR cc,e
  //   Opcode: 000cc000
  // Page: 266
  #[allow(non_snake_case)]
  fn inst_JR_cc(&mut self, o1: Operand, o2: Operand) {
    // signed argument
    if self.read_operand_u8(o1) != 0 {
      let val = self.read_operand_u8(o2) as i8;
      // signed addition (can jump back)
      self.reg_pc = ((self.reg_pc as i16) + (val as i16)) as u16;
      self.mcycle(1);
    }
  }

  // LD
  #[allow(non_snake_case)]
  fn inst_LD8(&mut self, o1: Operand, o2: Operand) {
    let val = self.read_operand_u8(o2);
    self.write_operand_u8(o1, val);
  }

  // LD
  #[allow(non_snake_case)]
  fn inst_LD16(&mut self, o1: Operand, o2: Operand) {
    let val = self.read_operand_u16(o2);
    self.write_operand_u16(o1, val);
    self.mcycle(1);
  }

  // LDD (HL),A
  //   Opcode: 0x32
  //   Moved: LD (nn),A -> LDD (HL),A
  //   Page: 149
  // LDD A,(HL)
  //   Opcode: 0x3a
  //   Moved: LD A,(nn) -> LDD A,(HL)
  #[allow(non_snake_case)]
  fn inst_LDD(&mut self, o1: Operand, o2: Operand) {
    let val = self.read_operand_u8(o2);
    self.write_operand_u8(o1, val);
    self.reg_hl = self.reg_hl.wrapping_sub(1);
  }

  // LD HL,SP+n
  //    Opcode: 0xf8
  #[allow(non_snake_case)]
  fn inst_LD_HL(&mut self, o1: Operand, o2: Operand) {
    let val1 = self.read_operand_u16(o1);
    let val2 = self.read_operand_u8(o2) as i8 as u16;
    let carry = (((val1 & 0xFF) + (val2 & 0xFF)) & 0x100) > 0;
    let result = val1.wrapping_add(val2);

    self.write_reg_u16(Reg::HL, result);
    self.write_flag(Flag::Z, false);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, (((val1 & 0xF) + (val2 & 0xF)) & 0x10) > 0);
    self.write_flag(Flag::C, carry);

    self.mcycle(1);
  }

  // LDI (HL),A
  //   Opcode: 0x22
  //   Moved: LD (nn),HL -> LDI (HL),A
  //   Page: 149
  // LDI A,(HL)
  //   Opcode: 0x2a
  //   Moved: LD HL,(nn) -> LDI A,(HL)
  //   Page: 146
  #[allow(non_snake_case)]
  fn inst_LDI(&mut self, o1: Operand, o2: Operand) {
    let val = self.read_operand_u8(o2);
    self.write_operand_u8(o1, val);
    self.reg_hl = self.reg_hl.wrapping_add(1);
  }

  // NOP
  //   Opcode: 0x00
  #[allow(non_snake_case)]
  fn inst_NOP(&self) {}

  // OR A,(HL)
  //   Opcode: 0xb6
  //   Page: 172
  // OR r
  //   Opcode: 0xb0 | 0xb1 | 0xb2 | 0xb3 | 0xb4 | 0xb5 | 0xb7
  //   Page: 172
  // OR n
  //   Opcode: 0xf6
  #[allow(non_snake_case)]
  fn inst_OR(&mut self, o1: Operand, o2: Operand) {
    let val1 = self.read_operand_u8(o1);
    let val2 = self.read_operand_u8(o2);
    let result = val1 | val2;

    self.write_operand_u8(o1, result);
    self.write_flag(Flag::Z, result == 0);
    self.write_flag(Flag::H, false);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::C, false);
  }

  // POP rr
  //   Opcode: 11rr0001
  //   Page: 137
  #[allow(non_snake_case)]
  fn inst_POP16(&mut self, o: Operand) {
    let val = self.pop_u16();
    self.write_operand_u16(o, val);
  }

  // PUSH rr
  //   Opcode: 11rr0101
  //   Page: 134
  #[allow(non_snake_case)]
  fn inst_PUSH16(&mut self, o: Operand) {
    let val = self.read_operand_u16(o);
    self.push_u16(val);
    self.mcycle(1);
  }

  // RET
  //   Opcode: 0xc9
  //   Page: 278
  #[allow(non_snake_case)]
  fn inst_RET(&mut self) {
    self.reg_pc = self.pop_u16();
    self.mcycle(1);
  }

  // RET cc
  //   Opcode: 0xc0 | 0xc8 | 0xd0 | 0xd8
  //   Page: 279
  #[allow(non_snake_case)]
  fn inst_RET_cc(&mut self, o: Operand) {
    if self.read_operand_u8(o) != 0 {
      self.reg_pc = self.pop_u16();
      self.mcycle(1);
    }
  }

  // RETI
  //   Opcode: 0xd9
  #[allow(non_snake_case)]
  fn inst_RETI(&mut self) {
    self.reg_pc = self.pop_u16();
    self.ime.set_enabled(true);
  }

  // RLA
  // Opcode: 0xCB 0x17
  // Page: 209
  #[allow(non_snake_case)]
  fn inst_RLA(&mut self) {
    let mut d = self.read_reg_u8(Reg::A);
    let prev_carry = self.read_flag(Flag::C);
    let carry = d & (1 << 7) != 0;

    d <<= 1;

    if prev_carry {
      d |= 1; // set bit 0 to 1
    } else {
      d &= !1; // set bit 0 to 0
    }

    self.write_reg_u8(Reg::A, d);
    self.write_flag(Flag::Z, false);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, false);
    self.write_flag(Flag::C, carry);
  }

  // RLCA
  //   Opcode: 0x07
  #[allow(non_snake_case)]
  fn inst_RLCA(&mut self) {
    let val = self.read_reg_u8(Reg::A);
    let carry = val & 0b10000000 != 0;
    let result = val.rotate_left(1);

    self.write_reg_u8(Reg::A, result);
    self.write_flag(Flag::Z, false);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, false);
    self.write_flag(Flag::C, carry);
  }

  // RRA
  //   Opcode: 0x1f
  //   Page: 211
  #[allow(non_snake_case)]
  fn inst_RRA(&mut self) {
    let val = self.read_reg_u8(Reg::A);
    let prev_carry = self.read_flag(Flag::C);
    let carry = val & 0x1 != 0;
    let mut result = val >> 1;

    if prev_carry {
      result |= 0b10000000; // set bit 7 to 1
    } else {
      result &= !0b10000000; // set bit 7 to 0
    }

    self.write_reg_u8(Reg::A, result);
    self.write_flag(Flag::Z, false);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, false);
    self.write_flag(Flag::C, carry);
  }

  // RRA
  //   Opcode: 0x0f
  #[allow(non_snake_case)]
  fn inst_RRCA(&mut self) {
    let val = self.read_reg_u8(Reg::A);
    let carry = val & 0x1 != 0;
    let result = val.rotate_right(1);

    self.write_reg_u8(Reg::A, result);
    self.write_flag(Flag::Z, false);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, false);
    self.write_flag(Flag::C, carry);
  }

  // RST n
  //   Opcode: 0xc7 | 0xcf | 0xd7 | 0xdf | 0xe7 | 0xef | 0xf7 | 0xff
  //   Page: 285
  #[allow(non_snake_case)]
  fn inst_RST(&mut self, o: Operand) {
    let pc = self.reg_pc;
    self.push_u16(pc);
    let val = self.read_operand_u8(o);
    self.reg_pc = val as u16;
  }

  // SBC A,r
  //   Opcode:
  #[allow(non_snake_case)]
  fn inst_SBC(&mut self, o: Operand) {
    let val1 = self.read_reg_u8(Reg::A);
    let val2 = self.read_operand_u8(o);
    let carry_val = if self.read_flag(Flag::C) { 1 } else { 0 };
    let (result, carry1) = val1.overflowing_sub(val2);
    let (result, carry2) = result.overflowing_sub(carry_val);

    self.write_reg_u8(Reg::A, result);
    self.write_flag(Flag::Z, result == 0);
    self.write_flag(Flag::N, true);
    self.write_flag(Flag::H, val1 & 0xf < (val2 & 0xf) + carry_val);
    self.write_flag(Flag::C, carry1 || carry2);
  }

  // SCF
  //   Opcode: 0x37
  #[allow(non_snake_case)]
  fn inst_SCF(&mut self) {
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, false);
    self.write_flag(Flag::C, true);
  }

  // STOP
  //   Opcode: 0x10
  #[allow(non_snake_case)]
  fn inst_STOP(&mut self) {
    // println!("STOP not implemented");
  }

  // SUB n
  //   Opcode: 0xd6
  //   Page: 166
  // SUB r
  //   Opcode: 0x90 | 0x91 | 0x92 | 0x93 | 0x94 | 0x95 | 0x97
  //   Page: 166
  #[allow(non_snake_case)]
  fn inst_SUB(&mut self, o1: Operand, o2: Operand) {
    let val1 = self.read_operand_u8(o1);
    let val2 = self.read_operand_u8(o2);
    let (result, carry) = val1.overflowing_sub(val2);

    self.write_operand_u8(o1, result);
    self.write_flag(Flag::Z, result == 0);
    self.write_flag(Flag::N, true);
    self.write_flag(Flag::H, val1 & 0x0F < val2 & 0x0F);
    self.write_flag(Flag::C, carry);
  }

  // XOR (HL)
  //   Opcode: 0xae
  //   Page: 174
  // XOR n
  //   Opcode: 0xee
  //   Page: 174
  // XOR r
  //   Opcode: 10110rrr
  //   Page: 174
  #[allow(non_snake_case)]
  fn inst_XOR(&mut self, o1: Operand, o2: Operand) {
    let val1 = self.read_operand_u8(o1);
    let val2 = self.read_operand_u8(o2);
    let result = val1 ^ val2;

    self.write_operand_u8(o1, result);
    self.write_flag(Flag::Z, result == 0);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, false);
    self.write_flag(Flag::C, false);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use super::super::system::SystemCtrl;
  use super::super::mem::MemoryIo;
  use std::io::Read;
  use std::fs::File;
  use std;

  use serde_json::{self, Map};

  struct TestSystem {
    ram: [u8; 0xFFFF + 1],
  }

  impl MemoryIo for TestSystem {
    fn read_u8(&self, addr: u16) -> Result<u8, String> {
      self.ram
        .get(addr as usize)
        .ok_or(format!("could not get byte at test ram offset {}", addr))
        .and_then(|&x| Ok(x))
    }

    fn write_u8(&mut self, addr: u16, value: u8) -> Result<(), String> {
      self.ram[addr as usize] = value;
      Ok(())
    }
  }

  impl SystemCtrl for TestSystem {
    fn as_memoryio(&self) -> &MemoryIo {
      self as &MemoryIo
    }
    fn has_interrupt(&self) -> bool {
      false
    }
  }

  impl TestSystem {
    fn new() -> TestSystem {
      TestSystem { ram: [0; 0xFFFF + 1] }
    }
  }

  fn testcpu() -> Cpu {
    Cpu::new(Box::new(TestSystem::new()))
  }

  struct HexVec(Vec<u8>);
  impl std::fmt::LowerHex for HexVec {
    fn fmt(&self, fmtr: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
      try!(fmtr.write_fmt(format_args!("[")));
      for (i, byte) in self.0.iter().enumerate() {
        try!(fmtr.write_fmt(format_args!("{:02x}", byte)));
        if i + 1 != self.0.len() {
          try!(fmtr.write_fmt(format_args!(" ")));
        }
      }
      try!(fmtr.write_fmt(format_args!("]")));
      Ok(())
    }
  }

  #[derive(Debug, PartialEq, Serialize, Deserialize)]
  struct CpuData {
    A: u8,
    B: u8,
    C: u8,
    D: u8,
    E: u8,
    F: u8,
    H: u8,
    L: u8,
    PC: u16,
    SP: u16,
    mem: Map<String, u8>,
  }

  #[derive(Debug, PartialEq, Serialize, Deserialize)]
  struct CpuDataTest {
    pre: CpuData,
    post: CpuData,
  }

  #[test]
  fn test_runner_json() {
    let mut f = File::open("testdata/cpu.json").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let data: Map<String, CpuDataTest> = serde_json::from_str(&s).unwrap();

    for (k, v) in data {
      let test_name = k;

      let mut c = testcpu();
      c.write_reg_u8(Reg::A, v.pre.A);
      c.write_reg_u8(Reg::F, v.pre.F);
      c.write_reg_u8(Reg::B, v.pre.B);
      c.write_reg_u8(Reg::C, v.pre.C);
      c.write_reg_u8(Reg::D, v.pre.D);
      c.write_reg_u8(Reg::E, v.pre.E);
      c.write_reg_u8(Reg::H, v.pre.H);
      c.write_reg_u8(Reg::L, v.pre.L);
      c.reg_sp = v.pre.SP;
      c.reg_pc = v.pre.PC;
      for (map_k, map_v) in v.pre.mem {
        c.system
          .write_u8(map_k.parse::<u16>().unwrap(), map_v)
          .unwrap();
      }

      c.step();

      let (v1, v2) = (c.read_reg_u8(Reg::A), v.post.A);
      assert!(v1 == v2,
              "\n{0}:\n A:\n  Got:      {1:#04x} [{1:08b}],\n  Expected: {2:#04x} [{2:08b}]",
              test_name,
              v1,
              v2);

      let (v1, v2) = (c.read_reg_u8(Reg::F), v.post.F);
      let mut flags1 = vec![];
      if v1 & 0b10000000 != 0 {
        flags1.push("Z");
      }
      if v1 & 0b01000000 != 0 {
        flags1.push("N");
      }
      if v1 & 0b00100000 != 0 {
        flags1.push("H");
      }
      if v1 & 0b00010000 != 0 {
        flags1.push("C");
      }
      let mut flags2 = vec![];
      if v2 & 0b10000000 != 0 {
        flags2.push("Z");
      }
      if v2 & 0b01000000 != 0 {
        flags2.push("N");
      }
      if v2 & 0b00100000 != 0 {
        flags2.push("H");
      }
      if v2 & 0b00010000 != 0 {
        flags2.push("C");
      }
      assert!(v1 == v2,
              "\n{0}:\n F:\n  Got:      {1:#04x} [{2:}],\n  Expected: {3:#04x} [{4:}]",
              test_name,
              v1,
              flags1.join(""),
              v2,
              flags2.join(""));


      let (v1, v2) = (c.read_reg_u8(Reg::B), v.post.B);
      assert!(v1 == v2,
              "\n{0}:\n B:\n  Got:      {1:#04x} [{1:08b}],\n  Expected: {2:#04x} [{2:08b}]",
              test_name,
              v1,
              v2);


      let (v1, v2) = (c.read_reg_u8(Reg::C), v.post.C);
      assert!(v1 == v2,
              "\n{0}:\n C:\n  Got:      {1:#04x} [{1:08b}],\n  Expected: {2:#04x} [{2:08b}]",
              test_name,
              v1,
              v2);


      let (v1, v2) = (c.read_reg_u8(Reg::D), v.post.D);
      assert!(v1 == v2,
              "\n{0}:\n D:\n  Got:      {1:#04x} [{1:08b}],\n  Expected: {2:#04x} [{2:08b}]",
              test_name,
              v1,
              v2);

      let (v1, v2) = (c.read_reg_u8(Reg::E), v.post.E);
      assert!(v1 == v2,
              "\n{0}:\n E:\n  Got:      {1:#04x} [{1:08b}],\n  Expected: {2:#04x} [{2:08b}]",
              test_name,
              v1,
              v2);

      let (v1, v2) = (c.read_reg_u8(Reg::H), v.post.H);
      assert!(v1 == v2,
              "\n{0}:\n H:\n  Got:      {1:#04x} [{1:08b}],\n  Expected: {2:#04x} [{2:08b}]",
              test_name,
              v1,
              v2);
      let (v1, v2) = (c.read_reg_u8(Reg::L), v.post.L);
      assert!(v1 == v2,
              "\n{0}:\n L:\n  Got:      {1:#04x} [{1:08b}],\n  Expected: {2:#04x} [{2:08b}]",
              test_name,
              v1,
              v2);

      let (v1, v2) = (c.read_reg_u16(Reg::SP), v.post.SP);
      assert!(v1 == v2,
              "\n{0}:\n SP:\n  Got:      {1:#04x} [{1:08b}],\n  Expected: {2:#04x} [{2:08b}]",
              test_name,
              v1,
              v2);
      let (v1, v2) = (c.read_reg_u16(Reg::PC), v.post.PC);
      assert!(v1 == v2,
              "\n{0}:\n PC:\n  Got:      {1:#04x} [{1:08b}],\n  Expected: {2:#04x} [{2:08b}]",
              test_name,
              v1,
              v2);

      for (map_k, map_v) in v.post.mem {
        let k = map_k.parse::<u16>().unwrap();
        let (v1, v2) = (c.system.read_u8(k).unwrap(), map_v);
        assert!(v1 == v2,
                "\n{0}\nmem {1}:\n  Got:      {2:#04x},\n  Expected: {3:#04x}",
                test_name,
                k,
                v1,
                v2);
      }
    }
  }

  #[test]
  fn test_write_read_reg_u8() {
    let mut c = testcpu();

    c.write_reg_u8(Reg::A, 0b01011010);
    assert!(c.reg_af == 0b01011010_00000000);
    assert!(c.read_reg_u8(Reg::A) == 0b01011010);
    c.write_reg_u8(Reg::F, 0b11011010);
    assert!(c.reg_af == 0b01011010_11011010);
    assert!(c.read_reg_u8(Reg::F) == 0b11011010);

    c.write_reg_u8(Reg::B, 0b01011010);
    assert!(c.reg_bc == 0b01011010_00000000);
    assert!(c.read_reg_u8(Reg::B) == 0b01011010);
    c.write_reg_u8(Reg::C, 0b11011010);
    assert!(c.reg_bc == 0b01011010_11011010);
    assert!(c.read_reg_u8(Reg::C) == 0b11011010);

    c.write_reg_u8(Reg::D, 0b01011010);
    assert!(c.reg_de == 0b01011010_00000000);
    assert!(c.read_reg_u8(Reg::D) == 0b01011010);
    c.write_reg_u8(Reg::E, 0b11011010);
    assert!(c.reg_de == 0b01011010_11011010);
    assert!(c.read_reg_u8(Reg::E) == 0b11011010);

    c.write_reg_u8(Reg::H, 0b01011010);
    assert!(c.reg_hl == 0b01011010_00000000);
    assert!(c.read_reg_u8(Reg::H) == 0b01011010);
    c.write_reg_u8(Reg::L, 0b11011010);
    assert!(c.reg_hl == 0b01011010_11011010);
    assert!(c.read_reg_u8(Reg::L) == 0b11011010);
  }

  #[test]
  fn test_write_read_flag() {
    let mut c = testcpu();

    c.write_flag(Flag::Z, true);
    assert_eq!(c.reg_af, 0b00000000_10000000);
    c.write_flag(Flag::N, true);
    assert_eq!(c.reg_af, 0b00000000_11000000);
    c.write_flag(Flag::H, true);
    assert_eq!(c.reg_af, 0b00000000_11100000);
    c.write_flag(Flag::C, true);
    assert_eq!(c.reg_af, 0b00000000_11110000);

    c.reg_af = 0b11111111_11111111;
    c.write_flag(Flag::NZ, true);
    assert_eq!(c.reg_af, 0b11111111_01111111);

    c.reg_af = 0b11111111_11111111;
    c.write_flag(Flag::Z, true);
    assert_eq!(c.reg_af, 0b11111111_11111111);

    c.reg_af = 0b11111111_11111111;
    c.write_flag(Flag::NC, true);
    assert_eq!(c.reg_af, 0b11111111_11101111);

    c.reg_af = 0b11111111_11111111;
    c.write_flag(Flag::C, true);
    assert_eq!(c.reg_af, 0b11111111_11111111);

    c.reg_af = 0b00000000_00000000;
    c.write_flag(Flag::NZ, true);
    assert_eq!(c.reg_af, 0b00000000_00000000);
  }
}
