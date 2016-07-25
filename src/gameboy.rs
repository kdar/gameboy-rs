use std::sync::Arc;
use std::cell::RefCell;

use super::cpu;
use super::mmu;
use super::system;

pub struct GameBoy {
  cpu: cpu::Cpu,
  system: Arc<RefCell<system::System>>,
}

impl GameBoy {
  pub fn new(cart_rom: Box<[u8]>) -> GameBoy {
    let system = Arc::new(RefCell::new(system::System::new(cart_rom)));
    GameBoy {
      cpu: cpu::Cpu::new(system.clone()),
      system: system.clone(),
    }
  }

  pub fn set_boot_rom(&mut self, rom: Box<[u8]>) {
    // self.system.get_mut().set_boot_rom(rom);
  }

  pub fn run(&mut self) {
    // loop {
    self.cpu.step();
    // }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {}
}
