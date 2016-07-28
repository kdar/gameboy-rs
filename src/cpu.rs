use std::fmt;
use std::default::Default;
use std::cmp::PartialEq;

use super::mem;
use super::reg::Reg;
use super::flag::Flag;
use super::instruction::Instruction;

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

  cycles: u32, // Current number of clock cycles

  mem: Box<mem::Memory>,
}

impl PartialEq for Cpu {
  fn eq(&self, x: &Cpu) -> bool {
    self.reg_af == x.reg_af && self.reg_bc == x.reg_bc && self.reg_de == x.reg_de &&
    self.reg_hl == x.reg_hl && self.reg_sp == x.reg_sp && self.reg_pc == x.reg_pc &&
    self.cycles == x.cycles
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
      cycles: 0,
      mem: Box::new(mem::Mem::new()),
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
    try!(write!(f, "\nCycles:  {}", self.cycles));
    write!(f, "\n")
  }
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu::default()
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
    d
  }

  fn read_pc_word(&mut self) -> u16 {
    let d = self.mem.read_word(self.reg_pc);
    self.reg_pc += 2;
    d
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

  pub fn step(&mut self) {
    let opcode = self.read_pc_byte();
    self.execute_opcode(opcode);

    // println!("{:?}", self);
  }

  fn execute_opcode(&mut self, opcode: u8) {
    if opcode == 0xCB {
      let opcode = self.read_pc_byte();
      self.execute_instruction(Instruction::from_cb(opcode));
    } else {
      self.execute_instruction(Instruction::from(opcode));
    }
  }

  fn execute_instruction(&mut self, ins: Instruction) {
    let cycles = match ins {
      Instruction::NOP => self.inst_nop(),
      Instruction::LD_dd_nn(dd) => self.inst_ld_dd_nn(dd),
      Instruction::LD_r_n(r) => self.inst_ld_r_n(r),
      Instruction::LD_0xff00c_a => self.inst_ld_0xff00c_a(),
      Instruction::LDD_hl_a => self.inst_ldd_hl_a(),
      Instruction::XOR_r(r) => self.inst_xor_r(r),
      Instruction::JR_cc_e(cc) => self.inst_jr_cc_e(cc),

      Instruction::BIT_b_r(b, r) => self.inst_bit_b_r(b, r),
      // _ => panic!("instruction not implemented: {:?}", ins),
    };

    self.cycles += cycles;
  }

  // NOP
  // 0x00
  fn inst_nop(&self) -> u32 {
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
  fn inst_jr_cc_e(&mut self, flag: Flag) -> u32 {
    // signed argument
    let e = self.read_pc_byte() as i8;
    if self.read_flag(flag) {
      // signed addition (can jump back)
      self.reg_pc = ((self.reg_pc as i16) + (e as i16)) as u16;
      12
    } else {
      8
    }
  }

  // LD dd,nn
  // Opcode: 00dd0001
  // Page: 120
  fn inst_ld_dd_nn(&mut self, reg: Reg) -> u32 {
    let nn = self.read_pc_word();
    self.write_reg_word(reg, nn);
    12
  }

  // LD r,n
  // Opcode: 00rrr110
  // Page: 100
  fn inst_ld_r_n(&mut self, reg: Reg) -> u32 {
    let n = self.read_pc_byte();
    self.write_reg_byte(reg, n);
    8
  }

  // LD (0xFF00+C),A
  // Opcode: 0xE2
  // Moved instruction.
  fn inst_ld_0xff00c_a(&mut self) -> u32 {
    let a = self.read_reg_byte(Reg::A);
    let c = self.read_reg_byte(Reg::C);
    self.mem.write_byte(0xFF00 + c as u16, a);
    8
  }

  // LDD (HL),A
  // Opcode: 0x32
  // Page: 149
  fn inst_ldd_hl_a(&mut self) -> u32 {
    let hl = self.reg_hl;
    let a = self.read_reg_byte(Reg::A);
    self.mem.write_byte(hl, a);
    self.reg_hl -= 1;
    8
  }

  // XOR r
  // Opcode: 10110rrr
  // Page: 174
  // This instruction is a subset of the defined instruction in the pdf.
  // The superset instruction is XOR s, where s can be r, n, (HL), (IX+d)
  // or (IY+d).
  fn inst_xor_r(&mut self, register: Reg) -> u32 {
    let register = self.read_reg_byte(register);
    let mut accumulator = self.read_reg_byte(Reg::A);
    accumulator = accumulator ^ register;
    self.write_reg_byte(Reg::A, accumulator);

    if accumulator == 0 {
      self.write_flag(Flag::Z, true);
    } else {
      self.write_flag(Flag::Z, false);
    }

    self.write_flag(Flag::N, false);
    self.write_flag(Flag::C, false);

    4
  }

  // BIT b,r
  // Opcode: 0xCB 01bbbrrr
  // Page: 242
  fn inst_bit_b_r(&mut self, b: u8, r: Reg) -> u32 {
    let d = self.read_reg_byte(r);

    if d & (1 << b) > 0 {
      self.write_flag(Flag::Z, false);
    } else {
      self.write_flag(Flag::Z, true);
    }

    self.write_flag(Flag::H, true);
    self.write_flag(Flag::N, false);

    8
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use super::super::reg::Reg;
  use super::super::flag::Flag;
  use super::super::instruction::Instruction;
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
        Difference::Same(ref x) => {
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
      fn $name() {
        cpu_inline_test!({
          ins: $ins,
          before: $before,
          after: $after,
        });
      }
    )
  }

  cpu_test!(test_inst_nop {
    ins: Instruction::NOP,
    before: Cpu::default(),
    after: Cpu { cycles: 4, ..Cpu::default() },
  });

  #[test]
  fn test_inst_jr_cc_e() {
    for flag in &[Flag::Z, Flag::C, Flag::NZ, Flag::NC] {
      let addrs = &[0x23, 0x00, 0xFF, 0xE6];
      let pcs = &[(0x1000 as i16) + (0x23 as i8 as i16) + 1,
                  (0x1000 as i16) + (0x00 as i8 as i16) + 1,
                  (0x1000 as i16) + (0xFF as i8 as i16) + 1,
                  (0x1000 as i16) + (0xE6 as i8 as i16) + 1];

      for i in 0..addrs.len() {
        let mut c = Cpu::default();
        c.reg_pc = 0x1000;
        c.mem.write_byte(0x1000, addrs[i]);
        c.write_flag(*flag, true);

        c.execute_instruction(Instruction::JR_cc_e(*flag));

        assert_eq!(c.reg_pc, pcs[i] as u16);
      }

      for i in 0..addrs.len() {
        let mut c = Cpu::default();
        c.reg_pc = 0x1000;
        c.mem.write_byte(0x1000, addrs[i]);
        c.write_flag(*flag, false);

        c.execute_instruction(Instruction::JR_cc_e(*flag));

        assert_eq!(c.reg_pc, 0x1001);
      }
    }
  }

  #[test]
  fn test_inst_ld_dd_nn() {
    cpu_inline_test!({
      ins: Instruction::LD_dd_nn(Reg::HL),
      before: {
        let mut c = Cpu::default();
        c.mem.write_word(0, 0xD8FE);
        c
      },
      after: Cpu {
        cycles: 12,
        reg_pc: 2,
        reg_hl: 0xD8FE,
        ..Cpu::default()
      },
    });

    cpu_inline_test!({
      ins: Instruction::LD_dd_nn(Reg::SP),
      before: {
        let mut c = Cpu::default();
        c.mem.write_word(0, 0xD8FE);
        c
      },
      after: Cpu {
        cycles: 12,
        reg_pc: 2,
        reg_sp: 0xD8FE,
        ..Cpu::default()
      },
    });
  }

  #[test]
  fn test_inst_ld_r_n() {
    for i in 0..7 {
      if i == 6 {
        continue;
      }

      let r = Reg::from(i);

      cpu_inline_test!({
        ins: Instruction::LD_r_n(r),
        before: {
          let mut c = Cpu::default();
          c.mem.write_byte(0, 0xFE);
          c
        },
        after: {
          let mut c = Cpu{
            cycles: 8,
            reg_pc: 1,
            ..Cpu::default()
          };
          c.write_reg_byte(r, 0xFE);
          c
        },
      });
    }
  }

  cpu_test!(test_inst_ld_0xff00c_a {
    ins: Instruction::LD_0xff00c_a,
    before: {
      let mut c = Cpu::default();
      c.write_reg_byte(Reg::C, 0x10);
      c.write_reg_byte(Reg::A, 0xFF);
      c
    },
    after: {
      let mut c = Cpu { cycles: 8, ..Cpu::default() };
      c.write_reg_byte(Reg::C, 0x10);
      c.write_reg_byte(Reg::A, 0xFF);
      c.mem.write_byte(0xFF10, 0xFF);
      c
    },
  });

  cpu_test!(test_inst_ldd_hl_a {
    ins: Instruction::LDD_hl_a,
    before: {
      let mut c = Cpu::default();
      c.write_reg_byte(Reg::A, 0x87);
      c.write_reg_byte(Reg::H, 0xC2);
      c.write_reg_byte(Reg::L, 0x21);
      c
    },
    after: {
      let mut c = Cpu { cycles: 8, ..Cpu::default() };
      c.write_reg_byte(Reg::A, 0x87);
      c.write_reg_byte(Reg::H, 0xC2);
      c.write_reg_byte(Reg::L, 0x20);
      c.mem.write_byte(0xC221, 0x87);
      c
    },
  });

  cpu_test!(inst_xor_a {
    ins: Instruction::XOR_r(Reg::A),
    before: {
      let mut c = Cpu::default();
      c.write_reg_byte(Reg::A, 200);
      c
    },
    after: {
      let mut c = Cpu { cycles: 4, ..Cpu::default() };
      c.write_flag(Flag::Z, true);
      c
    },
  });

  #[test]
  fn test_inst_xor_r() {
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
          let mut c = Cpu { cycles: 4, ..Cpu::default() };
          c.write_reg_byte(r, 200);
          c.write_flag(Flag::Z, true);
          c
        },
      });
    }
  }

  #[test]
  fn test_inst_bit_b_r() {
    let mut c = Cpu::default();

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
            let mut c = Cpu { cycles: 8, ..Cpu::default() };
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
              let mut c = Cpu { cycles: 8, ..Cpu::default() };
              c.write_flag(Flag::Z, true);
              c.write_flag(Flag::H, true);
              c
            },
          });
        }
      }
    }
  }
}
