use super::mem::MemoryIo;
use super::pic::{Pic, Interrupt};

bitflags! {
  flags PortSelect: u8 {
    const PORT_14 = 0b00010000,
    const PORT_15 = 0b00100000,
  }
}

#[derive(Debug, Copy, Clone)]
pub enum Button {
  Right = 0b00000001,
  Left = 0b00000010,
  Up = 0b00000100,
  Down = 0b00001000,
  // A, B, Select, and Start have the same values
  // as Right, Left, Up, and Down respectively.
  // We will use bitshifting to correct this.
  A = 0b00010000,
  B = 0b00100000,
  Select = 0b01000000,
  Start = 0b10000000,
}

impl Button {
  pub fn from_u8(v: u8) -> Button {
    match v {
      0b00000001 => Button::Right,
      0b00000010 => Button::Left,
      0b00000100 => Button::Up,
      0b00001000 => Button::Down,
      0b00010000 => Button::A,
      0b00100000 => Button::B,
      0b01000000 => Button::Select,
      0b10000000 => Button::Start,
      _ => Button::Start,
    }
  }
}

pub struct Gamepad {
  buttons1: u8,
  buttons2: u8,

  port_select: PortSelect,
  interrupt: bool,
}

impl Default for Gamepad {
  fn default() -> Gamepad {
    Gamepad {
      buttons1: 0x0f,
      buttons2: 0x0f,
      port_select: PortSelect::empty(),
      interrupt: false,
    }
  }
}

impl MemoryIo for Gamepad {
  fn read_u8(&self, addr: u16) -> Result<u8, String> {
    match addr {
      0xff00 => {
        // We negate the buttons because 1 = not pressed and 0 = pressed.
        if self.port_select.contains(PORT_14) {
          // println!("{}", 0b11000000 | PORT_14.bits | self.buttons2);
          // println!("port14: {:08b}", 0b11000000 | PORT_14.bits | self.buttons1);
          Ok(0b11000000 | PORT_14.bits | self.buttons2)
        } else {
          // println!("port15: {:08b}", 0b11000000 | PORT_15.bits | self.buttons2);
          Ok(0b11000000 | PORT_15.bits | self.buttons1)
        }
      }
      _ => unreachable!(),
    }
  }

  fn write_u8(&mut self, addr: u16, value: u8) -> Result<(), String> {
    match addr {
      0xff00 => self.port_select = PortSelect::from_bits_truncate(value),
      _ => unreachable!(),
    };
    Ok(())
  }
}

impl Gamepad {
  pub fn step(&mut self, pic: &mut Pic) {
    if self.interrupt {
      pic.interrupt(Interrupt::Gamepad);
      self.interrupt = false;
    }
  }

  pub fn set_button(&mut self, btn: Button, pressed: bool) {
    match btn {
      Button::Right | Button::Left | Button::Up | Button::Down => {
        if pressed {
          self.buttons1 &= !(btn as u8);
        } else {
          self.buttons1 |= btn as u8;
        }
      }
      Button::A | Button::B | Button::Select | Button::Start => {
        // Shift it 4 places so it lines up where it needs to be.
        if pressed {
          self.buttons2 &= (!(btn as u8)) >> 4;
        } else {
          self.buttons2 |= btn as u8 >> 4;
        }
      }
    }
    self.interrupt = true;
  }
}
