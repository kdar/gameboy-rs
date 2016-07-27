use std::fmt;
use std::default::Default;
use std::cmp::PartialEq;
use md5;

use super::mem_map;

pub enum Flag {
  Z = 0b10000000, // zero flag
  N = 0b01000000, // add/sub flag
  H = 0b00100000, // half carry flag
  C = 0b00010000, // carry flag
}

#[derive(Debug)]
pub enum Reg {
  B = 0b000,
  C = 0b001,
  D = 0b010,
  E = 0b011,
  H = 0b100,
  L = 0b101,
  F = 0b110,
  A = 0b111, /* BC,
              * DE,
              * HL,
              * SP, */
}

// impl Reg {
//   pub fn from_byte(r: u8) -> Reg {
//     match r {
//       0b000 => Reg::B,
//       0b001 => Reg::C,
//       0b010 => Reg::D,
//       0b011 => Reg::E,
//       0b100 => Reg::H,
//       0b101 => Reg::L,
//       0b110 => Reg::F,
//       0b111 => Reg::A,
//       _ => panic!("reg.from_raw_byte unknown register: {}", r),
//     }
//   }
//
//   pub fn from_word(r: u16) -> Reg {
//     match r {
//       0b00 => Reg::BC,
//       0b01 => Reg::DE,
//       0b10 => Reg::HL,
//       0b11 => Reg::SP,
//       _ => panic!("reg.from_raw_byte unknown register: {}", r),
//     }
//   }
// }

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

  boot_rom: Box<[u8]>,
  cart_rom: Box<[u8]>,
  booting: bool,

  work_ram_0: [u8; mem_map::WORK_RAM_0_LEN],
  work_ram_1: [u8; mem_map::WORK_RAM_1_LEN],
}

