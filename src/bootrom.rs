use super::mem::MemoryIo;

// Bootrom holds the data of the bootrom. This is a simple
// structure that provides a way to read from the Bootrom
// and determine if it's enabled or not.
pub struct Bootrom {
  rom: Box<[u8]>,
  enabled: bool,
}

impl Default for Bootrom {
  fn default() -> Bootrom {
    Bootrom {
      rom: Box::new([]),
      enabled: false,
    }
  }
}

impl Bootrom {
  pub fn new(rom: Option<Box<[u8]>>) -> Bootrom {
    if let Some(r) = rom {
      Bootrom {
        rom: r,
        enabled: true,
      }
    } else {
      Bootrom::default()
    }
  }

  pub fn is_enabled(&self) -> bool {
    self.enabled
  }
}

impl MemoryIo for Bootrom {
  fn read_byte(&self, addr: u16) -> Result<u8, String> {
    if !self.is_enabled() {
      panic!("bootom.read_byte: tried to read with the bootrom not enabled");
    }
    Ok(self.rom[addr as usize])
  }
}
