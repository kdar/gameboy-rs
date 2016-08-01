use std::fmt;
use std::default::Default;
use std::cmp::PartialEq;
use std::rc::Rc;
use std::cell::RefCell;

use super::mem;
use super::video;
use super::reg::Reg;
use super::flag::Flag;
use super::disassembler::Instruction;
use super::disassembler::Disassembler;

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

  mem: Box<mem::Memory>,
  video: Rc<RefCell<video::Video>>,
  disasm: Disassembler,
}

impl PartialEq for Cpu {
  fn eq(&self, x: &Cpu) -> bool {
    self.reg_af == x.reg_af && self.reg_bc == x.reg_bc && self.reg_de == x.reg_de &&
    self.reg_hl == x.reg_hl && self.reg_sp == x.reg_sp && self.reg_pc == x.reg_pc &&
    self.clock_t == x.clock_t
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
      mem: Box::new(mem::Mem::new()),
      video: Rc::new(RefCell::new(video::Video::new())),
      disasm: Disassembler::new(),
    }
  }
}

impl fmt::Debug for Cpu {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    try!(write!(f, "\nA:       {0:#04x} [{0:08b}]", high_byte(self.reg_af)));
    try!(write!(f, "\nF:       {0:#04x} [{0:08b}]", low_byte(self.reg_af)));
    try!(write!(f, "\nB:       {0:#04x} [{0:08b}]", high_byte(self.reg_bc)));
    try!(write!(f, "\nC:       {0:#04x} [{0:08b}]", low_byte(self.reg_bc)));
    try!(write!(f, "\nD:       {0:#04x} [{0:08b}]", high_byte(self.reg_de)));
    try!(write!(f, "\nE:       {0:#04x} [{0:08b}]", low_byte(self.reg_de)));
    try!(write!(f, "\nH:       {0:#04x} [{0:08b}]", high_byte(self.reg_hl)));
    try!(write!(f, "\nL:       {0:#04x} [{0:08b}]", low_byte(self.reg_hl)));
    try!(write!(f, "\nSP:      {0:#06x} [{0:016b}]", self.reg_sp));
    try!(write!(f, "\nPC:      {0:#06x} [{0:016b}]", self.reg_pc));
    try!(write!(f, "\nClock T: {}", self.clock_t));
    write!(f, "\n")
  }
}

impl Cpu {
  pub fn new() -> Cpu {
    let mut c = Cpu::default();
    c.mem.map(mem::VIDEO_RAM_START, mem::VIDEO_RAM_END, c.video.clone());
    c.mem.map(mem::SPRITE_TABLE_START,
              mem::SPRITE_TABLE_END,
              c.video.clone());
    c.mem.map(mem::VIDEO_CONTROL_START,
              mem::VIDEO_CONTROL_END,
              c.video.clone());
    c
  }

  pub fn set_boot_rom(&mut self, rom: Box<[u8]>) {
    self.mem.set_boot_rom(rom);
  }

  pub fn set_cart_rom(&mut self, rom: Box<[u8]>) {
    self.mem.set_cart_rom(rom);
  }

  pub fn read_reg_byte(&self, register: Reg) -> u8 {
    match register {
      Reg::B => high_byte(self.reg_bc),
      Reg::C => low_byte(self.reg_bc),
      Reg::D => high_byte(self.reg_de),
      Reg::E => low_byte(self.reg_de),
      Reg::H => high_byte(self.reg_hl),
      Reg::L => low_byte(self.reg_hl),
      Reg::A => high_byte(self.reg_af),
      Reg::F => low_byte(self.reg_af),
      _ => panic!("read_byte_register unknown register: {:?}", register),
    }
  }

  fn read_reg_word(&mut self, register: Reg) -> u16 {
    match register {
      Reg::BC => self.reg_bc,
      Reg::DE => self.reg_de,
      Reg::HL => self.reg_hl,
      Reg::AF => self.reg_af,
      Reg::SP => self.reg_sp,
      Reg::PC => self.reg_pc,
      _ => panic!("read_reg_word unknown register: {:?}", register),
    }
  }

  fn write_reg_word(&mut self, register: Reg, value: u16) {
    match register {
      Reg::BC => self.reg_bc = value,
      Reg::DE => self.reg_de = value,
      Reg::HL => self.reg_hl = value,
      Reg::AF => self.reg_af = value,
      Reg::SP => self.reg_sp = value,
      Reg::PC => self.reg_pc = value,
      _ => panic!("write_reg_word unknown register: {:?}", register),
    }
  }

