pub use self::module::Mem;

#[allow(dead_code)]
mod constants {
  // 16KB ROM Bank 00
  // In cartridge, fixed at bank 00
  pub const ROM_00_START: u16 = 0x0000;
  pub const ROM_00_END: u16 = 0x3FFF;

  // 16KB ROM Bank 01..NN
  // In cartridge, switable bank number
  pub const ROM_01_START: u16 = 0x4000;
  pub const ROM_01_END: u16 = 0x7FFF;

  // Swichable bank 0-1 in CGB Mode
  pub const VIDEO_RAM_START: u16 = 0x8000;
  pub const VIDEO_RAM_END: u16 = 0x9FFF;

  // In cartridge, switchable bank, if any
  pub const EXTERNAL_RAM_START: u16 = 0xA000;
  pub const EXTERNAL_RAM_END: u16 = 0xBFFF;

  // 4KB work RAM bank 0 (WRAM)
  pub const WORK_RAM_0_START: u16 = 0xC000;
  pub const WORK_RAM_0_END: u16 = 0xCFFF;
  pub const WORK_RAM_0_LEN: usize = WORK_RAM_0_END as usize - WORK_RAM_0_START as usize;

  // 4KB Work RAM Bank 1 (WRAM)
  // switchable bank 1-7 in CGB Mode
  pub const WORK_RAM_1_START: u16 = 0xD000;
  pub const WORK_RAM_1_END: u16 = 0xDFFF;
  pub const WORK_RAM_1_LEN: usize = WORK_RAM_1_END as usize - WORK_RAM_1_START as usize;

  // Same as C000-DDFF (ECHO)
  // typically not used
  pub const ECHO_START: u16 = 0xE000;
  pub const ECHO_END: u16 = 0xFDFF;

  // Sprite attribute table (OAM)
  pub const SPRITE_TABLE_START: u16 = 0xFE00;
  pub const SPRITE_TABLE_END: u16 = 0xFE9F;

  // Not usable
  pub const UNUSABLE_START: u16 = 0xFEA0;
  pub const UNUSABLE_END: u16 = 0xFEFF;

  // I/O Ports (gamepad buttons)
  pub const IO_PORTS_START: u16 = 0xFF00;
  pub const IO_PORTS_END: u16 = 0xFF7F;

  // High RAM (HRAM)
  pub const HIGH_RAM_START: u16 = 0xFF80;
  pub const HIGH_RAM_END: u16 = 0xFFFE;

  // Interrupt Enable Register
  pub const INTERRUPT_REGISTER_START: u16 = 0xFFFF;
  pub const INTERRUPT_REGISTER_END: u16 = 0xFFFF;
}

#[derive(Debug)]
pub enum Addr {
  Rom00(u16),
  Rom01(u16),
  VideoRam(u16),
  ExternalRam(u16),
  WorkRam0(u16),
  WorkRam1(u16),
  SpriteTable(u16),
  IoPorts(u16),
  HighRam(u16),
  InterruptRegister,
}

pub trait Memory {
  fn read_byte(&self, addr: u16) -> u8;
  fn write_byte(&mut self, addr: u16, value: u8);
  fn set_boot_rom(&mut self, rom: Box<[u8]>);
  fn set_cart_rom(&mut self, rom: Box<[u8]>);

  fn read_word(&self, addr: u16) -> u16 {
    let mut val: u16 = (self.read_byte(addr + 1) as u16) << 8;
    val |= self.read_byte(addr) as u16;
    val
  }

  fn write_word(&mut self, addr: u16, value: u16) {
    self.write_byte(addr + 1, (value >> 8) as u8 & 0b11111111);
    self.write_byte(addr, value as u8 & 0b11111111);
  }
}

#[cfg(not(test))]
mod module {
  use super::*;
  use std::fmt;
  use md5;
  use super::constants::*;

  pub struct Mem {
    boot_rom: Box<[u8]>,
    cart_rom: Box<[u8]>,
    booting: bool,

    work_ram_0: [u8; WORK_RAM_0_LEN],
    work_ram_1: [u8; WORK_RAM_1_LEN],
  }

  impl Mem {
    pub fn new() -> Mem {
      Mem {
        boot_rom: Box::new([]),
        cart_rom: Box::new([]),
        booting: false,
        work_ram_0: [0; WORK_RAM_0_LEN],
        work_ram_1: [0; WORK_RAM_1_LEN],
      }
    }

