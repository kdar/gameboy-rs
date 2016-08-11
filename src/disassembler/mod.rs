use std::io::Write;

pub mod disassembler;
pub mod instruction;

pub use self::disassembler::Disassembler;
pub use self::instruction::Instruction;
use super::mem::MemoryIo;

fn to_hex(v: &[u8]) -> String {
  let mut f = vec![];
  for val in v {
    write!(f, "{:x}", val).unwrap();
  }

  String::from_utf8(f).unwrap()
}

struct Dump {
  rom: Box<[u8]>,
}

impl MemoryIo for Dump {
  fn read_u8(&self, addr: u16) -> Result<u8, String> {
    if (addr as usize) < self.rom.len() {
      Ok(self.rom[addr as usize])
    } else {
      Err(format!("out of rom!"))
    }
  }
}

pub fn dump_all(rom: Box<[u8]>) {
  let dis = Disassembler::new();
  let dump: &MemoryIo = &Dump { rom: rom };

  let mut pc = 0u16;

  while let Ok((ins, inc)) = dis.at(dump, pc) {
    // let hex = to_hex(&rom[(pc as usize)..(pc as usize) + inc as usize]);
    let hex = to_hex(dump.read_vec(pc, inc).unwrap().as_slice());
    match ins {
      Instruction::JR_cc_e(_, e) => {
        println!("{:04x} {:12} {:20} ; Addr: {}",
                 pc,
                 hex,
                 format!("{:?}", ins),
                 (pc as i16) + (e as i16) + inc as i16)
      }
      _ => println!("{:04x} {:12} {:12?}", pc, hex, ins),
    }
    pc += inc;
  }
}
