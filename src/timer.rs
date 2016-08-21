use super::mem::MemoryIo;
use super::pic::{Pic, Interrupt};

#[derive(Copy, Clone)]
enum Divider {
  // Timer clock is 4096 Hz
  Div1024 = 10,
  // Timer clock is 262144 Hz
  Div16 = 4,
  // Timer clock is 65536 Hz
  Div64 = 6,
  // Timer clock is 16386
  Div256 = 8,
}

pub struct Timer {
  reg_divider: u8, // DIV
  reg_counter: u8, // TIMA
  reg_modulo: u8, // TMA
  reg_control: u8, // TAC

  counter: u32,
  divider: Divider,
  enabled: bool,
}

impl Default for Timer {
  fn default() -> Timer {
    Timer {
      reg_divider: 0,
      reg_counter: 0,
      reg_modulo: 0,
      reg_control: 0,
      counter: 0,
      divider: Divider::Div1024,
      enabled: false,
    }
  }
}

impl MemoryIo for Timer {
  fn read_u8(&self, addr: u16) -> Result<u8, String> {
    match addr {
      0xff04 => Ok(self.reg_divider),
      0xff05 => Ok(self.reg_counter),
      0xff06 => Ok(self.reg_modulo),
      0xff07 => Ok(self.reg_control),
      _ => unreachable!(),
    }
  }

  fn write_u8(&mut self, addr: u16, value: u8) -> Result<(), String> {
    match addr {
      // Always set to 0, regardless of value.
      0xff04 => self.reg_divider = 0,
      0xff05 => self.reg_counter = value,
      0xff06 => self.reg_modulo = value,
      0xff07 => {
        self.divider = match value & 0x3 {
          0 => Divider::Div1024,
          1 => Divider::Div16,
          2 => Divider::Div64,
          3 => Divider::Div256,
          _ => unreachable!(),
        };

        self.enabled = value & 0x4 != 0;
        self.reg_control = value & 0x7;
      }
      _ => unreachable!(),
    };
    Ok(())
  }
}

impl Timer {
  pub fn step(&mut self, pic: &mut Pic) {
    self.counter += 1;

    if !self.enabled {
      return;
    }

    let mask = (1 << (self.divider as usize)) - 1;

    if self.counter & mask == 0 {
      self.reg_counter = self.reg_counter.wrapping_add(1);
      if self.reg_counter == 0 {
        self.reg_counter = self.reg_modulo;
        pic.interrupt(Interrupt::Timer);
      }
    }
  }
}