    pub fn memory_map(&self, addr: u16) -> Addr {
      match addr {
        ROM_00_START...ROM_00_END => Addr::Rom00(addr - ROM_00_START),
        ROM_01_START...ROM_01_END => Addr::Rom01(addr - ROM_01_START),
        VIDEO_RAM_START...VIDEO_RAM_END => Addr::VideoRam(addr - VIDEO_RAM_START),
        EXTERNAL_RAM_START...EXTERNAL_RAM_END => Addr::ExternalRam(addr - EXTERNAL_RAM_START),
        WORK_RAM_0_START...WORK_RAM_0_END => Addr::WorkRam0(addr - WORK_RAM_0_START),
        WORK_RAM_1_START...WORK_RAM_1_END => Addr::WorkRam1(addr - WORK_RAM_1_START),
        ECHO_START...ECHO_END => self.memory_map(addr - ECHO_START + WORK_RAM_0_START),
        SPRITE_TABLE_START...SPRITE_TABLE_END => Addr::SpriteTable(addr - SPRITE_TABLE_START),
        UNUSABLE_START...UNUSABLE_END => panic!("unusable memory area!"),
        IO_PORTS_START...IO_PORTS_END => Addr::IoPorts(addr - IO_PORTS_START),
        HIGH_RAM_START...HIGH_RAM_END => Addr::HighRam(addr - HIGH_RAM_START),
        INTERRUPT_REGISTER_START...INTERRUPT_REGISTER_END => Addr::InterruptRegister,
        _ => {
          panic!("unrecognized memory mapped region");
        }
      }
    }
  }

  impl fmt::Debug for Mem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

  impl Memory for Mem {
    fn read_byte(&self, addr: u16) -> u8 {
      let mapped = self.memory_map(addr);
      match mapped {
        Addr::Rom00(offset) => {
          if self.booting {
            self.boot_rom[offset as usize]
          } else {
            self.cart_rom[offset as usize]
          }
        }
        Addr::Rom01(offset) => panic!("read_byte not implemented: {:?}", mapped),
        Addr::VideoRam(offset) => panic!("read_byte not implemented: {:?}", mapped),
        Addr::ExternalRam(offset) => panic!("read_byte not implemented: {:?}", mapped),
        Addr::WorkRam0(offset) => self.work_ram_0[offset as usize],
        Addr::WorkRam1(offset) => self.work_ram_1[offset as usize],
        Addr::SpriteTable(offset) => panic!("read_byte not implemented: {:?}", mapped),
        Addr::IoPorts(offset) => panic!("read_byte not implemented: {:?}", mapped),
        Addr::HighRam(offset) => panic!("read_byte not implemented: {:?}", mapped),
        Addr::InterruptRegister => panic!("read_byte not implemented: {:?}", mapped),
      }
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
      let mapped = self.memory_map(addr);
      match mapped {
        Addr::Rom00(offset) => {
          panic!("write_byte error: trying to write to rom0");
        }
        Addr::Rom01(offset) => panic!("write_byte not implemented: {:?}", mapped),
        Addr::VideoRam(offset) => panic!("write_byte not implemented: {:?}", mapped),
        Addr::ExternalRam(offset) => panic!("write_byte not implemented: {:?}", mapped),
        Addr::WorkRam0(offset) => {
          self.work_ram_0[offset as usize] = value;
        }
        Addr::WorkRam1(offset) => {
          self.work_ram_1[offset as usize] = value;
        }
        Addr::SpriteTable(offset) => panic!("write_byte not implemented: {:?}", mapped),
        Addr::IoPorts(offset) => panic!("write_byte not implemented: {:?}", mapped),
        Addr::HighRam(offset) => panic!("write_byte not implemented: {:?}", mapped),
        Addr::InterruptRegister => panic!("write_byte not implemented: {:?}", mapped),
      };
    }

    fn set_boot_rom(&mut self, rom: Box<[u8]>) {
      self.booting = true;
      self.boot_rom = rom;
    }

    fn set_cart_rom(&mut self, rom: Box<[u8]>) {
      self.cart_rom = rom;
    }
  }
}

#[cfg(test)]
mod module {
  use super::*;
  use std::fmt;
  use md5;

  pub struct Mem {
    boot_rom: Box<[u8]>,
    cart_rom: Box<[u8]>,
    booting: bool,

    ram: [u8; 0xFFFF + 1],
  }

  impl PartialEq for Mem {
    fn eq(&self, x: &Mem) -> bool {
      self.booting == x.booting && self.ram[..] == x.ram[..]
    }
  }

  impl fmt::Debug for Mem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      try!(write!(f, "\nBooting: {}", self.booting));
      try!(write!(f, "\nRam checksum: {:?}", md5::compute(&self.ram[..])));
      write!(f, "\n")
    }
  }

  impl Mem {
    pub fn new() -> Mem {
      Mem {
        boot_rom: Box::new([]),
        cart_rom: Box::new([]),
        booting: false,
        ram: [0; 0xFFFF + 1],
      }
    }
  }

  impl Memory for Mem {
    fn read_byte(&self, addr: u16) -> u8 {
      self.ram[addr as usize]
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
      self.ram[addr as usize] = value;
    }

    fn set_boot_rom(&mut self, rom: Box<[u8]>) {
      self.booting = true;
      self.boot_rom = rom;
    }

    fn set_cart_rom(&mut self, rom: Box<[u8]>) {
      self.cart_rom = rom;
    }
  }
}
