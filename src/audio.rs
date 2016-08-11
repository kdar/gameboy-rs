use super::mem::MemoryIo;

pub struct Audio;

impl Default for Audio {
  fn default() -> Audio {
    Audio
  }
}

impl Audio {
  pub fn new() -> Audio {
    Audio::default()
  }
}

impl MemoryIo for Audio {
  fn read_u8(&self, addr: u16) -> Result<u8, String> {
    // println!("reading audio byte from: {:#04x}", addr);
    match addr {
      0xff10...0xff3f => Ok(0),
      _ => Ok(0),
    }
  }

  fn write_u8(&mut self, addr: u16, value: u8) -> Result<(), String> {
    // println!("write audio byte to: {:#04x} {}", addr, value);
    Ok(())
  }
}
