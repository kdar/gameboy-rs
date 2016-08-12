use std::fmt;
use std::default::Default;
use std::cmp::PartialEq;

use super::reg::Reg;
use super::flag::Flag;
use super::operand::Operand;
use super::disassembler::Instruction;
use super::disassembler::Disassembler;
use super::system::{System, SystemCtrl};

fn high_byte(value: u16) -> u8 {
  (value >> 8) as u8
}

fn low_byte(value: u16) -> u8 {
  value as u8 & 0b11111111
}

pub struct Cpu {
  reg_af: u16, // Accumulator and flags
  reg_bc: u16, // B and C general purpose
  reg_de: u16, // D and E general purpose
  reg_hl: u16, // H and L general purpose

  reg_sp: u16, // Stack pointer
  reg_pc: u16, // Program counter

  clock_t: u32, // Cycles
  interrupt_master_enable: bool,
  halt: bool,

  system: Box<SystemCtrl>,
  disasm: Disassembler,
}

impl PartialEq for Cpu {
  fn eq(&self, x: &Cpu) -> bool {
    self.reg_af == x.reg_af && self.reg_bc == x.reg_bc && self.reg_de == x.reg_de && self.reg_hl == x.reg_hl &&
    self.reg_sp == x.reg_sp && self.reg_pc == x.reg_pc && self.clock_t == x.clock_t
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
      clock_t: 0,
      interrupt_master_enable: false,
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
    try!(write!(f, "\nClock T: {}", self.clock_t));
    write!(f, "\n")
  }
}

