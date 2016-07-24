use super::cpu;
use super::system;

pub struct GameBoy {
  cpu: cpu::CPU,
  system: system::System,
}

impl GameBoy {
  pub fn new(boot_rom: Option<Box<[u8]>>, cart_rom: Box<[u8]>) -> GameBoy {
    GameBoy {
      cpu: cpu::CPU::new(),
      system: system::System::new(boot_rom, cart_rom),
    }
  }

  pub fn set_boot_rom(boot_rom: Box<[u8]>) {}

  pub fn run(&self) {
    // loop {
    self.step();
    // }
  }

  pub fn step(&self) {
    println!("step");
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {}
}
