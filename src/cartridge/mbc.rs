use num;

use super::super::mem::MemoryIo;
use super::ram;
use super::rom;

#[derive(Debug, PartialEq, NumFromPrimitive)]
#[allow(enum_variant_names)]
enum Mode {
  RomBank = 0x00,
  RamBank = 0x01,
}

#[derive(Debug)]
#[allow(enum_variant_names)]
pub enum MbcType {
  None,
  Mbc1,
  Mbc2,
  Mbc3,
  Mbc5,
}

pub struct Mbc {
  mbc_type: MbcType,
  rom: Box<[u8]>,
  ram: Vec<u8>,
  ram_enabled: bool,
  rom_bank_lower: u8,
  bank_upper: u8,
  mode: Mode,
}

impl Default for Mbc {
  fn default() -> Mbc {
    Mbc {
      mbc_type: MbcType::None,
      rom: Box::new([]),
      ram: vec![],
      ram_enabled: false,
      rom_bank_lower: 0x1,
      bank_upper: 0,
      mode: Mode::RomBank,
    }
  }
}

impl MemoryIo for Mbc {
  fn read_byte(&self, addr: u16) -> Result<u8, String> {
    match self.mbc_type {
      MbcType::None => {
        match addr {
          0...0x7FFF => Ok(self.rom[addr as usize]),
          0xA000...0xBFFF => {
            if !self.ram_enabled {
              return Ok(0);
            }

            Ok(self.rom[(addr as usize) - 0xA000])
          }
          _ => Ok(0),
        }
      }
      MbcType::Mbc1 => {
        match addr {
          // First 16Kbytes of cartrige ROM.
          0x0000...0x3FFF => Ok(self.rom[addr as usize]),
          0x4000...0x7FFF => {
            let mut bank = self.rom_bank_lower;
            if self.mode == Mode::RomBank {
              bank |= self.bank_upper << 4;
            }
            let mut loc = (addr as usize) - 0x4000;
            loc += (bank as usize) * rom::ROM_BANK_SIZE;

            // println!("reading mem loc: {:#04x}, addr: {:#04x}, bank: {:#04x}, value: {:#02x}",
            //          loc,
            //          addr,
            //          bank,
            //          self.rom[loc]);
            Ok(self.rom[loc])
          }
          0xA000...0xBFFF => {
            if !self.ram_enabled {
              return Ok(0);
            }
            let mut loc = (addr as usize) - 0xA000;
            loc += self.bank_upper as usize * ram::RAM_BANK_SIZE;
            Ok(self.ram[loc])
          }
          _ => Ok(0),
        }
      }
      _ => Err(format!("mbc.read_byte: unsupported mbc type: {:?}", self.mbc_type)),
    }
  }

  fn write_byte(&mut self, addr: u16, value: u8) -> Result<(), String> {
    match self.mbc_type {
      MbcType::None => Ok(()),
      MbcType::Mbc1 => {
        match addr {
          // RAM Enable (write only)
          0x0000...0x1FFF => {
            self.ram_enabled = value & 0b00001111 == 0x0a;
            Ok(())
          }
          0x2000...0x3FFF => {
            self.rom_bank_lower = value & 0x1f;
            if self.rom_bank_lower == 0x00 {
              self.rom_bank_lower = 0x01;
            }
            Ok(())
          }
          0x4000...0x5FFF => {
            self.bank_upper = value & 0x03;
            Ok(())
          }
          0x6000...0x7FFF => {
            self.mode = match num::FromPrimitive::from_u8(value & 0x01) {
              Some(v) => v,
              None => return Err(format!("mbc.write_byte: unsupported mode: {}", value & 0x01)),
            };
            Ok(())
          }
          0xA000...0xBFFF => {
            if !self.ram_enabled {
              return Err("mbc.write_byte: tried to write to Mbc1 ram when it wasn't enabled".to_owned());
            }

            let mut loc = addr as usize - 0xA000;
            if self.mode == Mode::RamBank {
              loc += self.bank_upper as usize * 0x2000;
            }
            self.ram[loc] = value;
            Ok(())
          }
          _ => {
            panic!("cartridge.write_byte: unhandled address: {:#04x}", addr);
          }
        }
      }
      _ => Err(format!("mbc.write_byte: unsupported mbc type: {:?}", self.mbc_type)),
    }
  }
}

impl Mbc {
  pub fn new() -> Mbc {
    Mbc::default()
  }

  pub fn load(&mut self, mbc_type: MbcType, rom: Box<[u8]>) -> Result<(), String> {
    self.mbc_type = mbc_type;

    let rom_size: rom::RomSize = match num::FromPrimitive::from_u8(rom[0x148]) {
      Some(v) => v,
      None => {
        return Err(format!("unsupported ram size: {:#02x}", rom[0x0148]));
      }
    };

    if rom_size.as_usize() != rom.len() {
      return Err(format!("unexpected rom size: Got: {}, Expected: {}",
                         rom.len(),
                         rom_size.as_usize()));
    }

    let ram_size: ram::RamSize = match num::FromPrimitive::from_u8(rom[0x149]) {
      Some(v) => v,
      None => {
        return Err(format!("unsupported ram size: {:#02x}", rom[0x0149]));
      }
    };

    self.rom = From::from(rom);
    self.ram = vec![0; ram_size.as_usize()];

    Ok(())
  }
}
