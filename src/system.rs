use super::mmu;

pub struct System {
  boot_rom: Box<[u8]>,
  cart_rom: Box<[u8]>,
  booting: bool,
}

impl System {
  pub fn new(cart_rom: Box<[u8]>) -> System {
    System {
      boot_rom: Box::new([]),
      cart_rom: cart_rom,
      booting: false,
    }
  }

  pub fn set_boot_rom(&mut self, rom: Box<[u8]>) {
    self.booting = true;
    self.boot_rom = rom;
  }

  pub fn read_word(&self, addr: u16) -> u16 {
    let mut val: u16 = (self.read_byte(addr + 1) as u16) << 8;
    val |= self.read_byte(addr) as u16;
    val
  }

  pub fn read_byte(&self, addr: u16) -> u8 {
    let mapped = mmu::memory_map(addr);
    match mapped {
      mmu::Addr::Rom00(offset) => {
        if self.booting {
          self.boot_rom[offset as usize]
        } else {
          self.cart_rom[offset as usize]
        }
      }
      mmu::Addr::Rom01(offset) => panic!("not implemented: {:?}", mapped),
      mmu::Addr::VideoRam(offset) => panic!("not implemented: {:?}", mapped),
      mmu::Addr::ExternalRam(offset) => panic!("not implemented: {:?}", mapped),
      mmu::Addr::WorkRam00(offset) => panic!("not implemented: {:?}", mapped),
      mmu::Addr::WorkRam01(offset) => panic!("not implemented: {:?}", mapped),
      mmu::Addr::SpriteTable(offset) => panic!("not implemented: {:?}", mapped),
      mmu::Addr::IoPorts(offset) => panic!("not implemented: {:?}", mapped),
      mmu::Addr::HighRam(offset) => panic!("not implemented: {:?}", mapped),
      mmu::Addr::InterruptRegister => panic!("not implemented: {:?}", mapped),
    }
  }
}
