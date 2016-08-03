use super::cpu;

pub struct GameBoy {
  pub cpu: cpu::Cpu,
}

impl GameBoy {
  pub fn new(cart_rom: &[u8]) -> GameBoy {
    let mut cpu = cpu::Cpu::new();
    cpu.set_cart_rom(cart_rom);
    GameBoy { cpu: cpu }
  }

  pub fn set_boot_rom(&mut self, rom: Box<[u8]>) {
    self.cpu.set_boot_rom(rom);
  }

  pub fn run(&mut self) {
    loop {
      self.cpu.step();
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {}
}
