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

  pub fn load(&mut self, rom: Option<Box<[u8]>>) -> Result<(), String> {
    if let Some(r) = rom {
      self.enabled = true;
      self.rom = r;
    }

    Ok(())
  }

  pub fn is_enabled(&self) -> bool {
    self.enabled
  }
}

impl MemoryIo for Bios {
  fn read_byte(&self, addr: u16) -> Result<u8, String> {
    if !self.is_enabled() {
      panic!("bootom.read_byte: tried to read with the bootrom not enabled");
    }
    Ok(self.rom[addr as usize])
  }
}
