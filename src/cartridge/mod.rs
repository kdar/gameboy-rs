use std::path::Path;
use std::fs::File;
use std::io::Read;
use num;

mod ram;
mod rom;

use super::mem::MemoryIo;

#[allow(enum_variant_names)]
enum Mbc {
  None,
  Mbc1,
  Mbc2,
  Mbc3,
  Mbc5,
}

#[derive(PartialEq, Debug, NumFromPrimitive)]
enum CartType {
  RomOnly = 0x00,
  Mbc1 = 0x01,
  Mbc1Ram = 0x02,
  Mbc1RamBattery = 0x03,
  Mbc2 = 0x05,
  Mbc2Battery = 0x06,
  RomRam = 0x08,
  RomRamBattery = 0x09,
  Mmm01 = 0x0B,
  Mmm01Ram = 0x0C,
  Mmm01RamBattery = 0x0D,
  Mbc3TimerBattery = 0x0F,
  Mbc3TimerRamBattery = 0x10,
  Mbc3 = 0x11,
  Mbc3Ram = 0x12,
  Mbc3RamBattery = 0x13,
  Mbc4 = 0x15,
  Mbc4Ram = 0x16,
  Mbc4RamBattery = 0x17,
  Mbc5 = 0x19,
  Mbc5Ram = 0x1A,
  Mbc5RamBattery = 0x1B,
  Mbc5Rumble = 0x1C,
  Mbc5RumbleRam = 0x1D,
  Mbc5RumbleRamBattery = 0x1E,
  PocketCamera = 0xFC,
  BandaiTama5 = 0xFD,
  Huc3 = 0xFE,
  Huc1RamBattery = 0xFF,
}

impl CartType {
  fn as_mbc(&self) -> Mbc {
    use self::CartType::*;
    match *self {
      RomOnly | RomRam | RomRamBattery => Mbc::None,
      Mbc1 | Mbc1Ram | Mbc1RamBattery => Mbc::Mbc1,
      Mbc2 | Mbc2Battery => Mbc::Mbc2,
      Mbc3 |
      Mbc3Ram |
      Mbc3RamBattery |
      Mbc3TimerBattery |
      Mbc3TimerRamBattery => Mbc::Mbc3,
      Mbc5 |
      Mbc5Ram |
      Mbc5RamBattery |
      Mbc5Rumble |
      Mbc5RumbleRam |
      Mbc5RumbleRamBattery => Mbc::Mbc5,
      _ => panic!("unknown mbc type"),
    }
  }
}

pub struct Cartridge {
  rom: Vec<u8>,
  rom_bank: usize,
  rom_banks: usize,
  ram: Vec<u8>,
  ram_offset: usize,
  mbc: Mbc,
  cart_type: CartType,
  title: String,
}

impl MemoryIo for Cartridge {
  fn read_byte(&self, addr: u16) -> Result<u8, String> {
    if addr as usize >= self.rom.len() {
      return Err(format!("cartridge.read_byte: tried to read at #{:04x} when the ROM size is \
                          only #{:04x}",
                         addr,
                         self.rom.len()));
    }

    Ok(self.rom[addr as usize])
  }

  fn write_byte(&mut self, addr: u16, value: u8) -> Result<(), String> {
    panic!("writing to cartridge is illegal!");
  }
}

impl Default for Cartridge {
  fn default() -> Cartridge {
    Cartridge {
      rom: vec![],
      rom_bank: 0,
      rom_banks: 0,
      ram: vec![],
      ram_offset: 0,
      mbc: Mbc::None,
      cart_type: CartType::RomOnly,
      title: "".to_owned(),
    }
  }
}

impl Cartridge {
  pub fn new() -> Cartridge {
    Cartridge::default()
  }

  pub fn load_data(&mut self, data: &[u8]) -> Result<(), String> {
    if data.len() < 0x014F {
      return Err("invalid cartridge: too small".to_owned());
    }

    self.rom = From::from(data);

    self.cart_type = match num::FromPrimitive::from_u8(data[0x0147]) {
      Some(v) => v,
      None => {
        return Err(format!("unknown cartridge type: {:#02x}", data[0x0147]));
      }
    };

    let rom_size: rom::CartRomSize = match num::FromPrimitive::from_u8(data[0x148]) {
      Some(v) => v,
      None => {
        return Err(format!("unsupported ram size: {:#02x}", data[0x0148]));
      }
    };

    if rom_size.as_usize() != data.len() {
      return Err(format!("unexpected rom size: Got: {}, Expected: {}",
                         data.len(),
                         rom_size.as_usize()));
    }

    self.rom_banks = rom_size.banks();

    let ram_size: ram::CartRamSize = match num::FromPrimitive::from_u8(data[0x149]) {
      Some(v) => v,
      None => {
        return Err(format!("unsupported ram size: {:#02x}", data[0x0149]));
      }
    };

    self.mbc = self.cart_type.as_mbc();
    let ram_size = match self.mbc {
      Mbc::Mbc2 => 512,
      _ => ram_size.as_usize(),
    };

    self.ram = vec![0; ram_size];

    let new_cartridge = self.rom[0x14b] == 0x33;
    let title = if new_cartridge {
      &data[0x134..0x13f]
    } else {
      &data[0x134..0x144]
    };

    self.title = String::from_utf8_lossy(title).into_owned();

    self.load_mbc();

    Ok(())
  }

