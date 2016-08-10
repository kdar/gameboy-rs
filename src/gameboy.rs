use super::cpu::Cpu;
use super::bootrom::Bootrom;

pub struct GameBoy {
  pub cpu: Cpu,
  boot_rom: Bootrom,
}

impl Default for GameBoy {
  fn default() -> GameBoy {
    GameBoy {
      cpu: Cpu::default(),
      boot_rom: Bootrom::default(),
    }
  }
}

impl GameBoy {
  pub fn new(boot_rom: Option<Box<[u8]>>, cart_rom: Box<[u8]>) -> GameBoy {
    let mut cpu = Cpu::new();
    cpu.set_cart_rom(cart_rom);

    GameBoy {
      cpu: cpu,
      boot_rom: Bootrom::new(boot_rom),
    }
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
