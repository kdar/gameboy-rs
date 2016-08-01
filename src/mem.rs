use std::rc::Rc;
use std::cell::RefCell;

pub use self::module::Mem;

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

// I/O Ports (gamepad buttons, sound, etc..)
pub const IO_PORTS_START: u16 = 0xFF00;
pub const IO_PORTS_END: u16 = 0xFF7F;

// High RAM (HRAM)
pub const HIGH_RAM_START: u16 = 0xFF80;
pub const HIGH_RAM_END: u16 = 0xFFFE;
pub const HIGH_RAM_LEN: usize = HIGH_RAM_END as usize - HIGH_RAM_START as usize;

// Interrupt Enable Register
pub const INTERRUPT_REGISTER_START: u16 = 0xFFFF;
pub const INTERRUPT_REGISTER_END: u16 = 0xFFFF;

#[derive(Debug)]
pub enum Addr {
  Rom00(u16, u16),
  Rom01(u16, u16),
  VideoRam(u16, u16),
  ExternalRam(u16, u16),
  WorkRam0(u16, u16),
  WorkRam1(u16, u16),
  SpriteTable(u16, u16),
  IoPorts(u16, u16),
  HighRam(u16, u16),
  InterruptRegister,
}

pub trait MemoryMap {
  fn read_byte(&self, addr: u16) -> Option<u8>;
  fn write_byte(&mut self, addr: u16, value: u8);

  fn read_vec(&self, addr: u16, len: u16) -> Option<Vec<u8>> {
    let mut v = vec![];
    for i in addr..addr + len {
      match self.read_byte(i) {
        Some(x) => v.push(x),
        None => break,
      }
    }

    if v.len() == 0 {
      None
    } else {
      Some(v)
    }
  }

  fn read_word(&self, addr: u16) -> Option<u16> {
    let mut val: u16 = match self.read_byte(addr + 1) {
      Some(x) => (x as u16) << 8,
      None => return None,
    };
    val |= match self.read_byte(addr) {
      Some(x) => x as u16,
      None => return None,
    };
    Some(val)
  }

  fn write_word(&mut self, addr: u16, value: u16) {
    self.write_byte(addr + 1, (value >> 8) as u8 & 0b11111111);
    self.write_byte(addr, value as u8 & 0b11111111);
  }
}

pub trait Memory: MemoryMap {
  fn map(&mut self, start: u16, end: u16, mapper: Rc<RefCell<MemoryMap>>);
  fn set_booting(&mut self, value: bool);
  fn set_boot_rom(&mut self, rom: Box<[u8]>);
  fn set_cart_rom(&mut self, rom: Box<[u8]>);
}

#[cfg(not(test))]
mod module {
  use std::rc::Rc;
  use std::cell::RefCell;

  use super::*;
  use std::fmt;
  use md5;

  pub struct Mem {
    boot_rom: Box<[u8]>,
    cart_rom: Box<[u8]>,
    booting: bool,

    work_ram_0: [u8; WORK_RAM_0_LEN],
    work_ram_1: [u8; WORK_RAM_1_LEN],

    high_ram: [u8; HIGH_RAM_LEN],

    map: Vec<(u16, u16, Rc<RefCell<MemoryMap>>)>,
  }

  impl Mem {
    pub fn new() -> Mem {
      Mem {
        boot_rom: Box::new([]),
        cart_rom: Box::new([]),
        booting: false,
        work_ram_0: [0; WORK_RAM_0_LEN],
        work_ram_1: [0; WORK_RAM_1_LEN],
        high_ram: [0; HIGH_RAM_LEN],
        map: Vec::new(),
      }
    }

    pub fn memory_map(&self, addr: u16) -> Addr {
      match addr {
        ROM_00_START...ROM_00_END => Addr::Rom00(addr, addr - ROM_00_START),
        ROM_01_START...ROM_01_END => Addr::Rom01(addr, addr - ROM_01_START),
        // VIDEO_RAM_START...VIDEO_RAM_END => Addr::VideoRam(addr, addr - VIDEO_RAM_START),
        EXTERNAL_RAM_START...EXTERNAL_RAM_END => Addr::ExternalRam(addr, addr - EXTERNAL_RAM_START),
        WORK_RAM_0_START...WORK_RAM_0_END => Addr::WorkRam0(addr, addr - WORK_RAM_0_START),
        WORK_RAM_1_START...WORK_RAM_1_END => Addr::WorkRam1(addr, addr - WORK_RAM_1_START),
        ECHO_START...ECHO_END => self.memory_map(addr - ECHO_START + WORK_RAM_0_START),
        // SPRITE_TABLE_START...SPRITE_TABLE_END => Addr::SpriteTable(addr, addr - SPRITE_TABLE_START),
        UNUSABLE_START...UNUSABLE_END => panic!("unusable memory area!"),
        // IO_PORTS_START...IO_PORTS_END => Addr::IoPorts(addr, addr - IO_PORTS_START),
        HIGH_RAM_START...HIGH_RAM_END => Addr::HighRam(addr, addr - HIGH_RAM_START),
        INTERRUPT_REGISTER_START...INTERRUPT_REGISTER_END => Addr::InterruptRegister,
        _ => {
          panic!("unrecognized memory mapped region: {:#04x}", addr);
        }
      }
    }
  }