impl PartialEq for Cpu {
  fn eq(&self, x: &Cpu) -> bool {
    self.reg_af == x.reg_af && self.reg_bc == x.reg_bc && self.reg_de == x.reg_de &&
    self.reg_hl == x.reg_hl && self.reg_sp == x.reg_sp && self.reg_pc == x.reg_pc &&
    self.cycles == self.cycles && self.booting == x.booting &&
    self.work_ram_0[..] == x.work_ram_0[..] && self.work_ram_1[..] == x.work_ram_1[..]
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
      boot_rom: Box::new([]),
      cart_rom: Box::new([]),
      booting: false,
      work_ram_0: [0; mem_map::WORK_RAM_0_LEN],
      work_ram_1: [0; mem_map::WORK_RAM_1_LEN],
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
    try!(write!(f, "\nBooting: {}", self.booting));
    try!(write!(f,
                "\nWork ram 0 checksum: {:?}",
                md5::compute(&self.work_ram_0[..])));
    try!(write!(f,
                "\nWork ram 1 checksum: {:?}",
                md5::compute(&self.work_ram_1[..])));
    write!(f, "\n")
  }
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu::default()
  }

  pub fn set_boot_rom(&mut self, rom: Box<[u8]>) {
    self.booting = true;
    self.boot_rom = rom;
  }

  pub fn set_cart_rom(&mut self, rom: Box<[u8]>) {
    self.cart_rom = rom;
  }

  pub fn read_word(&self, addr: u16) -> u16 {
    let mut val: u16 = (self.read_mapped_byte(addr + 1) as u16) << 8;
    val |= self.read_mapped_byte(addr) as u16;
    val
  }

  pub fn read_mapped_byte(&self, addr: u16) -> u8 {
    let mapped = mem_map::memory_map(addr);
    match mapped {
      mem_map::Addr::Rom00(offset) => {
        if self.booting {
          self.boot_rom[offset as usize]
        } else {
          self.cart_rom[offset as usize]
        }
      }
      mem_map::Addr::Rom01(offset) => panic!("read_mapped_byte not implemented: {:?}", mapped),
      mem_map::Addr::VideoRam(offset) => panic!("read_mapped_byte not implemented: {:?}", mapped),
      mem_map::Addr::ExternalRam(offset) => {
        panic!("read_mapped_byte not implemented: {:?}", mapped)
      }
      mem_map::Addr::WorkRam0(offset) => self.work_ram_0[offset as usize],
      mem_map::Addr::WorkRam1(offset) => self.work_ram_1[offset as usize],
      mem_map::Addr::SpriteTable(offset) => {
        panic!("read_mapped_byte not implemented: {:?}", mapped)
      }
      mem_map::Addr::IoPorts(offset) => panic!("read_mapped_byte not implemented: {:?}", mapped),
      mem_map::Addr::HighRam(offset) => panic!("read_mapped_byte not implemented: {:?}", mapped),
      mem_map::Addr::InterruptRegister => panic!("read_mapped_byte not implemented: {:?}", mapped),
    }
  }

  pub fn write_mapped_word(&mut self, addr: u16, value: u16) {
    self.write_mapped_byte(addr + 1, (value >> 8) as u8 & 0b11111111);
    self.write_mapped_byte(addr, value as u8 & 0b11111111);
  }

  pub fn write_mapped_byte(&mut self, addr: u16, value: u8) {
    let mapped = mem_map::memory_map(addr);
    match mapped {
      mem_map::Addr::Rom00(offset) => {
        panic!("write_mapped_byte error: trying to write to rom0");
      }
      mem_map::Addr::Rom01(offset) => panic!("write_mapped_byte not implemented: {:?}", mapped),
      mem_map::Addr::VideoRam(offset) => panic!("write_mapped_byte not implemented: {:?}", mapped),
      mem_map::Addr::ExternalRam(offset) => {
        panic!("write_mapped_byte not implemented: {:?}", mapped)
      }
      mem_map::Addr::WorkRam0(offset) => {
        self.work_ram_0[offset as usize] = value;
      }
      mem_map::Addr::WorkRam1(offset) => {
        self.work_ram_1[offset as usize] = value;
      }
      mem_map::Addr::SpriteTable(offset) => {
        panic!("write_mapped_byte not implemented: {:?}", mapped)
      }
      mem_map::Addr::IoPorts(offset) => panic!("write_mapped_byte not implemented: {:?}", mapped),
      mem_map::Addr::HighRam(offset) => panic!("write_mapped_byte not implemented: {:?}", mapped),
      mem_map::Addr::InterruptRegister => panic!("write_mapped_byte not implemented: {:?}", mapped),
    };
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

    // println!("{:?}", self);
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
        0x32 => self.inst_ldd_hl_a(),
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
    self.write_reg_byte(Reg::H, h);
    self.write_reg_byte(Reg::L, l);
    12
  }

  // 0x31
  // LD SP,nn
  // Page 120
  fn inst_ld_sp_nn(&mut self) -> u32 {
    self.reg_sp = self.read_pc_word();
    12
  }

  // 0x32
  // LDD (HL),A
  // Page 149
  fn inst_ldd_hl_a(&mut self) -> u32 {
    let hl = self.reg_hl;
    let a = self.read_reg_byte(Reg::A);
    self.write_mapped_byte(hl, a);
    self.reg_hl -= 1;
    8
  }

  fn xor(&mut self, register: Reg) -> u32 {
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

  // 0xAF
  // XOR A
  fn inst_xor_a(&mut self) -> u32 {
    self.xor(Reg::A)
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
      // _ => panic!("read_mapped_byte_register unknown register: {:?}", register),
    }
  }

  // fn write_reg_word(&mut self, register: Reg, value: u16) {
  //   match register {
  //     Reg::BC => self.reg_bc = value,
  //     Reg::DE => self.reg_de = value,
  //     Reg::HL => self.reg_hl = value,
  //     Reg::SP => self.reg_sp = value,
  //     _ => panic!("write_reg_word unknown register: {:?}", register),
  //   }
  // }

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
      // _ => panic!("write_reg_byte unknown register: {:?}", register),
    }
  }

  fn read_pc_byte(&mut self) -> u8 {
    let d = self.read_mapped_byte(self.reg_pc);
    self.reg_pc += 1;
    d
  }

  fn read_pc_word(&mut self) -> u16 {
    let d = self.read_word(self.reg_pc);
    self.reg_pc += 2;
    d
  }

  fn write_flag(&mut self, flag: Flag, value: bool) {
    let mut d = self.read_reg_byte(Reg::F);
    if value {
      d |= flag as u8;
    } else {
      d &= !(flag as u8);
    }
    self.write_reg_byte(Reg::F, d);
  }

  fn read_flag(&self, flag: Flag) -> bool {
    let mut d = self.read_reg_byte(Reg::F);
    d & flag as u8 > 0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

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
    debug_assert_eq!(c.reg_af, 0b00000000_10000000);
    c.write_flag(Flag::N, true);
    debug_assert_eq!(c.reg_af, 0b00000000_11000000);
    c.write_flag(Flag::H, true);
    debug_assert_eq!(c.reg_af, 0b00000000_11100000);
    c.write_flag(Flag::C, true);
    debug_assert_eq!(c.reg_af, 0b00000000_11110000);
  }

  // We don't compare the boot_rom or cart_rom for equality.
  macro_rules! cpu_test {
    (
      $name:ident {
        ins: $ins:expr,
        before: $before:expr,
        after:  $after:expr,
      }
    ) =>
    (
      #[test]
      fn $name() {
        let mut cpu = $before;
        cpu.execute($ins);
        assert_eq!(cpu, $after);
        // assert_eq!(cpu.reg_af, $after.reg_af);
        // assert_eq!(cpu.reg_bc, $after.reg_bc);
        // assert_eq!(cpu.reg_de, $after.reg_de);
        // assert_eq!(cpu.reg_hl, $after.reg_hl);
        // assert_eq!(cpu.reg_sp, $after.reg_sp);
        // assert_eq!(cpu.reg_pc, $after.reg_pc);
        // assert_eq!(cpu.cycles, $after.cycles);
        // assert_eq!(cpu.booting, $after.booting);
      }
    )
  }

  cpu_test!(inst_nop {
    ins: 0x00,
    before: Cpu::default(),
    after: Cpu { cycles: 4, ..Cpu::default() },
  });

  cpu_test!(inst_ld_hl_nn {
    ins: 0x21,
    before: Cpu { cart_rom: Box::new([0xFE, 0xD8]), ..Cpu::default() },
    after: {
      let mut c = Cpu {
        cycles: 12,
        reg_pc: 2,
        ..Cpu::default()
      };
      c.write_reg_byte(Reg::H, 0xFE);
      c.write_reg_byte(Reg::L, 0xD8);
      c
    },
  });

  cpu_test!(inst_ld_sp_nn {
    ins: 0x31,
    before: Cpu { cart_rom: Box::new([0xFE, 0xD8]), ..Cpu::default() },
    after: Cpu {
      cycles: 12,
      reg_pc: 2,
      reg_sp: 0xD8FE,
      ..Cpu::default()
    },
  });

  cpu_test!(inst_ldd_hl_a {
    ins: 0x32,
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
      c.write_mapped_byte(0xC221, 0x87);
      c
    },
  });

  cpu_test!(inst_xor_a {
    ins: 0xAF,
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
}
