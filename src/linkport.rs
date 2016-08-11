use super::mem::MemoryIo;

pub struct LinkPort {
  last_byte: u8,
}

impl Default for LinkPort {
  fn default() -> LinkPort {
    LinkPort { last_byte: 0 }
  }
}

impl LinkPort {
  pub fn new() -> LinkPort {
    LinkPort { last_byte: 0 }
  }
}

impl MemoryIo for LinkPort {
  fn read_byte(&self, addr: u16) -> Result<u8, String> {
    Ok(self.last_byte)
  }

  fn write_byte(&mut self, addr: u16, value: u8) -> Result<(), String> {
    // println!("link write: {:#04x} {:x}", addr, value);
    match addr {
      0xff01 => {
        self.last_byte = value;
        Ok(())
      }
      0xff02 => {
        // if value == 0x81 {
        print!("{}", self.last_byte as char);
        // }
        Ok(())
      }
      _ => unreachable!(),
    }
  }
}
