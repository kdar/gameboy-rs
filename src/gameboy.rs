use super::cpu;
use super::system;

pub struct GameBoy {
  cpu: cpu::CPU,
  system: system::System,
}

impl GameBoy {
  pub fn new(cart_rom: Box<[u8]>) -> GameBoy {
    GameBoy {
      cpu: cpu::CPU::new(),
      system: system::System::new(cart_rom),
    }
  }

  pub fn run(&self) {
    // loop {
    self.cpu.step();
    // }
  }

  pub fn set_boot_rom(&mut self, rom: Box<[u8]>) {
    self.system.set_boot_rom(rom);
  }

  // pub fn system(&mut self) -> &mut system::System {
  //   &mut self.system
  // }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {}
}
