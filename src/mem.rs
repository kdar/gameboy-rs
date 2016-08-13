pub trait MemoryIo {
  #[allow(unused_variables)]
  fn read_u8(&self, addr: u16) -> Result<u8, String> {
    Ok(0)
  }

  #[allow(unused_variables)]
  fn write_u8(&mut self, addr: u16, value: u8) -> Result<(), String> {
    Ok(())
  }

  fn read_vec(&self, addr: u16, len: u16) -> Result<Vec<u8>, String> {
    let mut v = vec![];
    for i in addr..addr + len {
      match self.read_u8(i) {
        Ok(x) => v.push(x),
        Err(e) => return Err(e),
      }
    }

    if v.is_empty() {
      Err("length of vec read is 0".to_owned())
    } else {
      Ok(v)
    }
  }

  // TODO: Maybe allow MemoryIO objects implement this directly,
  // so it doesn't need to call read_u8 twice and instead just
  // read the word directly. Also would need a read_u32 or something
  // similar for performance reasons.
  fn read_u16(&self, addr: u16) -> Result<u16, String> {
    let mut val: u16 = match self.read_u8(addr + 1) {
      Ok(x) => (x as u16) << 8,
      Err(e) => return Err(e),
    };
    val |= match self.read_u8(addr) {
      Ok(x) => x as u16,
      Err(e) => return Err(e),
    };
    Ok(val)
  }

  fn write_u16(&mut self, addr: u16, value: u16) -> Result<(), String> {
    let addr = addr as usize;
    try!(self.write_u8((addr + 1) as u16, (value >> 8) as u8 & 0b11111111));
    try!(self.write_u8(addr as u16, value as u8 & 0b11111111));
    Ok(())
  }
}

// pub trait Memory: MemoryIo {
//  fn map(&mut self, start: u16, end: u16, mapper: Rc<RefCell<MemoryIo>>);
//  fn set_boot_rom(&mut self, rom: Box<[u8]>);
// }

// #[cfg(test)]
// mod module {
//  use std::rc::Rc;
//  use std::cell::RefCell;
//
//  use super::*;
//  use std::fmt;
//  use md5;
//
//  pub struct Mem {
//    // boot_rom: Box<[u8]>,
//    // cart_rom: Box<[u8]>,
//    booting: bool,
//
//    ram: [u8; 0xFFFF + 1],
//  }
//
//  impl PartialEq for Mem {
//    fn eq(&self, x: &Mem) -> bool {
//      self.booting == x.booting && self.ram[..] == x.ram[..]
//    }
//  }
//
//  impl fmt::Debug for Mem {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//      try!(write!(f, "\nBooting: {}", self.booting));
//      try!(write!(f, "\nRam checksum: {:?}", md5::compute(&self.ram[..])));
//      write!(f, "\n")
//    }
//  }
//
//  impl Mem {
//    pub fn new() -> Mem {
//      Mem {
//        // boot_rom: Box::new([]),
//        // cart_rom: Box::new([]),
//        booting: false,
//        ram: [0; 0xFFFF + 1],
//      }
//    }
//  }
//
//  impl MemoryIo for Mem {
//    fn read_u8(&self, addr: u16) -> Result<u8, String> {
//      self.ram
//        .get(addr as usize)
//        .ok_or(format!("could not get byte at test ram offset {}", addr))
//        .and_then(|&x| Ok(x))
//    }
//
//    fn write_u8(&mut self, addr: u16, value: u8) -> Result<(), String> {
//      self.ram[addr as usize] = value;
//      Ok(())
//    }
//  }
//
//  impl Memory for Mem {
//    fn map(&mut self, _: u16, _: u16, _: Rc<RefCell<MemoryIo>>) {}
//
//    fn set_boot_rom(&mut self, _: Box<[u8]>) {
//      panic!("set_boot_rom should not be used for testing. use write_u8 to write the rom to \
//              memory");
//    }
//  }
// }
