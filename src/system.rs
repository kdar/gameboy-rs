pub struct System {
  boot_rom: Option<Box<[u8]>>,
  cart_rom: Box<[u8]>,
}

impl System {
  pub fn new(boot_rom: Option<Box<[u8]>>, cart_rom: Box<[u8]>) -> System {
    System {
      boot_rom: boot_rom,
      cart_rom: cart_rom,
    }
  }
}
