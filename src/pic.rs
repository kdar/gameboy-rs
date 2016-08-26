use super::mem::MemoryIo;

bitflags! {
  flags Interrupts: u8 {
    const INT_GAMEPAD =  1 << 4,
    const INT_SERIAL =   1 << 3,
    const INT_TIMER =    1 << 2,
    const INT_LCD_STAT = 1 << 1,
    const INT_VBLANK =   1 << 0,
  }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Interrupt {
  Gamepad = 1 << 4,
  Serial = 1 << 3,
  Timer = 1 << 2,
  LcdStat = 1 << 1,
  Vblank = 1,
}

impl Interrupt {
  fn from_u8(v: u8) -> Option<Interrupt> {
    match v {
      0x10 => Some(Interrupt::Gamepad),
      0x8 => Some(Interrupt::Serial),
      0x4 => Some(Interrupt::Timer),
      0x2 => Some(Interrupt::LcdStat),
      0x1 => Some(Interrupt::Vblank),
      _ => None,
    }
  }

  pub fn addr(&self) -> u16 {
    match *self {
      Interrupt::Gamepad => 0x60,
      Interrupt::Serial => 0x58,
      Interrupt::Timer => 0x50,
      Interrupt::LcdStat => 0x48,
      Interrupt::Vblank => 0x40,
    }
  }
}

// Programmable interrupt controller
pub struct Pic {
  flags: Interrupts,
  enabled: Interrupts,
}

impl Default for Pic {
  fn default() -> Pic {
    Pic {
      flags: Interrupts::empty(),
      enabled: Interrupts::empty(),
    }
  }
}

impl MemoryIo for Pic {
  fn read_u8(&self, addr: u16) -> Result<u8, String> {
    match addr {
      // The top 3 bits are unusued and always set to 1.
      0xff0f => Ok(self.flags.bits()),// | 0b11100000),
      0xffff => Ok(self.enabled.bits()),
      _ => unreachable!(),
    }
  }

  fn write_u8(&mut self, addr: u16, value: u8) -> Result<(), String> {
    match addr {
      0xff0f => {
        self.flags = Interrupts::from_bits_truncate(value);
        // println!("flags: {:?}", self.flags);
      }
      0xffff => {
        self.enabled = Interrupts::from_bits_truncate(value);
        // println!("enabled: {:?}", self.enabled);
      }
      _ => unreachable!(),
    };
    Ok(())
  }
}

impl Pic {
  pub fn next_interrupt(&mut self) -> Option<Interrupt> {
    let bits = self.flags.bits() & self.enabled.bits();
    if bits == 0 {
      return None;
    }

    // https://stackoverflow.com/questions/18806481
    let bits = bits & ((!bits).wrapping_add(1));
    self.flags = Interrupts::from_bits_truncate(!bits & self.flags.bits());

    Interrupt::from_u8(bits)
  }

  pub fn interrupt(&mut self, int: Interrupt) {
    let int = Interrupts::from_bits_truncate(int as u8);
    self.flags.insert(int);
  }

  pub fn has_interrupt(&self) -> bool {
    self.flags.bits() & self.enabled.bits() != 0
  }
}
