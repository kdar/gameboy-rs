use super::mem::MemoryIo;
use super::pic::{Pic, Interrupt};

#[derive(Copy, Clone)]
#[allow(enum_variant_names)]
enum ClockSpeed {
  Clock4096hz = 0,
  Clock262144z = 1,
  Clock65536z = 2,
  Clock16384z = 3,
}

impl ClockSpeed {
  fn cycles(&self) -> i32 {
    match *self {
      ClockSpeed::Clock4096hz => 256,
      ClockSpeed::Clock262144z => 4,
      ClockSpeed::Clock65536z => 16,
      ClockSpeed::Clock16384z => 64,
    }
  }
}

pub struct Timer {
  reg_divider: u8, // DIV
  reg_counter: u8, // TIMA
  reg_modulo: u8, // TMA
  reg_control: u8, // TAC

  clock_speed: ClockSpeed,
  enabled: bool,
  cycles: i32,
}

impl Default for Timer {
  fn default() -> Timer {
    Timer {
      reg_divider: 0,
      reg_counter: 0,
      reg_modulo: 0,
      reg_control: 0,
      clock_speed: ClockSpeed::Clock4096hz,
      enabled: false,
      cycles: ClockSpeed::Clock4096hz.cycles(),
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
        self.clock_speed = match value & 0x3 {
          0 => ClockSpeed::Clock4096hz,
          1 => ClockSpeed::Clock262144z,
          2 => ClockSpeed::Clock65536z,
          3 => ClockSpeed::Clock16384z,
          _ => unreachable!(),
        };
        self.cycles = self.clock_speed.cycles();
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
    if !self.enabled {
      return;
    }

    // Each step we decrease the cycle.
    self.cycles -= 1;

    // If we are 0 or less, then we need to add the cycles back
    // based on the clock speed, and then determine if we've overflowed
    // or not.
    if self.cycles <= 0 {
      self.cycles += self.clock_speed.cycles();
      // The counter register overflowed! The docs say we must
      // copy the modulo register to the counter register, and
      // trigger an interrupt.
      if self.reg_counter == 0xff {
        self.reg_counter = self.reg_modulo;
        pic.interrupt(Interrupt::Timer);
      } else {
        // No overflow. Just increase the counter register.
        self.reg_counter += 1;
      }
    }
  }
}
