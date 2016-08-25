bitflags! {
  flags SpriteFlags: u8 {
     const SPRITE_PRIORITY =  0b10000000, // Bit 7
     const SPRITE_Y_FLIP =    0b01000000, // Bit 6
     const SPRITE_X_FLIP =    0b00100000, // Bit 5
     const SPRITE_PALETTE =   0b00010000, // Bit 4
     const SPRITE_TILE_BANK = 0b00001000, // Bit 3
  }
}

#[derive(Copy, Clone, Debug)]
pub struct Sprite {
  pub y: u8,
  pub x: u8,
  pub tile: u8,
  flags: SpriteFlags,
}

impl Default for Sprite {
  fn default() -> Sprite {
    Sprite {
      y: 0,
      x: 0,
      tile: 0,
      flags: SpriteFlags::empty(),
    }
  }
}

impl Sprite {
  pub fn set_flags(&mut self, value: u8) {
    self.flags = SpriteFlags::from_bits_truncate(value)
  }

  pub fn flags(&self) -> u8 {
    self.flags.bits()
  }

  pub fn screen_x(&self) -> u8 {
    self.x.wrapping_sub(8)
  }

  pub fn screen_y(&self) -> u8 {
    self.y.wrapping_sub(16)
  }

  pub fn has_yflip(&self) -> bool {
    self.flags.contains(SPRITE_Y_FLIP)
  }

  pub fn has_xflip(&self) -> bool {
    self.flags.contains(SPRITE_X_FLIP)
  }

  pub fn has_palette1(&self) -> bool {
    self.flags.contains(SPRITE_PALETTE)
  }

  pub fn has_low_priority(&self) -> bool {
    !self.flags.contains(SPRITE_PRIORITY)
  }
}
