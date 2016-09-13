use super::mem::MemoryIo;

// Bios holds the data of the bootrom. This is a simple
// structure that provides a way to read from the Bios
// and determine if it's enabled or not.
pub struct Bios {
  rom: Box<[u8]>,
  enabled: bool,
}

impl Default for Bios {
  fn default() -> Bios {
    Bios {
      rom: Box::new([]),
      enabled: false,
    }
  }
}

impl Bios {
  pub fn new() -> Bios {
    Bios::default()
  }

  pub fn load(&mut self, rom: Box<[u8]>) -> Result<(), String> {
    self.enabled = true;
    self.rom = rom;

    Ok(())
  }

  pub fn is_enabled(&self) -> bool {
    self.enabled
  }
}

impl MemoryIo for Bios {
  fn read_u8(&self, addr: u16) -> Result<u8, String> {
    if !self.is_enabled() {
      panic!("bios.read_u8: tried to read with the bootrom not enabled");
    }
    Ok(self.rom[addr as usize])
  }
}
