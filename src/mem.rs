use std::rc::Rc;
use std::cell::RefCell;

pub use self::module::Mem;

pub const BOOT_ROM_START: u16 = 0x0000;
pub const BOOT_ROM_END: u16 = 0xFF;

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

// Memory location where it can set if the system is booting or not.
// 0x1 -> Not booting
// 0x0 -> Booting
pub const BOOTING_FLAG: u16 = 0xFF50;

#[derive(Debug)]
pub enum Addr {
  BootRom(u16, u16),
  WorkRam0(u16, u16),
  WorkRam1(u16, u16),
  SpriteTable(u16, u16),
  IoPorts(u16, u16),
  Unusable(u16, u16),
  HighRam(u16, u16),
  BootingFlag(u16, u16),
  InterruptRegister,
}

pub trait MemoryIo {
  fn read_byte(&self, addr: u16) -> Result<u8, String>;
  fn write_byte(&mut self, addr: u16, value: u8) -> Result<(), String>;

  fn read_vec(&self, addr: u16, len: u16) -> Result<Vec<u8>, String> {
    let mut v = vec![];
    for i in addr..addr + len {
      match self.read_byte(i) {
        Ok(x) => v.push(x),
        Err(e) => return Err(e),
      }
    }

    if v.is_empty() {
      Err("length of vec read is 0".to_owned())
    } else {
      Ok(v)
    }
  }

  // TODO: Maybe allow MemoryIO objects implement this directly,
  // so it doesn't need to call read_byte twice and instead just
  // read the word directly. Also would need a read_u32 or something
  // similar for performance reasons.
  fn read_word(&self, addr: u16) -> Result<u16, String> {
    let mut val: u16 = match self.read_byte(addr + 1) {
      Ok(x) => (x as u16) << 8,
      Err(e) => return Err(e),
    };
    val |= match self.read_byte(addr) {
      Ok(x) => x as u16,
      Err(e) => return Err(e),
    };
    Ok(val)
  }

  fn write_word(&mut self, addr: u16, value: u16) -> Result<(), String> {
    try!(self.write_byte(addr + 1, (value >> 8) as u8 & 0b11111111));
    try!(self.write_byte(addr, value as u8 & 0b11111111));
    Ok(())
  }
}

pub trait Memory: MemoryIo {
  fn map(&mut self, start: u16, end: u16, mapper: Rc<RefCell<MemoryIo>>);
  fn set_boot_rom(&mut self, rom: Box<[u8]>);
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
    booting: bool,

    work_ram_0: [u8; WORK_RAM_0_LEN + 1],
    work_ram_1: [u8; WORK_RAM_1_LEN + 1],

    high_ram: [u8; HIGH_RAM_LEN + 1],

    map: Vec<(u16, u16, Rc<RefCell<MemoryIo>>)>,