  impl Memory for Mem {
    fn map(&mut self, start: u16, end: u16, mapper: Rc<RefCell<MemoryMap>>) {
      self.map.push((start, end, mapper));
    }

    fn set_booting(&mut self, value: bool) {
      self.booting = value;
    }

    fn set_boot_rom(&mut self, rom: Box<[u8]>) {
      self.set_booting(true);
      self.boot_rom = rom;
    }

    fn set_cart_rom(&mut self, rom: Box<[u8]>) {
      self.cart_rom = rom;
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

  impl MemoryMap for Mem {
    fn read_byte(&self, addr: u16) -> Option<u8> {
      // for i in self.map.iter() {
      //   if i.0 <= addr && addr <= i.1 {
      //     return i.2.borrow_mut().read_byte(addr);
      //   }
      // }

      let mapped = self.memory_map(addr);
      match mapped {
        Addr::Rom00(_, offset) => {
          if self.booting && offset <= 0xFF {
            self.boot_rom.get(offset as usize).and_then(|&x| Some(x))
          } else {
            self.cart_rom.get(offset as usize).and_then(|&x| Some(x))
          }
        }
        Addr::Rom01(_, offset) => panic!("read_byte not implemented: {:?}", mapped),
        Addr::VideoRam(_, offset) => panic!("read_byte not implemented: {:?}", mapped),
        Addr::ExternalRam(_, offset) => panic!("read_byte not implemented: {:?}", mapped),
        Addr::WorkRam0(_, offset) => self.work_ram_0.get(offset as usize).and_then(|&x| Some(x)),
        Addr::WorkRam1(_, offset) => self.work_ram_1.get(offset as usize).and_then(|&x| Some(x)),
        Addr::SpriteTable(_, offset) => panic!("read_byte not implemented: {:?}", mapped),
        Addr::IoPorts(_, offset) => panic!("read_byte not implemented: {:?}", mapped),
        Addr::HighRam(_, offset) => self.high_ram.get(offset as usize).and_then(|&x| Some(x)),
        Addr::InterruptRegister => panic!("read_byte not implemented: {:?}", mapped),
      }
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
      let mapped = self.memory_map(addr);
      match mapped {
        Addr::Rom00(_, offset) => {
          panic!("write_byte error: trying to write to rom0");
        }
        Addr::Rom01(_, offset) => panic!("write_byte not implemented: {:?}", mapped),
        Addr::VideoRam(_, offset) => panic!("write_byte not implemented: {:?}", mapped),
        Addr::ExternalRam(_, offset) => panic!("write_byte not implemented: {:?}", mapped),
        Addr::WorkRam0(_, offset) => {
          self.work_ram_0[offset as usize] = value;
        }
        Addr::WorkRam1(_, offset) => {
          self.work_ram_1[offset as usize] = value;
        }
        Addr::SpriteTable(_, offset) => panic!("write_byte not implemented: {:?}", mapped),
        Addr::IoPorts(_, offset) => panic!("write_byte not implemented: {:?}", mapped),
        Addr::HighRam(_, offset) => {
          self.high_ram[offset as usize] = value;
        }
        Addr::InterruptRegister => panic!("write_byte not implemented: {:?}", mapped),
      };
    }
  }
}


#[cfg(test)]
mod module {
  use std::rc::Rc;
  use std::cell::RefCell;

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

  impl MemoryMap for Mem {
    fn read_byte(&self, addr: u16) -> Option<u8> {
      self.ram.get(addr as usize).and_then(|&x| Some(x))
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
      self.ram[addr as usize] = value;
    }
  }

  impl Memory for Mem {
    fn map(&mut self, start: u16, end: u16, mapper: Rc<RefCell<MemoryMap>>) {}

    fn set_booting(&mut self, value: bool) {
      self.booting = value;
    }

    fn set_boot_rom(&mut self, rom: Box<[u8]>) {
      panic!("set_boot_rom should not be used for testing. use write_byte to write the rom to \
              memory");
    }

    fn set_cart_rom(&mut self, rom: Box<[u8]>) {
      panic!("set_cart_rom should not be used for testing. use write_byte to write the rom to \
              memory");
    }
  }
}