  pub fn load_path<P: AsRef<Path>>(&mut self, path: P) -> Result<(), String> {
    let mut f = try!(File::open(path).map_err(|e| format!("{}", e)));
    let mut v = vec![];
    try!(f.read_to_end(&mut v).map_err(|e| format!("{}", e)));

    self.load_data(&v.as_slice())
  }

  fn load_mbc(&mut self) {}
}

#[cfg(test)]
mod test {
  use super::*;
  use super::CartType;

  #[test]
  fn test_from_data() {
    let test_cart =
      [0xf5, 0x2a, 0x12, 0x13, 0x05, 0x20, 0xfa, 0xf1, 0xc9, 0x3e, 0x20, 0xe0, 0x00, 0xf0, 0x00,
       0xf0, 0x00, 0x2f, 0xe6, 0x0f, 0xcb, 0x37, 0x47, 0x3e, 0x10, 0xe0, 0x00, 0xf0, 0x00, 0xf0,
       0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0x2f, 0xe6, 0x0f, 0xb0, 0x47, 0xfa,
       0xa6, 0xc1, 0x2f, 0xa0, 0xea, 0xa7, 0xc1, 0x78, 0xea, 0xa6, 0xc1, 0x3e, 0x30, 0xe0, 0x00,
       0xc9, 0xff, 0xff, 0xff, 0xc3, 0x61, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xd9, 0xff, 0xff,
       0xff, 0xff, 0xff, 0xff, 0xff, 0xd9, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xd9, 0xff,
       0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xd9, 0xf3, 0xf5, 0x3e, 0xc0, 0xe0, 0x46, 0x3e, 0x28,
       0x3d, 0x20, 0xfd, 0x21, 0x00, 0xfe, 0xcd, 0x86, 0x00, 0xfa, 0xa3, 0xc0, 0x3c, 0xea, 0xa3,
       0xc0, 0xcd, 0x21, 0x04, 0xfa, 0xa2, 0xc0, 0x3c, 0xea, 0xa2, 0xc0, 0xf1, 0xfb, 0xd9, 0xe5,
       0x21, 0xa6, 0xc0, 0x2a, 0xfe, 0x00, 0x28, 0x5c, 0xfe, 0x01, 0x20, 0x05, 0x2a, 0xe0, 0x43,
       0x18, 0xf2, 0xfe, 0x02, 0x20, 0x05, 0x2a, 0xe0, 0x42, 0x18, 0xe9, 0xfe, 0x03, 0x20, 0x19,
       0xc5, 0xd5, 0x2a, 0x57, 0x2a, 0x5f, 0x2a, 0x47, 0x2a, 0x4f, 0x2a, 0xe5, 0x6f, 0x61, 0x2a,
       0x12, 0x13, 0x05, 0x20, 0xfa, 0xe1, 0xd1, 0xc1, 0x18, 0xcc, 0xfe, 0x04, 0x20, 0x26, 0xc5,
       0xd5, 0x2a, 0x57, 0x2a, 0x5f, 0x2a, 0x47, 0x2a, 0x4f, 0x2a, 0xe5, 0x6f, 0x61, 0x7e, 0x12,
       0x7d, 0xc6, 0x10, 0x30, 0x01, 0x24, 0x6f, 0x7b, 0xc6, 0x20, 0x30, 0x01, 0x14, 0x5f, 0x05,
       0x20, 0xed, 0xe1, 0xd1, 0xc1, 0x18, 0xa2, 0xc3, 0x8a, 0x00, 0x3e, 0x00, 0xea, 0xa6, 0xc0,
       0xea, 0xa5, 0xc0, 0xe1, 0xc9, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
       0xff, 0x00, 0xc3, 0x50, 0x01, 0xce, 0xed, 0x66, 0x66, 0xcc, 0x0d, 0x00, 0x0b, 0x03, 0x73,
       0x00, 0x83, 0x00, 0x0c, 0x00, 0x0d, 0x00, 0x08, 0x11, 0x1f, 0x88, 0x89, 0x00, 0x0e, 0xdc,
       0xcc, 0x6e, 0xe6, 0xdd, 0xdd, 0xd9, 0x99, 0xbb, 0xbb, 0x67, 0x63, 0x6e, 0x0e, 0xec, 0xcc,
       0xdd, 0xdc, 0x99, 0x9f, 0xbb, 0xb9, 0x33, 0x3e, 0x4f, 0x70, 0x75, 0x73, 0x20, 0x54, 0x65,
       0x73, 0x74, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
       0x00, 0x00, 0x01, 0x9f, 0x1b, 0xab];


    let mut cartridge = Cartridge::default();

    let result = cartridge.load_data(&test_cart);
    assert!(result.is_ok());

    assert_eq!(cartridge.cart_type, CartType::RomOnly);
    assert_eq!(cartridge.title, "Opus Test       ");
  }
}