    interrupt_enable: u8,
  }

  impl Default for Mem {
    fn default() -> Mem {
      Mem {
        boot_rom: Box::new([]),
        booting: false,
        work_ram_0: [0; WORK_RAM_0_LEN + 1],
        work_ram_1: [0; WORK_RAM_1_LEN + 1],
        high_ram: [0; HIGH_RAM_LEN + 1],
        map: Vec::new(),
        interrupt_enable: 0,
      }
    }
  }

  impl Mem {
    pub fn new() -> Mem {
      Mem::default()
    }

    pub fn memory_map(&self, addr: u16) -> Addr {
      match addr {
        BOOT_ROM_START...BOOT_ROM_END => Addr::BootRom(addr, addr - BOOT_ROM_START),
        WORK_RAM_0_START...WORK_RAM_0_END => Addr::WorkRam0(addr, addr - WORK_RAM_0_START),
        WORK_RAM_1_START...WORK_RAM_1_END => Addr::WorkRam1(addr, addr - WORK_RAM_1_START),
        ECHO_START...ECHO_END => self.memory_map(addr - ECHO_START + WORK_RAM_0_START),
        UNUSABLE_START...UNUSABLE_END => Addr::Unusable(addr, addr - UNUSABLE_START),
        HIGH_RAM_START...HIGH_RAM_END => Addr::HighRam(addr, addr - HIGH_RAM_START),
        BOOTING_FLAG => Addr::BootingFlag(addr, addr - BOOTING_FLAG),
        INTERRUPT_REGISTER_START...INTERRUPT_REGISTER_END => Addr::InterruptRegister,
        IO_PORTS_START...IO_PORTS_END => Addr::IoPorts(addr, addr - IO_PORTS_START),
        _ => {
          panic!("unrecognized memory mapped region: {:#04x}", addr);
        }
      }
    }
  }

  impl Memory for Mem {
    fn map(&mut self, start: u16, end: u16, mapper: Rc<RefCell<MemoryIo>>) {
      self.map.push((start, end, mapper));
    }

    fn set_boot_rom(&mut self, rom: Box<[u8]>) {
      self.booting = true;
      self.boot_rom = rom;
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

  impl MemoryIo for Mem {
    fn read_byte(&self, addr: u16) -> Result<u8, String> {
      // If we're not booting or if the address is greater than
      // 0xFF, then we need to check our mappings to see
      // if this address exists in there.
      if !self.booting || addr >= BOOT_ROM_END {
        for i in &self.map {
          if i.0 <= addr && addr <= i.1 {
            return i.2.borrow_mut().read_byte(addr);
          }
        }
      }

      let mapped = self.memory_map(addr);
      match mapped {
        Addr::BootRom(_, offset) => {
          // If we get to this point, it means we're booting and/or the address
          // is less than 0xFF.
          self.boot_rom
            .get(offset as usize)
            .ok_or_else(|| format!("could not get byte at boot_rom offset {}", offset))
            .and_then(|&x| Ok(x))
        }
        Addr::WorkRam0(_, offset) => {
          self.work_ram_0
            .get(offset as usize)
            .ok_or_else(|| format!("could not get byte at work_ram_0 offset {}", offset))
            .and_then(|&x| Ok(x))
        }
        Addr::WorkRam1(_, offset) => {
          self.work_ram_1
            .get(offset as usize)
            .ok_or_else(|| format!("could not get byte at work_ram_1 offset {}", offset))
            .and_then(|&x| Ok(x))
        }
        Addr::SpriteTable(_, _) => Err(format!("read_byte not implemented: {:?}", mapped)),
        Addr::IoPorts(_, _) => {
          // Err(format!("read_byte not implemented: {:?}", mapped)),
          Ok((0))
        }
        Addr::Unusable(_, _) => {
          // println!("read_byte occurred at unusable memory addr: {:#04x}", addr);
          Ok((0))
        }
        Addr::HighRam(_, offset) => {
          self.high_ram
            .get(offset as usize)
            .ok_or_else(|| format!("could not get byte at high_ram offset {}", offset))
            .and_then(|&x| Ok(x))
        }
        Addr::BootingFlag(_, _) => {
          // Err(format!("the booting flag shouldn't need to be read: {:?}", mapped))
          if self.booting {
            Ok((0))
          } else {
            Ok((1))
          }
        }
        Addr::InterruptRegister => Ok(self.interrupt_enable),
      }
    }

    fn write_byte(&mut self, addr: u16, value: u8) -> Result<(), String> {
      for i in &self.map {
        if i.0 <= addr && addr <= i.1 {
          return i.2.borrow_mut().write_byte(addr, value);
        }
      }

      let mapped = self.memory_map(addr);
      match mapped {
        Addr::BootRom(_, _) => Err("mem.write_byte: shouldn't be writing to boot rom".to_owned()),
        Addr::WorkRam0(_, offset) => {
          self.work_ram_0[offset as usize] = value;
          Ok(())
        }
        Addr::WorkRam1(_, offset) => {
          self.work_ram_1[offset as usize] = value;
          Ok(())
        }
        Addr::SpriteTable(_, _) => {
          Err(format!("write_byte Addr::SpriteTable not implemented: {:?}", mapped))
        }
        Addr::IoPorts(_, _) => {
          // Err(format!("write_byte Addr::IOPorts not implemented: {:?}", mapped))
          Ok(())
        }
        Addr::Unusable(_, _) => {
          // println!("write_byte occurred at unusable memory addr: {:#04x}", addr);
          Ok(())
        }
        Addr::HighRam(_, offset) => {
          self.high_ram[offset as usize] = value;
          Ok(())
        }
        Addr::BootingFlag(_, _) => {
          self.booting = value == 0;
          Ok(())
        }
        Addr::InterruptRegister => {
          self.interrupt_enable = value;
          Ok(())
        }
      }
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
    // boot_rom: Box<[u8]>,
    // cart_rom: Box<[u8]>,
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
        // boot_rom: Box::new([]),
        // cart_rom: Box::new([]),
        booting: false,
        ram: [0; 0xFFFF + 1],
      }
    }
  }

  impl MemoryIo for Mem {
    fn read_byte(&self, addr: u16) -> Result<u8, String> {
      self.ram
        .get(addr as usize)
        .ok_or(format!("could not get byte at test ram offset {}", addr))
        .and_then(|&x| Ok(x))
    }

    fn write_byte(&mut self, addr: u16, value: u8) -> Result<(), String> {
      self.ram[addr as usize] = value;
      Ok(())
    }
  }

  impl Memory for Mem {
    fn map(&mut self, _: u16, _: u16, _: Rc<RefCell<MemoryIo>>) {}

    fn set_boot_rom(&mut self, _: Box<[u8]>) {
      panic!("set_boot_rom should not be used for testing. use write_byte to write the rom to \
              memory");
    }
  }
}
