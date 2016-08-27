use std::fmt;
use md5;
use std::sync::mpsc::Sender;

use super::bios::Bios;
use super::cartridge::Cartridge;
use super::mem::MemoryIo;
use super::video::Video;
use super::audio::Audio;
use super::linkport::LinkPort;
use super::GbEvent;
use super::pic::{Pic, Interrupt};
use super::timer::Timer;
use super::gamepad::Gamepad;

pub const WORK_RAM_0_LEN: usize = 0xcfff - 0xc000;
pub const WORK_RAM_1_LEN: usize = 0xdfff - 0xd000;
pub const HIGH_RAM_LEN: usize = 0xfffe - 0xff80;

struct Dma {
  src: u16,
  dst: u16,
  state: DmaState,
}

impl Default for Dma {
  fn default() -> Dma {
    Dma {
      src: 0,
      dst: 0,
      state: DmaState::Stopped,
    }
  }
}

impl Dma {
  fn start(&mut self, addr_high: u8) {
    self.state = DmaState::Starting;
    self.src = (addr_high as u16) << 8;
  }
}

enum DmaState {
  Started,
  Stopped,
  Starting,
}

pub struct Config {
  cfg_boot_rom: Option<Box<[u8]>>,
  cfg_cart_rom: Box<[u8]>,
  cfg_frame_sender: Option<Sender<Vec<[u8; 4]>>>,
}

