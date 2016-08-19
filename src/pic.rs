bitflags! {
  flags Interrupts: u8 {
    const INT_JOYPAD =   1 << 4,
    const INT_SERIAL =   1 << 3,
    const INT_TIMER =    1 << 2,
    const INT_LCD_STAT = 1 << 1,
    const INT_VBLANK =   1 << 0,
  }
}

pub enum Interrupt {
  Joypad = 1 << 4,
  Serial = 1 << 3,
  Timer = 1 << 2,
  LcdStat = 1 << 1,
  Vblank = 1 << 0,
}

impl Interrupt {
  fn from_u8(v: u8) -> Option<Interrupt> {
    match v {
      0x10 => Some(Interrupt::Joypad),
      0x8 => Some(Interrupt::Serial),
      0x4 => Some(Interrupt::Timer),
      0x2 => Some(Interrupt::LcdStat),
      0x1 => Some(Interrupt::Vblank),
      _ => None,
    }
  }

  pub fn addr(&self) -> u16 {
    match *self {
      Interrupt::Joypad => 0x60,
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

impl Pic {
  pub fn set_flags(&mut self, v: u8) {
    self.flags = Interrupts::from_bits_truncate(v);
  }

  pub fn flags(&self) -> u8 {
    self.flags.bits()
  }

  pub fn set_enabled(&mut self, v: u8) {
    self.enabled = Interrupts::from_bits_truncate(v);
  }

  pub fn enabled(&self) -> u8 {
    self.enabled.bits()
  }

  pub fn next_interrupt(&self) -> Option<Interrupt> {
    // https://stackoverflow.com/questions/18806481
    Interrupt::from_u8(self.flags.bits() & ((!self.flags.bits()).wrapping_add(1)))
  }
}