impl Cpu {
  pub fn new(system: Box<SystemCtrl>) -> Cpu {
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

  pub fn read_operand_u8(&mut self, operand: Operand) -> u8 {
    match operand {
      Operand::A => high_byte(self.reg_af),
      Operand::F => low_byte(self.reg_af),
      Operand::B => high_byte(self.reg_bc),
      Operand::C => low_byte(self.reg_bc),
      Operand::D => high_byte(self.reg_de),
      Operand::E => low_byte(self.reg_de),
      Operand::H => high_byte(self.reg_hl),
      Operand::L => low_byte(self.reg_hl),
      Operand::_BC_ => {
        let bc = self.reg_bc;
        self.read_u8(bc)
      }
      Operand::_DE_ => {
        let de = self.reg_de;
        self.read_u8(de)
      }
      Operand::_HL_ => {
        let hl = self.reg_hl;
        self.read_u8(hl)
      }
      Operand::_SP_ => {
        let sp = self.reg_sp;
        self.read_u8(sp)
      }
      Operand::FlagZ => {
        if 0b10000000 & self.reg_af != 0 {
          1
        } else {
          0
        }
      }
      Operand::FlagN => {
        if 0b01000000 & self.reg_af != 0 {
          1
        } else {
          0
        }
      }
      Operand::FlagH => {
        if 0b00100000 & self.reg_af != 0 {
          1
        } else {
          0
        }
      }
      Operand::FlagC => {
        if 0b00010000 & self.reg_af != 0 {
          1
        } else {
          0
        }
      }
      Operand::FlagNZ => {
        if 0b10000000 & self.reg_af == 0 {
          1
        } else {
          0
        }
      }
      Operand::FlagNC => {
        if 0b00010000 & self.reg_af == 0 {
          1
        } else {
          0
        }
      }
      Operand::Imm8(i) => i,
      _ => panic!("cpu.read_operand_u8: unrecognized operand: {}", operand),
    }
  }

  pub fn read_operand_u16(&self, operand: Operand) -> u16 {
    match operand {
      Operand::AF => self.reg_af,
      Operand::BC => self.reg_bc,
      Operand::DE => self.reg_de,
      Operand::HL => self.reg_hl,
      Operand::SP => self.reg_sp,
      Operand::PC => self.reg_pc,
      Operand::Imm16(i) => i,
      _ => panic!("cpu.read_operand_u16: unrecognized operand: {}", operand),
    }
  }

  pub fn write_operand_u8(&mut self, operand: Operand, value: u8) {
    match operand {
      Operand::A => self.reg_af = (value as u16) << 8 | low_byte(self.reg_af) as u16,
      Operand::F => self.reg_af = (high_byte(self.reg_af) as u16) << 8 | value as u16,
      Operand::B => self.reg_bc = (value as u16) << 8 | low_byte(self.reg_bc) as u16,
      Operand::C => self.reg_bc = (high_byte(self.reg_bc) as u16) << 8 | value as u16,
      Operand::D => self.reg_de = (value as u16) << 8 | low_byte(self.reg_de) as u16,
      Operand::E => self.reg_de = (high_byte(self.reg_de) as u16) << 8 | value as u16,
      Operand::H => self.reg_hl = (value as u16) << 8 | low_byte(self.reg_hl) as u16,
      Operand::L => self.reg_hl = (high_byte(self.reg_hl) as u16) << 8 | value as u16,
      Operand::_BC_ => {
        let bc = self.reg_bc;
        self.write_u8(bc, value)
      }
      Operand::_DE_ => {
        let de = self.reg_de;
        self.write_u8(de, value)
      }
      Operand::_HL_ => {
        let hl = self.reg_hl;
        self.write_u8(hl, value)
      }
      Operand::_SP_ => {
        let sp = self.reg_sp;
        self.write_u8(sp, value)
      }
      Operand::Imm16(i) => self.write_u8(i, value),
      _ => panic!("cpu.write_operand_u8: unrecognized operand: {}", operand),
    }
  }

  pub fn write_operand_u16(&mut self, operand: Operand, value: u16) {
    match operand {
      Operand::AF => self.reg_af = value,
      Operand::BC => self.reg_bc = value,
      Operand::DE => self.reg_de = value,
      Operand::HL => self.reg_hl = value,
      Operand::SP => self.reg_sp = value,
      Operand::PC => self.reg_pc = value,
      Operand::Imm16(i) => self.write_u16(i, value),
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

  pub fn read_u8(&mut self, addr: u16) -> u8 {
    let val = self.system.read_u8(addr);
    match val {
      Ok(v) => v,
      Err(e) => panic!("cpu.read_u8: {}\n{:?}", e, self),
    }
  }

  fn read_u16(&mut self, addr: u16) -> u16 {
    let val = self.system.read_u16(addr);
    match val {
      Ok(v) => v,
      Err(e) => panic!("cpu.read_u16: {}\n{:?}", e, self),
    }
  }

  fn write_u8(&mut self, addr: u16, value: u8) {
    match self.system.write_u8(addr, value) {
      Ok(v) => v,
      Err(e) => panic!("cpu.write_u8: {}\n{:?}", e, self),
    }
  }

  fn write_u16(&mut self, addr: u16, value: u16) {
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
    // TODO: unhalt on interrupt
    // if self.halt {
    //
    // }
    match self.disasm.at(self.system.as_memoryio(), self.reg_pc) {
      Ok((inst, inc)) => {
        let pc_at_inst = self.reg_pc;
        self.reg_pc += inc;
        self.execute_instruction(inst);

        self.system.step();

        (inst, pc_at_inst)
      }
      Err(e) => {
        panic!("cpu.step: {}", e);
      }
    }
  }

  fn execute_instruction(&mut self, ins: Instruction) {
    match ins {
      Instruction::Invalid(d) => {
        // Ignore instructions that the Gameboy doesn't support.
        match d {
          0xFC | _ => {
            panic!("execute_instruction: Invalid instruction encountered: {:#04x}\n{:?}",
                   d,
                   self)
          }
        }
      }
      Instruction::InvalidCB(d) => {
        panic!("execute_instruction: Invalid CB instruction encountered: {:#04x}\n{:?}",
               d,
               self);
      }

      // 0xCB instructions
      Instruction::BIT(o1, o2) => self.inst_BIT(o1, o2),
      Instruction::RL(o) => self.inst_RL(o),
      Instruction::RR(o) => self.inst_RR(o),
      Instruction::RLA => self.inst_RLA(),
      Instruction::SRL(o) => self.inst_SRL(o),
      Instruction::SWAP(v) => self.inst_SWAP(v),

      Instruction::ADC(o1, o2) => self.inst_ADC(o1, o2),
      Instruction::ADD8(o1, o2) => self.inst_ADD8(o1, o2),
      Instruction::ADD16(o1, o2) => self.inst_ADD16(o1, o2),
      Instruction::AND(o) => self.inst_AND(o),
      Instruction::CALL_cc(o1, o2) => self.inst_CALL_cc(o1, o2),
      Instruction::CALL(o) => self.inst_CALL(o),
      Instruction::CP(o) => self.inst_CP(o),
      Instruction::DEC8(o) => self.inst_DEC8(o),
      Instruction::DEC16(o) => self.inst_DEC16(o),
      Instruction::DI => self.inst_DI(),
      Instruction::EI => self.inst_EI(),
      Instruction::HALT => self.inst_HALT(),
      Instruction::INC8(o) => self.inst_INC8(o),
      Instruction::INC16(o) => self.inst_INC16(o),
      Instruction::JP(o) => self.inst_JP(o),
      Instruction::JP_cc(o1, o2) => self.inst_JP_cc(o1, o2),
      Instruction::JR_cc_e(cc, e) => self.inst_JR_cc_e(cc, e),
      Instruction::JR_e(e) => self.inst_JR_e(e),
      Instruction::LD_·0xFF00C·_A => self.inst_LD_·0xFF00C·_A(),
      Instruction::LD_·0xFF00n·_A(n) => self.inst_LD_·0xFF00n·_A(n),
      Instruction::LD_·DE·_A => self.inst_LD_·DE·_A(),
      Instruction::LD_·HL·_n(n) => self.inst_LD_·HL·_n(n),
      Instruction::LD_·HL·_r(r) => self.inst_LD_·HL·_r(r),
      Instruction::LD_·nn·_A(nn) => self.inst_LD_·nn·_A(nn),
      Instruction::LD_·nn·_SP(nn) => self.inst_LD_·nn·_SP(nn),
      Instruction::LD_A_·BC· => self.inst_LD_A_·BC·(),
      Instruction::LD_A_·DE· => self.inst_LD_A_·DE·(),
      Instruction::LD_A_·nn·(nn) => self.inst_LD_A_·nn·(nn),
      Instruction::LD_A_·0xFF00n·(n) => self.inst_LD_A_·0xFF00n·(n),
      Instruction::LD_dd_nn(dd, nn) => self.inst_LD_dd_nn(dd, nn),
      Instruction::LD_r_·HL·(r) => self.inst_LD_r_·HL·(r),
      Instruction::LD_r_n(r, n) => self.inst_LD_r_n(r, n),
      Instruction::LD_r_r(r1, r2) => self.inst_LD_r_r(r1, r2),
      Instruction::LDI_A_·HL· => self.inst_LDI_A_·HL·(),
      Instruction::LDD_·HL·_A => self.inst_LDD_·HL·_A(),
      Instruction::LDI_·HL·_A => self.inst_LDI_·HL·_A(),
      Instruction::OR_A_·HL· => self.inst_OR_A_·HL·(),
      Instruction::OR_r(r) => self.inst_OR_r(r),
      Instruction::POP_rr(rr) => self.inst_POP_rr(rr),
      Instruction::PUSH_rr(rr) => self.inst_PUSH_rr(rr),
      Instruction::RET => self.inst_RET(),
      Instruction::RET_cc(cc) => self.inst_RET_cc(cc),
      Instruction::RRA => self.inst_RRA(),
      Instruction::RST_t(t) => self.inst_RST_t(t),
      Instruction::SUB_n(n) => self.inst_SUB_n(n),
      Instruction::SUB_r(r) => self.inst_SUB_r(r),
      Instruction::NOP => self.inst_NOP(),
      Instruction::XOR_·HL· => self.inst_XOR_·HL·(),
      Instruction::XOR_n(n) => self.inst_XOR_n(n),
      Instruction::XOR_r(r) => self.inst_XOR_r(r),

      _ => panic!("instruction not implemented: {:?}\n{:?}", ins, self),
    };

    // self.clock_t += t;
  }

  fn push_word(&mut self, w: u16) {
    self.reg_sp -= 2;
    let sp = self.reg_sp;
    self.write_u16(sp, w);
  }

  fn pop_word(&mut self) -> u16 {
    let sp = self.reg_sp;
    let val = self.read_u16(sp);
    self.reg_sp += 2;
    val
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

  // RL r
  //   Opcode: 0xcb 0x17 | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15
  // RL (HL)
  //   Opcode: 0xcb 0x16
  // Page: 220
  #[allow(non_snake_case)]
  fn inst_RL(&mut self, o: Operand) {
    let mut val = self.read_operand_u8(o);
    let carry = self.read_flag(Flag::C);

    val <<= 1;

    if carry {
      val |= 1; // set bit 0 to 1
    } else {
      val &= !1; // set bit 0 to 0
    }

    self.write_operand_u8(o, val);
    self.write_flag(Flag::Z, val == 0);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, false);
    self.write_flag(Flag::C, val & (1 << 7) != 0);
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
  //   Opcode: 0xCB 0x37 | 0x30 | 0x31 | 0x32 | 0x33 | 0x34 | 0x35
  // SWAP (HL)
  //   Opcode: 0xCB 0x36
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

    let c = if self.read_flag(Flag::C) {
      1
    } else {
      0
    };

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
  fn inst_ADD16(&mut self, o1: Operand, o2: Operand) {
    let val1 = self.read_operand_u16(o1);
    let val2 = self.read_operand_u16(o2);

    let (result, carry) = val1.overflowing_add(val2);

    self.write_operand_u16(o1, result);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, (result ^ val1 ^ val2) & 0x1000 != 0);
    self.write_flag(Flag::C, carry);
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
      self.push_word(pc);
      self.reg_pc = nn;
    }
  }

  // CALL nn
  //   Opcode: 0xcd
  // Page: 273
  #[allow(non_snake_case)]
  fn inst_CALL(&mut self, o: Operand) {
    let nn = self.read_operand_u16(o);

    let pc = self.reg_pc;
    self.push_word(pc);
    self.reg_pc = nn;
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
  }

  // DI
  // Opcode: 0xf3
  // Page: 192
  #[allow(non_snake_case)]
  fn inst_DI(&mut self) {
    self.interrupt_master_enable = false;
  }

  // EI
  // Opcode: 0xfb
  // Page: 193
  #[allow(non_snake_case)]
  fn inst_EI(&mut self) {
    self.interrupt_master_enable = true;
  }

  // HALT
  // Opcode: 0x76
  #[allow(non_snake_case)]
  fn inst_HALT(&mut self) {
    self.halt = true;
    println!("halted @ {:#06x}!", self.reg_pc);
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
    }
  }

  // JR cc,e
  // Opcode: 000cc000
  // Page: 266
  // This is a superset of many different instructions:
  // JR NZ,e
  // JR Z,e
  // JR NC,e
  // JR C,e
  #[allow(non_snake_case)]
  fn inst_JR_cc_e(&mut self, cc: Flag, e: i8) {
    // signed argument
    if self.read_flag(cc) {
      // signed addition (can jump back)
      self.reg_pc = ((self.reg_pc as i16) + (e as i16)) as u16;
    }
  }

  // JR e
  // Opcode: 0x18
  // Page: 259
  #[allow(non_snake_case)]
  fn inst_JR_e(&mut self, e: i8) {
    // signed addition (can jump back)
    self.reg_pc = ((self.reg_pc as i16) + (e as i16)) as u16;
  }

  // LD (0xFF00+C),A
  // Opcode: 0xE2
  // Moved instruction.
  // Moved: RET PO -> LD (FF00+n),A
  #[allow(non_snake_case)]
  fn inst_LD_·0xFF00C·_A(&mut self) {
    let a = self.read_reg_u8(Reg::A);
    let c = self.read_reg_u8(Reg::C);
    self.write_u8(0xFF00 + c as u16, a);
  }

  // LD (0xFF00+n),A
  // Opcode: 0xE0 nn
  // Moved instruction.
  // Moved: JP PO,nn -> LD (FF00+C),A
  #[allow(non_snake_case)]
  fn inst_LD_·0xFF00n·_A(&mut self, n: u8) {
    let a = self.read_reg_u8(Reg::A);
    self.write_u8(0xFF00 + n as u16, a);
  }

  // LD (DE),A
  // Opcode: 0x12
  #[allow(non_snake_case)]
  fn inst_LD_·DE·_A(&mut self) {
    let de = self.read_reg_u16(Reg::DE);
    let a = self.read_reg_u8(Reg::A);
    self.write_u8(de, a);
  }

  // LD (HL),n
  // Opcode: 0x36
  // Page: 107
  #[allow(non_snake_case)]
  fn inst_LD_·HL·_n(&mut self, n: u8) {
    let hl = self.read_reg_u16(Reg::HL);
    self.write_u8(hl, n);
  }

  // LD (HL),r
  // Opcode: 01110rrr
  // Page: 104
  #[allow(non_snake_case)]
  fn inst_LD_·HL·_r(&mut self, r: Reg) {
    let hl = self.read_reg_u16(Reg::HL);
    let val = self.read_reg_u8(r);
    self.write_u8(hl, val);
  }

  // LD (nn),A
  // Opcode: 0xEA
  // Page: 115
  // Moved: JP PE,nn => LD (nn),A
  #[allow(non_snake_case)]
  fn inst_LD_·nn·_A(&mut self, nn: u16) {
    let val = self.read_reg_u8(Reg::A);
    self.write_u8(nn, val);
  }

  // LD (nn),SP
  // Opcode: 0x08
  // Page:
  #[allow(non_snake_case)]
  fn inst_LD_·nn·_SP(&mut self, nn: u16) {
    let sp = self.reg_sp;
    self.write_u16(nn, sp);
  }

  // LD A,(BC)
  // Opcode: 0x0A
  // Page: 110
  #[allow(non_snake_case)]
  fn inst_LD_A_·BC·(&mut self) {
    let bc = self.reg_bc;
    let val = self.read_u8(bc);
    self.write_reg_u8(Reg::A, val);
  }

  // LD A,(DE)
  // Opcode: 0x1A
  // Page: 111
  #[allow(non_snake_case)]
  fn inst_LD_A_·DE·(&mut self) {
    let de = self.reg_de;
    let val = self.read_u8(de);
    self.write_reg_u8(Reg::A, val);
  }

  // LD A,(nn)
  // Opcode: 0xFA
  // Page:
  #[allow(non_snake_case)]
  fn inst_LD_A_·nn·(&mut self, nn: u16) {
    let val = self.read_u8(nn);
    self.write_reg_u8(Reg::A, val);
  }

  // LD A,(0xFF00n)
  // Opcode: 0xF0
  // Moved: RET P -> LD A,(FF00+n)
  #[allow(non_snake_case)]
  fn inst_LD_A_·0xFF00n·(&mut self, n: u8) {
    let val = self.read_u8(0xFF00 + n as u16);
    self.write_reg_u8(Reg::A, val);
  }

  // LD dd,nn
  // Opcode: 00dd0001
  // Page: 120
  #[allow(non_snake_case)]
  fn inst_LD_dd_nn(&mut self, dd: Reg, nn: u16) {
    self.write_reg_u16(dd, nn);
  }

  // LD r,(HL)
  // Opcode: 01rrr110
  // Page: 101
  #[allow(non_snake_case)]
  fn inst_LD_r_·HL·(&mut self, r: Reg) {
    let hl = self.read_reg_u16(Reg::HL);
    let val = self.read_u8(hl);
    self.write_reg_u8(r, val);
  }

  // LD r,r
  // Opcode: 01_rrr_rrr
  // Page: 120
  #[allow(non_snake_case)]
  fn inst_LD_r_r(&mut self, r1: Reg, r2: Reg) {
    let val = self.read_reg_u8(r2);
    self.write_reg_u8(r1, val);
  }

  // LD r,n
  // Opcode: 00rrr110
  // Page: 100
  #[allow(non_snake_case)]
  fn inst_LD_r_n(&mut self, r: Reg, n: u8) {
    self.write_reg_u8(r, n);
  }

  // LDI (HL),A
  // Opcode: 0x32
  // Page: 149
  // Moved: LD HL,(nn) -> LDI A,(HL)
  #[allow(non_snake_case)]
  fn inst_LDI_A_·HL·(&mut self) {
    let hl = self.read_reg_u16(Reg::HL);
    let val = self.read_u8(hl);

    self.write_reg_u8(Reg::A, val);
    self.write_reg_u16(Reg::HL, hl + 1);
  }

  // LDD (HL),A
  // Opcode: 0x32
  // Page: 149
  // Moved: LD (nn),A -> LDD (HL),A
  #[allow(non_snake_case)]
  fn inst_LDD_·HL·_A(&mut self) {
    let hl = self.read_reg_u16(Reg::HL);
    let a = self.read_reg_u8(Reg::A);
    self.write_u8(hl, a);
    self.write_reg_u16(Reg::HL, hl - 1);
  }

  // LDI (HL),A
  // Opcode: 0x22
  // Page: 146
  // Moved: LD (nn),HL -> LDI (HL),A
  #[allow(non_snake_case)]
  fn inst_LDI_·HL·_A(&mut self) {
    let hl = self.read_reg_u16(Reg::HL);
    let a = self.read_reg_u8(Reg::A);
    self.write_u8(hl, a);
    self.write_reg_u16(Reg::HL, hl + 1);
  }

  // NOP
  // 0x00
  #[allow(non_snake_case)]
  fn inst_NOP(&self) {}

  // OR r
  // Opcode: 0xb6
  // Page: 172
  #[allow(non_snake_case)]
  fn inst_OR_A_·HL·(&mut self) {
    let hl = self.read_reg_u16(Reg::HL);
    let val = self.read_u8(hl);
    let result = self.read_reg_u8(Reg::A) | val;

    self.write_reg_u8(Reg::A, result);
    self.write_flag(Flag::Z, result == 0);
    self.write_flag(Flag::H, false);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::C, false);
  }