impl Default for Config {
  fn default() -> Config {
    Config {
      cfg_boot_rom: None,
      cfg_cart_rom: Box::new([]),
      cfg_frame_sender: None,
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

  pub fn frame_sender(mut self, s: Sender<Vec<[u8; 4]>>) -> Config {
    self.cfg_frame_sender = Some(s);
    self
  }

  pub fn create(self) -> Result<Box<SystemCtrl + Send>, String> {
    let mut s = System::new();
    try!(s.bios.load(self.cfg_boot_rom));
    // self.cfg_boot_rom = None;
    try!(s.cartridge.load(self.cfg_cart_rom));
    // self.cfg_cart_rom = Box::new([]);

    if self.cfg_frame_sender.is_some() {
      s.video.set_frame_sender(self.cfg_frame_sender.unwrap());
    }

    Ok(Box::new(s))
  }
}

pub trait SystemCtrl: MemoryIo {
  fn step(&mut self) {}
  fn as_memoryio(&self) -> &MemoryIo;
  fn debug(&self) {}
  fn next_interrupt(&mut self) -> Option<Interrupt> {
    None
  }
  fn has_interrupt(&self) -> bool;
}

pub struct System {
  bios: Bios,
  cartridge: Cartridge,
  video: Video,
  audio: Audio,
  linkport: LinkPort,
  dma: Dma,
  pic: Pic,
  timer: Timer,
  gamepad: Gamepad,

  work_ram_0: [u8; WORK_RAM_0_LEN + 1],
  work_ram_1: [u8; WORK_RAM_1_LEN + 1],

  high_ram: [u8; HIGH_RAM_LEN + 1],

  booting: bool,
}

impl Default for System {
  fn default() -> System {
    System {
      bios: Bios::default(),
      cartridge: Cartridge::default(),
      video: Video::new(),
      audio: Audio::default(),
      linkport: LinkPort::default(),
      dma: Dma::default(),
      pic: Pic::default(),
      timer: Timer::default(),
      gamepad: Gamepad::default(),
      work_ram_0: [0; WORK_RAM_0_LEN + 1],
      work_ram_1: [0; WORK_RAM_1_LEN + 1],
      high_ram: [0; HIGH_RAM_LEN + 1],
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
    // try!(write!(f, "\nVideo\n{}", self.video));
    write!(f, "\n")
  }
}

impl MemoryIo for System {
  fn read_u8(&self, addr: u16) -> Result<u8, String> {
    match addr {
      // boot / cart rom
      0x0000...0x3fff => {
        if self.booting && addr < 0x100 {
          self.bios.read_u8(addr)
        } else {
          self.cartridge.read_u8(addr)
        }
      }
      // cart rom 01 | cart ram
      0x4000...0x7fff | 0xa000...0xbfff => self.cartridge.read_u8(addr),
      // video ram | sprite table
      0x8000...0x9fff | 0xfe00...0xfe9f => self.video.read_u8(addr),
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
      // echo
      0xe000...0xfdff => self.read_u8(addr - 0xe000 + 0xc000),
      // Unused
      0xfea0...0xfeff => Ok(0),
      0xff00...0xffff => {
        match addr {
          // gamepad
          0xff00 => self.gamepad.read_u8(addr),
          // link port
          0xff01...0xff02 => self.linkport.read_u8(addr),
          // timer
          0xff04...0xff07 => self.timer.read_u8(addr),
          // interrupt flags | interrupt enable
          0xff0f | 0xffff => self.pic.read_u8(addr),
          // audio
          0xff10...0xff3f => self.audio.read_u8(addr),
          // video control
          0xff40...0xff45 | 0xff47...0xff4c => self.video.read_u8(addr),
          // DMA transfer
          0xff46 => panic!("reading dma transfer register?"),
          // GBC mode
          // 0xff4d => Ok(0),
          // booting flag
          0xff50 => {
            // Err(format!("the booting flag shouldn't need to be read: {:?}", mapped))
            if self.booting { Ok((0)) } else { Ok((1)) }
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
          _ => Ok(0),
        }
      }
      _ => Err(format!("system.read_u8: unknown mapped addr: {:#04x}", addr)),
    }
  }

  fn write_u8(&mut self, addr: u16, value: u8) -> Result<(), String> {
    match addr {
      // boot / cart rom
      0x0000...0x3fff => {
        if self.booting && addr < 0x100 {
          Err("system.write_u8: shouldn't be writing to boot rom".to_owned())
        } else {
          self.cartridge.write_u8(addr, value)
        }
      }
      // cart rom 01 | cart ram
      0x4000...0x7fff | 0xa000...0xbfff => self.cartridge.write_u8(addr, value),
      // video ram | sprite table
      0x8000...0x9fff | 0xfe00...0xfe9f => self.video.write_u8(addr, value),
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
      // echo
      0xe000...0xfdff => self.write_u8(addr - 0xe000 + 0xc000, value),
      // Unused
      0xfea0...0xfeff => Ok(()),
      0xff00...0xffff => {
        match addr {
          // gamepad
          0xff00 => self.gamepad.write_u8(addr, value),
          // link port
          0xff01...0xff02 => self.linkport.write_u8(addr, value),
          // timer
          0xff04...0xff07 => self.timer.write_u8(addr, value),
          // interrupt flags | interrupt enable
          0xff0f | 0xffff => self.pic.write_u8(addr, value),
          // audio
          0xff10...0xff3f => self.audio.write_u8(addr, value),
          // video control
          0xff40...0xff45 | 0xff47...0xff4b => self.video.write_u8(addr, value),
          // DMA transfer
          0xff46 => {
            self.dma.start(value);
            Ok(())
          }
          // GBC mode
          // 0xff4d => Ok(()),
          // booting flag
          0xff50 => {
            self.booting = value == 0;
            Ok(())
          }
          // high ram
          0xff80...0xfffe => {
            self.high_ram[(addr - 0xff80) as usize] = value;
            Ok(())
          }
          _ => Ok(()),
        }
      }
      _ => Err(format!("system.write_u8: unknown mapped addr: {:#04x}", addr)),
    }
  }
}

impl System {
  pub fn new() -> System {
    System::default()
  }

  pub fn dma_step(&mut self) {
    match self.dma.state {
      DmaState::Starting => {
        self.dma.dst = 0xfe00;
        self.dma.state = DmaState::Started;
      }
      DmaState::Started => {
        if self.dma.dst >= 0xfea0 {
          self.dma.dst = 0xfe00;
          self.dma.state = DmaState::Stopped;
        } else {
          let value = self.read_u8(self.dma.src).unwrap();
          let dst = self.dma.dst;
          self.write_u8(dst, value).unwrap();
          self.dma.src += 1;
          self.dma.dst += 1;
        }
      }
      DmaState::Stopped => {}
    };
  }
}

impl SystemCtrl for System {
  fn debug(&self) {
    self.video.debug();
  }

  fn step(&mut self) {
    self.video.step(&mut self.pic);
    self.dma_step();
    self.timer.step(&mut self.pic);
    self.gamepad.step(&mut self.pic);
  }

  fn as_memoryio(&self) -> &MemoryIo {
    self as &MemoryIo
  }

  fn next_interrupt(&mut self) -> Option<Interrupt> {
    self.pic.next_interrupt()
  }

  fn has_interrupt(&self) -> bool {
    self.pic.has_interrupt()
  }
}
