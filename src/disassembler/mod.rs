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
      // FIXME: just need to redesign how the disassembler works. I shouldn't
      // make exceptions for instructions here.
      // Instruction::JR_cc(_, o2) => {
      //  let val = match o2 {
      //    Operand::Imm8(v) => v as i16,
      //    Operand::Imm16(v) => v as i16,
      //    _ => panic!("disassembler.dump_all: unknown operand: {}", o2),
      //  };
      //  println!("{:04x} {:12} {:20} ; Addr: {}",
      //           pc,
      //           hex,
      //           format!("{:?}", ins),
      //           (pc as i16) + val + inc as i16)
      // }
      _ => println!("{:04x} {:12} {:12?}", pc, hex, ins),
    }
    pc += inc;
  }
}
