use super::mem::MemoryIo;

pub const AUDIO_START: u16 = 0xFF10;
pub const AUDIO_END: u16 = 0xFF3F;

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
  fn read_byte(&self, addr: u16) -> Result<u8, String> {
    // println!("reading audio byte from: {:#04x}", addr);
    match addr {
      0xFF44 => Ok(0x90), // Some(self.current_line),
      _ => Ok(0),
    }
  }

  fn write_byte(&mut self, addr: u16, value: u8) -> Result<(), String> {
    // println!("write audio byte to: {:#04x} {}", addr, value);
    Ok(())
  }
}