  // OR r
  // Opcode: 10110rrr
  // Page: 172
  #[allow(non_snake_case)]
  fn inst_OR_r(&mut self, r: Reg) {
    let val = self.read_reg_u8(r);
    let result = self.read_reg_u8(Reg::A) | val;

    self.write_reg_u8(Reg::A, result);
    self.write_flag(Flag::Z, result == 0);
    self.write_flag(Flag::H, false);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::C, false);
  }

  // POP rr
  // Opcode: 11rr0001
  // Page: 137
  #[allow(non_snake_case)]
  fn inst_POP_rr(&mut self, rr: Reg) {
    let val = self.pop_word();
    self.write_reg_u16(rr, val);
  }

  // PUSH rr
  // Opcode: 11rr0101
  // Page: 134
  #[allow(non_snake_case)]
  fn inst_PUSH_rr(&mut self, rr: Reg) {
    let val = self.read_reg_u16(rr);
    self.push_word(val);
  }

  // RET
  // Opcode: 0xC9
  // Page: 278
  #[allow(non_snake_case)]
  fn inst_RET(&mut self) {
    self.reg_pc = self.pop_word();
  }

  // RET cc
  // Opcode: 11ccc000
  // Page: 279
  #[allow(non_snake_case)]
  fn inst_RET_cc(&mut self, cc: Flag) {
    if self.read_flag(cc) {
      self.reg_pc = self.pop_word();
    }
  }

  // RRA
  // Opcode: 0x1f
  // Page: 211
  #[allow(non_snake_case)]
  fn inst_RRA(&mut self) {
    let mut val = self.read_reg_u8(Reg::A);
    let prev_carry = self.read_flag(Flag::C);
    let carry = val & 1 != 0;

    val >>= 1;

    if prev_carry {
      val |= 0b10000000; // set bit 7 to 1
    } else {
      val &= !0b10000000; // set bit 7 to 0
    }

    self.write_reg_u8(Reg::A, val);
    self.write_flag(Flag::Z, false);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, false);
    self.write_flag(Flag::C, carry);
  }

  // RST n
  // Opcode: 11ttt111
  // Page: 285
  #[allow(non_snake_case)]
  fn inst_RST_t(&mut self, t: u8) {
    let pc = self.reg_pc;
    self.push_word(pc);
    self.reg_pc = t as u16 * 0x08;
  }

  // SUB n
  // Opcode: 0xd6
  // Page: 166
  #[allow(non_snake_case)]
  fn inst_SUB_n(&mut self, n: u8) {
    let a = self.read_reg_u8(Reg::A);
    let (result, carry) = a.overflowing_sub(n);

    self.write_reg_u8(Reg::A, result);
    self.write_flag(Flag::Z, result == 0);
    self.write_flag(Flag::N, true);
    self.write_flag(Flag::H, a & 0x0F < n & 0x0F);
    self.write_flag(Flag::C, carry);
  }

  // SUB r
  // Opcode: 10010rrr
  // Page: 166
  #[allow(non_snake_case)]
  fn inst_SUB_r(&mut self, r: Reg) {
    let a = self.read_reg_u8(Reg::A);
    let val = self.read_reg_u8(r);
    let (result, carry) = a.overflowing_sub(val);

    self.write_reg_u8(Reg::A, result);
    self.write_flag(Flag::Z, result == 0);
    self.write_flag(Flag::N, true);
    self.write_flag(Flag::H, a & 0x0F < val & 0x0F);
    self.write_flag(Flag::C, carry);
  }

  // XOR (HL)
  // Opcode: 0xae
  // Page: 174
  #[allow(non_snake_case)]
  fn inst_XOR_·HL·(&mut self) {
    let hl = self.read_reg_u16(Reg::HL);
    let val = self.read_u8(hl);
    let mut a = self.read_reg_u8(Reg::A);
    a ^= val;
    self.write_reg_u8(Reg::A, a);

    self.write_flag(Flag::Z, a == 0);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, false);
    self.write_flag(Flag::C, false);
  }

  // XOR n
  // Opcode: 0xee
  // Page: 174
  // This instruction is a subset of the defined instruction in the pdf.
  // The superset instruction is XOR s, where s can be r, n, (HL), (IX+d)
  // or (IY+d).
  #[allow(non_snake_case)]
  fn inst_XOR_n(&mut self, n: u8) {
    let mut a = self.read_reg_u8(Reg::A);
    a ^= n;
    self.write_reg_u8(Reg::A, a);

    self.write_flag(Flag::Z, a == 0);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, false);
    self.write_flag(Flag::C, false);
  }

  // XOR r
  // Opcode: 10110rrr
  // Page: 174
  // This instruction is a subset of the defined instruction in the pdf.
  // The superset instruction is XOR s, where s can be r, n, (HL), (IX+d)
  // or (IY+d).
  #[allow(non_snake_case)]
  fn inst_XOR_r(&mut self, r: Reg) {
    let val = self.read_reg_u8(r);
    let mut a = self.read_reg_u8(Reg::A);
    a ^= val;
    self.write_reg_u8(Reg::A, a);

    self.write_flag(Flag::Z, a == 0);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, false);
    self.write_flag(Flag::C, false);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use super::super::reg::Reg;
  use super::super::flag::Flag;
  use super::super::system::SystemCtrl;
  use super::super::mem::MemoryIo;
  use std::io::Read;
  use std::fs::File;
  use std;

  use yaml_rust::YamlLoader;
  use yaml_rust::yaml::Yaml;

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

  #[test]
  fn test_runner() {
    // let s = include_str!("../testdata/cpu.yaml");
    let mut f = File::open("testdata/cpu.yaml").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let docs = YamlLoader::load_from_str(&s).unwrap();
    let doc = &docs[0];

    for (k, _) in doc.as_hash().unwrap() {
      let test_name = k.as_str().unwrap();
      let unit = &doc[test_name];
      let setup = &unit["setup"];
      let test = &unit["test"];

      let mut c = testcpu();
      for (setup_k, setup_v) in setup.as_hash().unwrap() {
        match setup_k.as_str().unwrap() {
          "A" => c.write_reg_u8(Reg::A, setup_v.as_i64().unwrap() as u8),
          "F" => c.write_reg_u8(Reg::F, setup_v.as_i64().unwrap() as u8),
          "B" => c.write_reg_u8(Reg::B, setup_v.as_i64().unwrap() as u8),
          "C" => c.write_reg_u8(Reg::C, setup_v.as_i64().unwrap() as u8),
          "D" => c.write_reg_u8(Reg::D, setup_v.as_i64().unwrap() as u8),
          "E" => c.write_reg_u8(Reg::E, setup_v.as_i64().unwrap() as u8),
          "H" => c.write_reg_u8(Reg::H, setup_v.as_i64().unwrap() as u8),
          "L" => c.write_reg_u8(Reg::L, setup_v.as_i64().unwrap() as u8),
          "SP" => c.reg_sp = setup_v.as_i64().unwrap() as u16,
          "PC" => c.reg_pc = setup_v.as_i64().unwrap() as u16,
          "mem" => {
            match setup_v {
              &Yaml::Array(ref a) => {
                let mut count = 0;
                for x in a {
                  c.system.write_u8(count, x.as_i64().unwrap() as u8).unwrap();
                  count += 1;
                }
              }
              _ => panic!("unknown mem value"),
            };
          }
          _ => panic!("unknown key in setup"),
        };
      }

      c.step();

      for (test_k, test_v) in test.as_hash().unwrap() {
        match test_k.as_str().unwrap() {
          "A" => {
            let v1 = c.read_reg_u8(Reg::A);
            let v2 = test_v.as_i64().unwrap() as u8;
            assert!(v1 == v2,
                    "\n{0}:\n A:\n  Got:      {1:#04x} [{1:08b}],\n  Expected: {2:#04x} [{2:08b}]",
                    test_name,
                    v1,
                    v2);
          }
          "F" => {
            let v1 = c.read_reg_u8(Reg::F);
            let v2 = test_v.as_i64().unwrap() as u8;
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
          }
          "B" => {
            let v1 = c.read_reg_u8(Reg::B);
            let v2 = test_v.as_i64().unwrap() as u8;
            assert!(v1 == v2,
                    "\n{0}:\n B:\n  Got:      {1:#04x} [{1:08b}],\n  Expected: {2:#04x} [{2:08b}]",
                    test_name,
                    v1,
                    v2);
          }
          "C" => {
            let v1 = c.read_reg_u8(Reg::C);
            let v2 = test_v.as_i64().unwrap() as u8;
            assert!(v1 == v2,
                    "\n{0}:\n C:\n  Got:      {1:#04x} [{1:08b}],\n  Expected: {2:#04x} [{2:08b}]",
                    test_name,
                    v1,
                    v2);
          }
          "D" => {
            let v1 = c.read_reg_u8(Reg::D);
            let v2 = test_v.as_i64().unwrap() as u8;
            assert!(v1 == v2,
                    "\n{0}:\n D:\n  Got:      {1:#04x} [{1:08b}],\n  Expected: {2:#04x} [{2:08b}]",
                    test_name,
                    v1,
                    v2);
          }
          "E" => {
            let v1 = c.read_reg_u8(Reg::E);
            let v2 = test_v.as_i64().unwrap() as u8;
            assert!(v1 == v2,
                    "\n{0}:\n E:\n  Got:      {1:#04x} [{1:08b}],\n  Expected: {2:#04x} [{2:08b}]",
                    test_name,
                    v1,
                    v2);
          }
          "H" => {
            let v1 = c.read_reg_u8(Reg::H);
            let v2 = test_v.as_i64().unwrap() as u8;
            assert!(v1 == v2,
                    "\n{0}:\n H:\n  Got:      {1:#04x} [{1:08b}],\n  Expected: {2:#04x} [{2:08b}]",
                    test_name,
                    v1,
                    v2);
          }
          "L" => {
            let v1 = c.read_reg_u8(Reg::L);
            let v2 = test_v.as_i64().unwrap() as u8;
            assert!(v1 == v2,
                    "\n{0}:\n L:\n  Got:      {1:#04x} [{1:08b}],\n  Expected: {2:#04x} [{2:08b}]",
                    test_name,
                    v1,
                    v2);
          }
          "SP" => {
            let v1 = c.reg_sp;
            let v2 = test_v.as_i64().unwrap() as u16;
            assert!(v1 == v2,
                    "\n{0}:\n SP:\n  Got:      {1:#04x} [{1:08b}],\n  Expected: {2:#04x} \
                     [{2:08b}]",
                    test_name,
                    v1,
                    v2);
          }
          "PC" => {
            let v1 = c.reg_pc;
            let v2 = test_v.as_i64().unwrap() as u16;
            assert!(v1 == v2,
                    "\n{0}:\n PC:\n  Got:      {1:#04x} [{1:08b}],\n  Expected: {2:#04x} \
                     [{2:08b}]",
                    test_name,
                    v1,
                    v2);
          }
          "mem" => {
            match test_v {
              &Yaml::Array(ref a) => {
                let mut count = 0;
                let mut v1 = vec![];
                let mut v2 = vec![];
                for x in a {
                  let m1 = c.read_u8(count);
                  let m2 = x.as_i64().unwrap() as u8;
                  v1.push(m1);
                  v2.push(m2);
                  count += 1;
                }

                assert!(v1 == v2,
                        "\n{0}:\n mem:\n  Got:      {1:#04x},\n  Expected: {2:#04x}",
                        test_name,
                        HexVec(v1),
                        HexVec(v2));
              }
              _ => panic!("unknown mem value"),
            };
          }
          _ => panic!("unknown key in setup"),
        }
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