  pub fn write_reg_byte(&mut self, register: Reg, value: u8) {
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
      _ => panic!("write_reg_byte unknown register: {:?}", register),
    }
  }

  fn read_pc_byte(&mut self) -> u8 {
    let d = self.mem.read_byte(self.reg_pc);
    self.reg_pc += 1;
    match d {
      Some(v) => v,
      None => panic!("read_by_byte: could not read byte at {:#04x}", self.reg_pc),
    }
  }

  fn read_pc_word(&mut self) -> u16 {
    let d = self.mem.read_word(self.reg_pc);
    self.reg_pc += 2;
    match d {
      Some(v) => v,
      None => panic!("read_by_byte: could not read byte at {:#04x}", self.reg_pc),
    }
  }

  fn write_flag(&mut self, flag: Flag, mut value: bool) {
    let mut d = self.read_reg_byte(Reg::F);

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

    self.write_reg_byte(Reg::F, d);
  }

  fn read_flag(&self, flag: Flag) -> bool {
    let d = self.read_reg_byte(Reg::F);

    match flag {
      Flag::Z => 0b10000000 & d > 0,
      Flag::N => 0b01000000 & d > 0,
      Flag::H => 0b00100000 & d > 0,
      Flag::C => 0b00010000 & d > 0,
      Flag::NZ => 0b10000000 & d == 0,
      Flag::NC => 0b00010000 & d == 0,
    }
  }

  // pub fn reset(&mut self) {
  //   self.reg_gpr = [0; NUM_GPR];
  //   self.reg_sp = 0;
  //   self.reg_pc = 0;
  //   self.cycles = 0;
  // }

  pub fn step(&mut self) -> (Instruction, u16) {
    if let Some((inst, inc)) = self.disasm.at(&self.mem, self.reg_pc) {
      self.reg_pc += inc;
      self.execute_instruction(inst.clone());

      // Once we get to this point, we are no longer booting.
      if self.reg_pc > 0xFF {
        self.mem.set_booting(false);
      }

      return (inst, self.reg_pc);
    }

    (Instruction::Invalid, self.reg_pc)
  }

  fn execute_instruction(&mut self, ins: Instruction) {
    let t = match ins {
      Instruction::Invalid => panic!("execute_instruction: Invalid instruction encountered"),
      Instruction::Data(_) => (0),

      // 0xCB instructions
      Instruction::BIT_b_r(b, r) => self.inst_BIT_b_r(b, r),
      Instruction::RL_r(r) => self.inst_RL_r(r),
      Instruction::RLA => self.inst_RLA(),

      Instruction::ADD_A_·HL· => self.inst_ADD_A_·HL·(),
      Instruction::CALL_nn(nn) => self.inst_CALL_nn(nn),
      Instruction::CP_·HL· => self.inst_CP_·HL·(),
      Instruction::CP_n(n) => self.inst_CP_n(n),
      Instruction::DEC_r(r) => self.inst_DEC_r(r),
      Instruction::INC_r(r) => self.inst_INC_r(r),
      Instruction::INC_rr(ss) => self.inst_INC_rr(ss),
      Instruction::JR_cc_e(cc, e) => self.inst_JR_cc_e(cc, e),
      Instruction::JR_e(e) => self.inst_JR_e(e),
      Instruction::LD_·0xFF00C·_A => self.inst_LD_·0xFF00C·_A(),
      Instruction::LD_·0xFF00n·_A(n) => self.inst_LD_·0xFF00n·_A(n),
      Instruction::LD_·HL·_r(r) => self.inst_LD_·HL·_r(r),
      Instruction::LD_·nn·_A(nn) => self.inst_LD_·nn·_A(nn),
      Instruction::LD_A_·DE· => self.inst_LD_A_·DE·(),
      Instruction::LD_A_·0xFF00n·(n) => self.inst_LD_A_·0xFF00n·(n),
      Instruction::LD_dd_nn(dd, nn) => self.inst_LD_dd_nn(dd, nn),
      Instruction::LD_r_n(r, n) => self.inst_LD_r_n(r, n),
      Instruction::LD_r_r(r1, r2) => self.inst_LD_r_r(r1, r2),
      Instruction::LDD_·HL·_A => self.inst_LDD_·HL·_A(),
      Instruction::LDI_·HL·_A => self.inst_LDI_·HL·_A(),
      Instruction::POP_rr(rr) => self.inst_POP_rr(rr),
      Instruction::PUSH_rr(rr) => self.inst_PUSH_rr(rr),
      Instruction::RET => self.inst_RET(),
      Instruction::SUB_r(r) => self.inst_SUB_r(r),
      Instruction::NOP => self.inst_NOP(),
      Instruction::XOR_r(r) => self.inst_XOR_r(r),
      // _ => panic!("instruction not implemented: {:?}", ins),
    };

    self.clock_t += t;
  }

  // BIT b,r
  // Opcode: 0xCB 01bbbrrr
  // Page: 242
  #[allow(non_snake_case)]
  fn inst_BIT_b_r(&mut self, b: u8, r: Reg) -> u32 {
    let d = self.read_reg_byte(r);

    self.write_flag(Flag::Z, d & (1 << b) == 0);

    self.write_flag(Flag::H, true);
    self.write_flag(Flag::N, false);

    8
  }

  // RL r
  // Opcode: 0xCB 000010xxx
  // Page: 220
  #[allow(non_snake_case)]
  fn inst_RL_r(&mut self, r: Reg) -> u32 {
    let mut d = self.read_reg_byte(r);

    let carry = self.read_flag(Flag::C);

    self.write_flag(Flag::C, d & (1 << 7) > 0);

    d <<= 1;

    if carry {
      d |= 1;
    } else {
      d &= !1;
    }

    self.write_reg_byte(r, d);

    self.write_flag(Flag::Z, d == 0);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, false);

    8
  }

  // RLA
  // Opcode: 0xCB 0x17
  // Page: 209
  #[allow(non_snake_case)]
  fn inst_RLA(&mut self) -> u32 {
    let mut d = self.read_reg_byte(Reg::A);

    let carry = self.read_flag(Flag::C);

    self.write_flag(Flag::C, d & (1 << 7) > 0);

    d <<= 1;

    if carry {
      d |= 1;
    } else {
      d &= !1;
    }

    self.write_reg_byte(Reg::A, d);

    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, false);

    4
  }

  // ADD A,(HL)
  // Opcode: 0x86
  // Page: 161
  #[allow(non_snake_case)]
  fn inst_ADD_A_·HL·(&mut self) -> u32 {
    // byte A = GetHighByte(m_AF);
    // byte HL = m_MMU->Read(m_HL);
    // SetHighByte(&m_AF, AddByte(A, HL));

    let a = self.read_reg_byte(Reg::A);
    let d = match self.mem.read_byte(self.reg_hl) {
      Some(v) => v,
      None => {
        panic!("inst_ADD_A_·HL·: could not read (HL) byte (memory address {:#04x})",
               self.reg_hl);
      }
    };

    let result = a + d;
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, ((result ^ a ^ d) & 0x10) > 0);
    self.write_flag(Flag::C, result < a);

    8
  }

  // CALL nn
  // Opcode: 0xCD
  // Page: 273
  #[allow(non_snake_case)]
  fn inst_CALL_nn(&mut self, nn: u16) -> u32 {
    self.reg_sp -= 2;
    self.mem.write_word(self.reg_sp, self.reg_pc);
    self.reg_pc = nn;
    24
  }

  // CP (HL)
  // Opcode: 0xBE
  // Page: 176
  #[allow(non_snake_case)]
  fn inst_CP_·HL·(&mut self) -> u32 {
    let d = match self.mem.read_byte(self.reg_hl) {
      Some(v) => v,
      None => {
        panic!("inst_CP_·HL·: could not read (HL) byte (memory address {:#04x})",
               self.reg_hl);
      }
    };

    let a = self.read_reg_byte(Reg::A);
    let result = a - d;

    self.write_flag(Flag::Z, result == 0);
    self.write_flag(Flag::N, false);
    self.write_flag(Flag::H, a & 0x0F < d & 0x0F);
    self.write_flag(Flag::C, a & 0xFF < d & 0xFF);

    8
  }

  // CP n
  // Opcode: 0xFE
  // Page: 176
  #[allow(non_snake_case)]
  fn inst_CP_n(&mut self, n: u8) -> u32 {
    let a = self.read_reg_byte(Reg::A);
    let result = (a as i8 - n as i8) as u8;

    self.write_flag(Flag::Z, result == 0);

    // Set the carry flag if the A register is less than n.
    // (for the full value).
    self.write_flag(Flag::C, a & 0xFF < n & 0xFF);

    // Set the half carry flag if half of register A is less than
    // half of n.
    self.write_flag(Flag::H, a & 0x0F < n & 0x0F);

    self.write_flag(Flag::N, true);

    4
  }

  // DEC r
  // Opcode: 00rrr101
  // Page: 182
  #[allow(non_snake_case)]
  fn inst_DEC_r(&mut self, r: Reg) -> u32 {
    let d = self.read_reg_byte(r);
    let newd = d - 1;
    self.write_reg_byte(r, newd);

    self.write_flag(Flag::H, (newd ^ 0x01 ^ d) & 0x10 > 0);
    self.write_flag(Flag::Z, newd == 0);

    self.write_flag(Flag::N, false);

    4
  }

  // INC r
  // Opcode: 00rrr100
  // Page: 178
  #[allow(non_snake_case)]
  fn inst_INC_r(&mut self, r: Reg) -> u32 {
    let d = self.read_reg_byte(r);
    let newd = d + 1;
    self.write_reg_byte(r, newd);

    self.write_flag(Flag::H, (d & 0xF + 1 & 0xF) & 0x10 > 0);
    self.write_flag(Flag::Z, newd == 0);

    self.write_flag(Flag::N, false);

    4
  }

  // INC rr
  // Opcode: 00ss0011
  // Page: 202
  // Originally called INC ss
  #[allow(non_snake_case)]
  fn inst_INC_rr(&mut self, ss: Reg) -> u32 {
    let mut d = self.read_reg_word(ss);
    d += 1;
    self.write_reg_word(ss, d);
    4
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
  fn inst_JR_cc_e(&mut self, cc: Flag, e: i8) -> u32 {
    // signed argument
    if self.read_flag(cc) {
      // signed addition (can jump back)
      self.reg_pc = ((self.reg_pc as i16) + (e as i16)) as u16;
      12
    } else {
      8
    }
  }

  // JR e
  // Opcode: 0x18
  // Page: 259
  #[allow(non_snake_case)]
  fn inst_JR_e(&mut self, e: i8) -> u32 {
    // signed addition (can jump back)
    self.reg_pc = ((self.reg_pc as i16) + (e as i16)) as u16;
    12
  }

  // LD (0xFF00+C),A
  // Opcode: 0xE2
  // Moved instruction.
  // Moved: RET PO -> LD (FF00+n),A
  #[allow(non_snake_case)]
  fn inst_LD_·0xFF00C·_A(&mut self) -> u32 {
    let a = self.read_reg_byte(Reg::A);
    let c = self.read_reg_byte(Reg::C);
    self.mem.write_byte(0xFF00 + c as u16, a);
    8
  }

  // LD (0xFF00+n),A
  // Opcode: 0xE0 nn
  // Moved instruction.
  // Moved: JP PO,nn -> LD (FF00+C),A
  #[allow(non_snake_case)]
  fn inst_LD_·0xFF00n·_A(&mut self, n: u8) -> u32 {
    let a = self.read_reg_byte(Reg::A);
    self.mem.write_byte(0xFF00 + n as u16, a);
    12
  }

  // LD (HL),r
  // Opcode: 01110rrr
  // Page: 104
  #[allow(non_snake_case)]
  fn inst_LD_·HL·_r(&mut self, reg: Reg) -> u32 {
    let hl = self.reg_hl;
    let a = self.read_reg_byte(reg);
    self.mem.write_byte(hl, a);
    8
  }

  // LD (nn),A
  // Opcode: 0xEA
  // Page: 115
  // Moved: JP PE,nn => LD (nn),A
  #[allow(non_snake_case)]
  fn inst_LD_·nn·_A(&mut self, nn: u16) -> u32 {
    let d = self.read_reg_byte(Reg::A);
    self.mem.write_byte(nn, d);
    16
  }

  // LD A,(DE)
  // Opcode: 0x1A
  // Page: 111
  #[allow(non_snake_case)]
  fn inst_LD_A_·DE·(&mut self) -> u32 {
    let val = match self.mem.read_byte(self.reg_de) {
      Some(v) => v,
      None => {
        panic!("inst_LD_A_·DE·: could not read (DE) byte (memory address {:#04x})",
               self.reg_de);
      }
    };
    self.write_reg_byte(Reg::A, val);
    8
  }

  // LD A,(0xFF00n)
  // Opcode: 0xF0
  // Moved: RET P -> LD A,(FF00+n)
  #[allow(non_snake_case)]
  fn inst_LD_A_·0xFF00n·(&mut self, n: u8) -> u32 {
    let d = match self.mem.read_byte(0xFF00 + n as u16) {
      Some(v) => v,
      None => {
        panic!("inst_LD_A_·0xFF00n·: could not read 0xFF00+n (memory address: {:#04x})",
               0xFF00 + n as u16);
      }
    };

    self.write_reg_byte(Reg::A, d);
    12
  }

  // LD dd,nn
  // Opcode: 00dd0001
  // Page: 120
  #[allow(non_snake_case)]
  fn inst_LD_dd_nn(&mut self, dd: Reg, nn: u16) -> u32 {
    self.write_reg_word(dd, nn);
    12
  }

  // LD r,r
  // Opcode: 01_rrr_rrr
  // Page: 120
  #[allow(non_snake_case)]
  fn inst_LD_r_r(&mut self, r1: Reg, r2: Reg) -> u32 {
    let tmp = self.read_reg_byte(r2);
    self.write_reg_byte(r1, tmp);
    4
  }

  // LD r,n
  // Opcode: 00rrr110
  // Page: 100
  #[allow(non_snake_case)]
  fn inst_LD_r_n(&mut self, r: Reg, n: u8) -> u32 {
    self.write_reg_byte(r, n);
    8
  }

  // LDD (HL),A
  // Opcode: 0x32
  // Page: 149
  // Moved: LD (nn),A -> LDD (HL),A
  #[allow(non_snake_case)]
  fn inst_LDD_·HL·_A(&mut self) -> u32 {
    let hl = self.reg_hl;
    let a = self.read_reg_byte(Reg::A);
    self.mem.write_byte(hl, a);
    self.reg_hl -= 1;

    self.write_flag(Flag::H, false);
    self.write_flag(Flag::N, false);

    8
  }

  // LDI (HL),A
  // Opcode: 0x22
  // Page: 146
  // Moved: LD (nn),HL -> LDI (HL),A
  #[allow(non_snake_case)]
  fn inst_LDI_·HL·_A(&mut self) -> u32 {
    let hl = self.reg_hl;
    let a = self.read_reg_byte(Reg::A);
    self.mem.write_byte(hl, a);
    self.reg_hl += 1;

    self.write_flag(Flag::H, false);
    self.write_flag(Flag::N, false);

    8
  }

  // NOP
  // 0x00
  #[allow(non_snake_case)]
  fn inst_NOP(&self) -> u32 {
    4
  }

  // POP rr
  // Opcode: 11rr0001
  // Page: 137
  #[allow(non_snake_case)]
  fn inst_POP_rr(&mut self, rr: Reg) -> u32 {
    let d = match self.mem.read_word(self.reg_sp) {
      Some(v) => v,
      None => {
        panic!("inst_POP_rr: could not read (SP) byte (memory address {:#04x})",
               self.reg_sp);
      }
    };

    self.write_reg_word(rr, d);
    self.reg_sp += 2;
    12
  }

  // PUSH rr
  // Opcode: 11rr0101
  // Page: 134
  #[allow(non_snake_case)]
  fn inst_PUSH_rr(&mut self, rr: Reg) -> u32 {
    let d = self.read_reg_word(rr);
    self.reg_sp -= 2;
    self.mem.write_word(self.reg_sp, d);
    16
  }

  // RET
  // Opcode: 0xC9
  // Page: 278
  #[allow(non_snake_case)]
  fn inst_RET(&mut self) -> u32 {
    let d = match self.mem.read_word(self.reg_sp) {
      Some(v) => v,
      None => {
        panic!("inst_RET: could not read (SP) byte (memory address {:#04x})",
               self.reg_sp);
      }
    };
    self.reg_pc = d;
    self.reg_sp += 2;
    16
  }

  // SUB r
  // Opcode: 10010rrr
  // Page: 166
  fn inst_SUB_r(&mut self, r: Reg) -> u32 {
    let a = self.read_reg_byte(Reg::A);
    let d = self.read_reg_byte(r);
    let result = a - d;

    self.write_flag(Flag::Z, result == 0);
    self.write_flag(Flag::N, false);

    self.write_flag(Flag::H, a & 0x0F < d & 0x0F);
    self.write_flag(Flag::C, a & 0xFF < d & 0xFF);

    4
  }

  // XOR r
  // Opcode: 10110rrr
  // Page: 174
  // This instruction is a subset of the defined instruction in the pdf.
  // The superset instruction is XOR s, where s can be r, n, (HL), (IX+d)
  // or (IY+d).
  #[allow(non_snake_case)]
  fn inst_XOR_r(&mut self, register: Reg) -> u32 {
    let register = self.read_reg_byte(register);
    let mut accumulator = self.read_reg_byte(Reg::A);
    accumulator = accumulator ^ register;
    self.write_reg_byte(Reg::A, accumulator);

    self.write_flag(Flag::Z, accumulator == 0);

    self.write_flag(Flag::N, false);
    self.write_flag(Flag::C, false);

    4
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use super::super::reg::Reg;
  use super::super::flag::Flag;
  use super::super::disassembler::Instruction;
  use difference::{self, Difference};
  use std::io::Write;

  #[test]
  fn test_write_read_reg_byte() {
    let mut c = Cpu::new();

    c.write_reg_byte(Reg::A, 0b01011010);
    assert!(c.reg_af == 0b01011010_00000000);
    assert!(c.read_reg_byte(Reg::A) == 0b01011010);
    c.write_reg_byte(Reg::F, 0b11011010);
    assert!(c.reg_af == 0b01011010_11011010);
    assert!(c.read_reg_byte(Reg::F) == 0b11011010);

    c.write_reg_byte(Reg::B, 0b01011010);
    assert!(c.reg_bc == 0b01011010_00000000);
    assert!(c.read_reg_byte(Reg::B) == 0b01011010);
    c.write_reg_byte(Reg::C, 0b11011010);
    assert!(c.reg_bc == 0b01011010_11011010);
    assert!(c.read_reg_byte(Reg::C) == 0b11011010);

    c.write_reg_byte(Reg::D, 0b01011010);
    assert!(c.reg_de == 0b01011010_00000000);
    assert!(c.read_reg_byte(Reg::D) == 0b01011010);
    c.write_reg_byte(Reg::E, 0b11011010);
    assert!(c.reg_de == 0b01011010_11011010);
    assert!(c.read_reg_byte(Reg::E) == 0b11011010);

    c.write_reg_byte(Reg::H, 0b01011010);
    assert!(c.reg_hl == 0b01011010_00000000);
    assert!(c.read_reg_byte(Reg::H) == 0b01011010);
    c.write_reg_byte(Reg::L, 0b11011010);
    assert!(c.reg_hl == 0b01011010_11011010);
    assert!(c.read_reg_byte(Reg::L) == 0b11011010);
  }

  #[test]
  fn test_write_read_flag() {
    let mut c = Cpu::new();
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
  }

  fn cpu_diff(c1: &Cpu, c2: &Cpu) -> String {
    let mut w = Vec::new();
    let (_, changeset) = difference::diff(&format!("{:?}", c1), &format!("{:?}", c2), "\n");
    for i in 0..changeset.len() {
      match changeset[i] {
        Difference::Same(_) => {
          // writeln!(w, "{}", x);
        }
        Difference::Add(ref x) => {
          writeln!(w, "Got:\n{}", x);
        }
        Difference::Rem(ref x) => {
          writeln!(w, "Expected:\n{}", x);
        }
      }
    }

    String::from_utf8(w).unwrap()
  }

  macro_rules! cpu_inline_test {
    (
      {
        ins: $ins:expr,
        before: $before:expr,
        after: $after:expr,
      }
    ) =>
    {
      let mut cpu = $before;
      cpu.execute_instruction($ins);
      assert!(cpu == $after, "\n{}", cpu_diff(&$after, &cpu));
    }
  }

  macro_rules! cpu_test {
    (
      $name:ident {
        ins: $ins:expr,
        before: $before:expr,
        after: $after:expr,
      }
    ) =>
    (
      #[test]
      #[allow(non_snake_case)]
      fn $name() {
        cpu_inline_test!({
          ins: $ins,
          before: $before,
          after: $after,
        });
      }
    )
  }

  #[test]
  #[allow(non_snake_case)]
  fn test_inst_BIT_b_r() {
    // Test with setting bit to 1
    for r in 0..7 {
      if r == 6 {
        // skip flag register
        continue;
      }

      for b in 0..7 {
        let r = Reg::from(r);
        cpu_inline_test!({
          ins: Instruction::BIT_b_r(b, r),
          before: {
            let mut c = Cpu::default();
            c.write_reg_byte(r, 1 << b);
            c
          },
          after: {
            let mut c = Cpu { clock_t: 8, ..Cpu::default() };
            c.write_reg_byte(r, 1 << b);
            c.write_flag(Flag::Z, false);
            c.write_flag(Flag::H, true);
            c
          },
        });
      }

      // Test with setting bit to 0
      for r in 0..7 {
        if r == 6 {
          // skip flag register
          continue;
        }

        for b in 0..7 {
          let r = Reg::from(r);
          cpu_inline_test!({
            ins: Instruction::BIT_b_r(b, r),
            before: Cpu::default(),
            after: {
              let mut c = Cpu { clock_t: 8, ..Cpu::default() };
              c.write_flag(Flag::Z, true);
              c.write_flag(Flag::H, true);
              c
            },
          });
        }
      }
    }
  }

  cpu_test!(test_inst_CALL_nn {
    ins: Instruction::CALL_nn(0x0095),
    before: {
      let mut c = Cpu::default();
      c.reg_sp = 100;
      c.reg_pc = 200;
      c
    },
    after: {
      let mut c = Cpu { clock_t: 24, ..Cpu::default() };
      c.reg_sp = 98;
      c.mem.write_byte(98, 200);
      c.reg_pc = 0x0095;
      c
    },
  });

  cpu_test!(test_inst_CP_n {
    ins: Instruction::CP_n(0x95),
    before: {
      let mut c = Cpu::default();
      c.write_reg_byte(Reg::A, 0x88);
      c
    },
    after: {
      let mut c = Cpu { clock_t: 4, ..Cpu::default() };
      c.write_flag(Flag::N, true);
      c.write_flag(Flag::C, true);
      c.write_reg_byte(Reg::A, 0x88);
      c
    },
  });

  #[test]
  #[allow(non_snake_case)]
  fn test_DEC_r() {
    for i in 0..7 {
      if i == 6 {
        continue;
      }

      let r = Reg::from(i);
      cpu_inline_test!({
        ins: Instruction::DEC_r(r),
        before: {
          let mut c = Cpu::default();
          c.write_reg_byte(r, 0x10);
          c
        },
        after: {
          let mut c = Cpu { clock_t: 4, ..Cpu::default()};
          c.write_flag(Flag::H, true);
          c.write_reg_byte(r, 0x0F);
          c
        },
      });
    }
  }

  #[test]
  #[allow(non_snake_case)]
  fn test_INC_r() {
    for i in 0..7 {
      if i == 6 {
        continue;
      }

      let r = Reg::from(i);
      cpu_inline_test!({
        ins: Instruction::INC_r(r),
        before: {
          let mut c = Cpu::default();
          c.write_reg_byte(r, 0x10);
          c
        },
        after: {
          let mut c = Cpu { clock_t: 4, ..Cpu::default()};
          c.write_reg_byte(r, 0x11);
          c
        },
      });
    }
  }

  #[test]
  #[allow(non_snake_case)]
  fn test_INC_rr() {
    for i in 0..3 {
      let r = Reg::from_pair(i);
      cpu_inline_test!({
        ins: Instruction::INC_rr(r),
        before: {
          let mut c = Cpu::default();
          c.write_reg_word(r, 0x10);
          c
        },
        after: {
          let mut c = Cpu { clock_t: 4, ..Cpu::default()};
          c.write_reg_word(r, 0x11);
          c
        },
      });
    }
  }

  #[test]
  #[allow(non_snake_case)]
  fn test_inst_JR_cc_e() {
    for flag in &[Flag::Z, Flag::C, Flag::NZ, Flag::NC] {
      let addrs = &[0x23, 0x00, 0xFF, 0xE6];
      let pcs = &[(0x1000 as i16) + (0x23 as u8 as i8 as i16),
                  (0x1000 as i16) + (0x00 as u8 as i8 as i16),
                  (0x1000 as i16) + (0xFF as u8 as i8 as i16),
                  (0x1000 as i16) + (0xE6 as u8 as i8 as i16)];

      for i in 0..addrs.len() {
        let mut c = Cpu::default();
        c.reg_pc = 0x1000;
        // c.mem.write_byte(0x1000, addrs[i]);
        c.write_flag(*flag, true);

        c.execute_instruction(Instruction::JR_cc_e(*flag, addrs[i]));

        assert_eq!(c.reg_pc, pcs[i] as u16);
      }

      for i in 0..addrs.len() {
        let mut c = Cpu::default();
        c.reg_pc = 0x1000;
        // c.mem.write_byte(0x1000, addrs[i]);
        c.write_flag(*flag, false);

        c.execute_instruction(Instruction::JR_cc_e(*flag, addrs[i]));

        assert_eq!(c.reg_pc, 0x1000);
      }
    }
  }

  cpu_test!(test_inst_LD_·0xFF00C·_A {
    ins: Instruction::LD_·0xFF00C·_A,
    before: {
      let mut c = Cpu::default();
      c.write_reg_byte(Reg::C, 0x10);
      c.write_reg_byte(Reg::A, 0xFF);
      c
    },
    after: {
      let mut c = Cpu { clock_t: 8, ..Cpu::default() };
      c.write_reg_byte(Reg::C, 0x10);
      c.write_reg_byte(Reg::A, 0xFF);
      c.mem.write_byte(0xFF10, 0xFF);
      c
    },
  });

  cpu_test!(test_inst_LD_·0xFF00n·_A {
    ins: Instruction::LD_·0xFF00n·_A(0x10),
    before: {
      let mut c = Cpu::default();
      c.write_reg_byte(Reg::A, 0xFF);
      c
    },
    after: {
      let mut c = Cpu { clock_t: 12, reg_pc: 0, ..Cpu::default() };
      c.write_reg_byte(Reg::A, 0xFF);
      c
    },
  });

  #[test]
  #[allow(non_snake_case)]
  fn test_inst_LD_·HL·_r() {
    for i in 0..7 {
      if i == 6 {
        continue;
      }

      let r = Reg::from(i);
      cpu_inline_test!({
        ins: Instruction::LD_·HL·_r(r),
        before: {
          let mut c = Cpu::default();
          c.write_reg_byte(r, 0x87);
          c.write_reg_byte(Reg::H, 0xC2);
          c.write_reg_byte(Reg::L, 0x21);
          c
        },
        after: {
          let mut c = Cpu { clock_t: 8, ..Cpu::default() };
          c.write_reg_byte(r, 0x87);
          c.write_reg_byte(Reg::H, 0xC2);
          c.write_reg_byte(Reg::L, 0x21);
          c.mem.write_byte(0xC221, 0x87);
          c
        },
      });
    }
  }

  cpu_test!(test_inst_LD_A_·DE· {
    ins: Instruction::LD_A_·DE·,
    before: {
      let mut c = Cpu::default();
      c.write_reg_word(Reg::DE, 0x0104);
      c.mem.write_byte(0x0104, 0x10);
      c
    },
    after: {
      let mut c = Cpu { clock_t: 8, ..Cpu::default() };
      c.write_reg_word(Reg::DE, 0x0104);
      c.write_reg_byte(Reg::A, 0x10);
      c.mem.write_byte(0x0104, 0x10);
      c
    },
  });

  #[test]
  #[allow(non_snake_case)]
  fn test_inst_LD_dd_nn() {
    cpu_inline_test!({
      ins: Instruction::LD_dd_nn(Reg::HL, 0xD8FE),
      before: Cpu::default(),
      after: Cpu {
        clock_t: 12,
        reg_hl: 0xD8FE,
        ..Cpu::default()
      },
    });

    cpu_inline_test!({
      ins: Instruction::LD_dd_nn(Reg::SP, 0xD8FE),
      before: Cpu::default(),
      after: Cpu {
        clock_t: 12,
        reg_sp: 0xD8FE,
        ..Cpu::default()
      },
    });
  }

  #[test]
  #[allow(non_snake_case)]
  fn test_inst_LD_r_n() {
    for i in 0..7 {
      if i == 6 {
        continue;
      }

      let r = Reg::from(i);

      cpu_inline_test!({
        ins: Instruction::LD_r_n(r, 0xFE),
        before: Cpu::default(),
        after: {
          let mut c = Cpu{
            clock_t: 8,
            ..Cpu::default()
          };
          c.write_reg_byte(r, 0xFE);
          c
        },
      });
    }
  }

  #[test]
  #[allow(non_snake_case)]
  fn test_inst_LD_r_r() {
    for j in 0..7 {
      if j == 6 {
        continue;
      }

      for i in 0..7 {
        if i == 6 {
          continue;
        }

        let r1 = Reg::from(i);
        let r2 = Reg::from(j);

        cpu_inline_test!({
        ins: Instruction::LD_r_r(r1, r2),
        before: {
          let mut c = Cpu::default();
          c.write_reg_byte(r2, 0xFE);
          c
        },
        after: {
          let mut c = Cpu{
            clock_t: 4,
            ..Cpu::default()
          };
          c.write_reg_byte(r1, 0xFE);
          c.write_reg_byte(r2, 0xFE);
          c
        },
      });
      }
    }
  }

  cpu_test!(test_inst_LDD_HL_A {
    ins: Instruction::LDD_·HL·_A,
    before: {
      let mut c = Cpu::default();
      c.write_reg_byte(Reg::A, 0x87);
      c.write_reg_byte(Reg::H, 0xC2);
      c.write_reg_byte(Reg::L, 0x21);
      c
    },
    after: {
      let mut c = Cpu { clock_t: 8, ..Cpu::default() };
      c.write_reg_byte(Reg::A, 0x87);
      c.write_reg_byte(Reg::H, 0xC2);
      c.write_reg_byte(Reg::L, 0x20);
      c.mem.write_byte(0xC221, 0x87);
      c
    },
  });

  cpu_test!(test_inst_LDI_HL_A {
    ins: Instruction::LDI_·HL·_A,
    before: {
      let mut c = Cpu::default();
      c.write_reg_byte(Reg::A, 0x87);
      c.write_reg_byte(Reg::H, 0xC2);
      c.write_reg_byte(Reg::L, 0x21);
      c
    },
    after: {
      let mut c = Cpu { clock_t: 8, ..Cpu::default() };
      c.write_reg_byte(Reg::A, 0x87);
      c.write_reg_byte(Reg::H, 0xC2);
      c.write_reg_byte(Reg::L, 0x22);
      c.mem.write_byte(0xC221, 0x87);
      c
    },
  });

  cpu_test!(test_inst_nop {
    ins: Instruction::NOP,
    before: Cpu::default(),
    after: Cpu { clock_t: 4, ..Cpu::default() },
  });

  cpu_test!(inst_xor_a {
    ins: Instruction::XOR_r(Reg::A),
    before: {
      let mut c = Cpu::default();
      c.write_reg_byte(Reg::A, 200);
      c
    },
    after: {
      let mut c = Cpu { clock_t: 4, ..Cpu::default() };
      c.write_flag(Flag::Z, true);
      c
    },
  });

  #[test]
  #[allow(non_snake_case)]
  fn test_inst_XOR_r() {
    for r in 0..7 {
      if r == 6 {
        // skip flag register
        continue;
      }
      let r = Reg::from(r);
      cpu_inline_test!({
        ins: Instruction::XOR_r(r),
        before: {
          let mut c = Cpu::default();
          c.write_reg_byte(r, 200);
          c.write_reg_byte(Reg::A, 200);
          c
        },
        after: {
          let mut c = Cpu { clock_t: 4, ..Cpu::default() };
          c.write_reg_byte(r, 200);
          c.write_flag(Flag::Z, true);
          c
        },
      });
    }
  }
}
