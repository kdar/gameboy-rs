use std::fmt;
use md5;
use std::any::Any;

use super::bios::Bios;
use super::cartridge::Cartridge;
use super::mem::MemoryIo;
use super::video::Video;
use super::audio::Audio;
use super::linkport::LinkPort;

pub const WORK_RAM_0_LEN: usize = 0xcfff - 0xc000;
pub const WORK_RAM_1_LEN: usize = 0xdfff - 0xd000;
pub const HIGH_RAM_LEN: usize = 0xfffe - 0xff80;

pub struct Config {
  cfg_boot_rom: Option<Box<[u8]>>,
  cfg_cart_rom: Box<[u8]>,
}

impl Default for Config {
  fn default() -> Config {
    Config {
      cfg_boot_rom: None,
      cfg_cart_rom: Box::new([]),
    }
  }
}

impl Config {
  pub fn new() -> Config {
    Config::default()
  }

  pub fn boot_rom(mut self, boot_rom: Option<Box<[u8]>>) -> Config {
    self.cfg_boot_rom = boot_rom;
    self
  }

  pub fn cart_rom(mut self, cart_rom: Box<[u8]>) -> Config {
    self.cfg_cart_rom = cart_rom;
    self
  }

  pub fn create(self) -> Result<Box<SystemCtrl>, String> {
    let mut s = System::new();
    try!(s.bios.load(self.cfg_boot_rom));
    // self.cfg_boot_rom = None;
    try!(s.cartridge.load(self.cfg_cart_rom));
    // self.cfg_cart_rom = Box::new([]);

    Ok(Box::new(s))
  }
}

pub trait SystemCtrl: MemoryIo {
  fn step(&mut self) {}
  fn as_memoryio(&self) -> &MemoryIo;
}

pub struct System {
  bios: Bios,
  cartridge: Cartridge,
  video: Video,
  audio: Audio,
  linkport: LinkPort,

  work_ram_0: [u8; WORK_RAM_0_LEN + 1],
  work_ram_1: [u8; WORK_RAM_1_LEN + 1],

  high_ram: [u8; HIGH_RAM_LEN + 1],

  interrupt_enable: u8,
  booting: bool,
}

impl Default for System {
  fn default() -> System {
    System {
      bios: Bios::default(),
      cartridge: Cartridge::default(),
      video: Video::default(),
      audio: Audio::default(),
      linkport: LinkPort::default(),
      work_ram_0: [0; WORK_RAM_0_LEN + 1],
      work_ram_1: [0; WORK_RAM_1_LEN + 1],
      high_ram: [0; HIGH_RAM_LEN + 1],
      interrupt_enable: 0,
      booting: true,
    }
  }
}

impl fmt::Debug for System {
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

impl MemoryIo for System {
  fn read_u8(&self, addr: u16) -> Result<u8, String> {
    match addr {
      // boot / cart rom
      0x0000...0x3fff => {
        if self.booting && addr < 0xFF {
          self.bios.read_u8(addr)
        } else {
          self.cartridge.read_u8(addr)
        }
      }
      // cart rom 01
      0x4000...0x7fff => self.cartridge.read_u8(addr),
      // video ram
      0x8000...0x9fff => self.video.read_u8(addr),
      // cart ram
      0xa000...0xbfff => self.cartridge.read_u8(addr),
      // sprite table
      0xfe00...0xfe9f => self.video.read_u8(addr),
      // audio
      0xff10...0xff3f => self.audio.read_u8(addr),
      // video control
      0xff40...0xff4c => self.video.read_u8(addr),
      // link port
      0xff01...0xff02 => self.linkport.read_u8(addr),
      // echo
      0xe000...0xfdff => self.read_u8(addr - 0xe000 + 0xc000),

      // work ram 0
      0xc000...0xcfff => {
        self.work_ram_0
          .get((addr - 0xc000) as usize)
          .ok_or_else(|| {
            format!("system.read_u8: could not get byte at work_ram_0 addr {}",
                    addr)
          })
          .and_then(|&x| Ok(x))
      }
      // work ram 1
      0xd000...0xdfff => {
        self.work_ram_1
          .get((addr - 0xd000) as usize)
          .ok_or_else(|| {
            format!("system.read_u8: could not get byte at work_ram_1 addr {}",
                    addr)
          })
          .and_then(|&x| Ok(x))
      }
      // unusuable
      0xfea0...0xfeff => {
        // println!("read_u8 occurred at unusable memory addr: {:#04x}", addr);
        Ok((0))
      }
      // high ram
      0xff80...0xfffe => {
        self.high_ram
          .get((addr - 0xff80) as usize)
          .ok_or_else(|| {
            format!("system.read_u8: could not get byte at high_ram addr {}",
                    addr)
          })
          .and_then(|&x| Ok(x))
      }
      // booting flag
      0xff50 => {
        // Err(format!("the booting flag shouldn't need to be read: {:?}", mapped))
        if self.booting {
          Ok((0))
        } else {
          Ok((1))
        }
      }
      // interrupt enable
      0xffff => Ok(self.interrupt_enable),
      // io ports
      0xff00...0xff7f => Ok((0)),
      _ => Err(format!("system.read_u8: unknown mapped addr: {:#04x}", addr)),
    }
  }

  fn write_u8(&mut self, addr: u16, value: u8) -> Result<(), String> {
    match addr {
      // boot / cart rom
      0x0000...0x3fff => {
        if self.booting && addr < 0xFF {
          Err("system.write_u8: shouldn't be writing to boot rom".to_owned())
        } else {
          self.cartridge.write_u8(addr, value)
        }
      }
      // cart rom 01
      0x4000...0x7fff => self.cartridge.write_u8(addr, value),
      // video ram
      0x8000...0x9fff => self.video.write_u8(addr, value),
      // cart ram
      0xa000...0xbfff => self.cartridge.write_u8(addr, value),
      // sprite table
      0xfe00...0xfe9f => self.video.write_u8(addr, value),
      // audio
      0xff10...0xff3f => self.audio.write_u8(addr, value),
      // video control
      0xff40...0xff4c => self.video.write_u8(addr, value),
      // link port
      0xff01...0xff02 => self.linkport.write_u8(addr, value),
      // echo
      0xe000...0xfdff => self.write_u8(addr - 0xe000 + 0xc000, value),

      // work ram 0
      0xc000...0xcfff => {
        self.work_ram_0[(addr - 0xc000) as usize] = value;
        Ok(())
      }
      // work ram 1
      0xd000...0xdfff => {
        self.work_ram_1[(addr - 0xd000) as usize] = value;
        Ok(())
      }
      // unusuable
      0xfea0...0xfeff => {
        // println!("write_u8 occurred at unusable memory addr: {:#04x}", addr);
        Ok(())
      }
      // high ram
      0xff80...0xfffe => {
        self.high_ram[(addr - 0xff80) as usize] = value;
        Ok(())
      }
      // booting flag
      0xff50 => {
        self.booting = value == 0;
        Ok(())
      }
      // interrupt enable
      0xffff => {
        self.interrupt_enable = value;
        Ok(())
      }
      // io ports
      0xff00...0xff7f => {
        // Err(format!("write_u8 Addr::IOPorts not implemented: {:?}", mapped))
        Ok(())
      }
      _ => Err(format!("system.write_u8: unknown mapped addr: {:#04x}", addr)),
    }
  }
}

impl System {
  pub fn new() -> System {
    System::default()
  }
}

impl SystemCtrl for System {
  fn step(&mut self) {
    self.video.step();
  }

  fn as_memoryio(&self) -> &MemoryIo {
    self as &MemoryIo
  }
}
