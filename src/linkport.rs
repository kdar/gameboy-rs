use super::mem::MemoryIo;

pub const SERIAL_DATA: u16 = 0xFF01;
pub const SERIAL_CONTROL: u16 = 0xFF02;

pub struct LinkPort {
  last_byte: u8,
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
    println!("link write: {:#04x} {}", addr, value);
    match addr {
      SERIAL_DATA => {
        self.last_byte = value;
        Ok(())
      }
      SERIAL_CONTROL => {
        if value == 0x81 {
          print!("{}", self.last_byte as char);
        }
        Ok(())
      }
      _ => unreachable!(),
    }
  }
}
