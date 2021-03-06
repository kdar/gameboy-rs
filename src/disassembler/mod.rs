use std::io::Write;

pub mod instruction;

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
      Err("out of rom!".to_owned())
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

pub struct Disassembler;

impl Default for Disassembler {
  fn default() -> Disassembler {
    Disassembler
  }
}

impl Disassembler {
  pub fn new() -> Disassembler {
    Disassembler::default()
  }

  fn imm8(&self, m: &MemoryIo, pc: &mut u16, addr: u16) -> Result<u8, String> {
    let n = try!(m.read_u8(addr + *pc));
    *pc += 1;
    Ok(n)
  }

  fn imm16(&self, m: &MemoryIo, pc: &mut u16, addr: u16) -> Result<u16, String> {
    let nn = try!(m.read_u16(addr + *pc));
    *pc += 2;
    Ok(nn)
  }

  pub fn at(&self, m: &MemoryIo, addr: u16) -> Result<(Instruction, u16), String> {
    let mut pc = 0u16;

    use self::instruction::Instruction as I;
    use super::operand::Operand as O;

    let op = try!(m.read_u8(addr + pc));
    pc += 1;

    if op == 0xCB {
      let op = try!(m.read_u8(addr + pc));
      pc += 1;
      let ins = match op {
        0x47 => I::BIT(O::Imm8(0), O::RegA),
        0x4f => I::BIT(O::Imm8(1), O::RegA),
        0x57 => I::BIT(O::Imm8(2), O::RegA),
        0x5f => I::BIT(O::Imm8(3), O::RegA),
        0x67 => I::BIT(O::Imm8(4), O::RegA),
        0x6f => I::BIT(O::Imm8(5), O::RegA),
        0x77 => I::BIT(O::Imm8(6), O::RegA),
        0x7f => I::BIT(O::Imm8(7), O::RegA),
        0x40 => I::BIT(O::Imm8(0), O::RegB),
        0x48 => I::BIT(O::Imm8(1), O::RegB),
        0x50 => I::BIT(O::Imm8(2), O::RegB),
        0x58 => I::BIT(O::Imm8(3), O::RegB),
        0x60 => I::BIT(O::Imm8(4), O::RegB),
        0x68 => I::BIT(O::Imm8(5), O::RegB),
        0x70 => I::BIT(O::Imm8(6), O::RegB),
        0x78 => I::BIT(O::Imm8(7), O::RegB),
        0x41 => I::BIT(O::Imm8(0), O::RegC),
        0x49 => I::BIT(O::Imm8(1), O::RegC),
        0x51 => I::BIT(O::Imm8(2), O::RegC),
        0x59 => I::BIT(O::Imm8(3), O::RegC),
        0x61 => I::BIT(O::Imm8(4), O::RegC),
        0x69 => I::BIT(O::Imm8(5), O::RegC),
        0x71 => I::BIT(O::Imm8(6), O::RegC),
        0x79 => I::BIT(O::Imm8(7), O::RegC),
        0x42 => I::BIT(O::Imm8(0), O::RegD),
        0x4a => I::BIT(O::Imm8(1), O::RegD),
        0x52 => I::BIT(O::Imm8(2), O::RegD),
        0x5a => I::BIT(O::Imm8(3), O::RegD),
        0x62 => I::BIT(O::Imm8(4), O::RegD),
        0x6a => I::BIT(O::Imm8(5), O::RegD),
        0x72 => I::BIT(O::Imm8(6), O::RegD),
        0x7a => I::BIT(O::Imm8(7), O::RegD),
        0x43 => I::BIT(O::Imm8(0), O::RegE),
        0x4b => I::BIT(O::Imm8(1), O::RegE),
        0x53 => I::BIT(O::Imm8(2), O::RegE),
        0x5b => I::BIT(O::Imm8(3), O::RegE),
        0x63 => I::BIT(O::Imm8(4), O::RegE),
        0x6b => I::BIT(O::Imm8(5), O::RegE),
        0x73 => I::BIT(O::Imm8(6), O::RegE),
        0x7b => I::BIT(O::Imm8(7), O::RegE),
        0x44 => I::BIT(O::Imm8(0), O::RegH),
        0x4c => I::BIT(O::Imm8(1), O::RegH),
        0x54 => I::BIT(O::Imm8(2), O::RegH),
        0x5c => I::BIT(O::Imm8(3), O::RegH),
        0x64 => I::BIT(O::Imm8(4), O::RegH),
        0x6c => I::BIT(O::Imm8(5), O::RegH),
        0x74 => I::BIT(O::Imm8(6), O::RegH),
        0x7c => I::BIT(O::Imm8(7), O::RegH),
        0x45 => I::BIT(O::Imm8(0), O::RegL),
        0x4d => I::BIT(O::Imm8(1), O::RegL),
        0x55 => I::BIT(O::Imm8(2), O::RegL),
        0x5d => I::BIT(O::Imm8(3), O::RegL),
        0x65 => I::BIT(O::Imm8(4), O::RegL),
        0x6d => I::BIT(O::Imm8(5), O::RegL),
        0x75 => I::BIT(O::Imm8(6), O::RegL),
        0x7d => I::BIT(O::Imm8(7), O::RegL),
        0x46 => I::BIT(O::Imm8(0), O::AddrHL),
        0x4e => I::BIT(O::Imm8(1), O::AddrHL),
        0x56 => I::BIT(O::Imm8(2), O::AddrHL),
        0x5e => I::BIT(O::Imm8(3), O::AddrHL),
        0x66 => I::BIT(O::Imm8(4), O::AddrHL),
        0x6e => I::BIT(O::Imm8(5), O::AddrHL),
        0x76 => I::BIT(O::Imm8(6), O::AddrHL),
        0x7e => I::BIT(O::Imm8(7), O::AddrHL),

        0x87 => I::RES(O::Imm8(0), O::RegA),
        0x8f => I::RES(O::Imm8(1), O::RegA),
        0x97 => I::RES(O::Imm8(2), O::RegA),
        0x9f => I::RES(O::Imm8(3), O::RegA),
        0xa7 => I::RES(O::Imm8(4), O::RegA),
        0xaf => I::RES(O::Imm8(5), O::RegA),
        0xb7 => I::RES(O::Imm8(6), O::RegA),
        0xbf => I::RES(O::Imm8(7), O::RegA),
        0x80 => I::RES(O::Imm8(0), O::RegB),
        0x88 => I::RES(O::Imm8(1), O::RegB),
        0x90 => I::RES(O::Imm8(2), O::RegB),
        0x98 => I::RES(O::Imm8(3), O::RegB),
        0xa0 => I::RES(O::Imm8(4), O::RegB),
        0xa8 => I::RES(O::Imm8(5), O::RegB),
        0xb0 => I::RES(O::Imm8(6), O::RegB),
        0xb8 => I::RES(O::Imm8(7), O::RegB),
        0x81 => I::RES(O::Imm8(0), O::RegC),
        0x89 => I::RES(O::Imm8(1), O::RegC),
        0x91 => I::RES(O::Imm8(2), O::RegC),
        0x99 => I::RES(O::Imm8(3), O::RegC),
        0xa1 => I::RES(O::Imm8(4), O::RegC),
        0xa9 => I::RES(O::Imm8(5), O::RegC),
        0xb1 => I::RES(O::Imm8(6), O::RegC),
        0xb9 => I::RES(O::Imm8(7), O::RegC),
        0x82 => I::RES(O::Imm8(0), O::RegD),
        0x8a => I::RES(O::Imm8(1), O::RegD),
        0x92 => I::RES(O::Imm8(2), O::RegD),
        0x9a => I::RES(O::Imm8(3), O::RegD),
        0xa2 => I::RES(O::Imm8(4), O::RegD),
        0xaa => I::RES(O::Imm8(5), O::RegD),
        0xb2 => I::RES(O::Imm8(6), O::RegD),
        0xba => I::RES(O::Imm8(7), O::RegD),
        0x83 => I::RES(O::Imm8(0), O::RegE),
        0x8b => I::RES(O::Imm8(1), O::RegE),
        0x93 => I::RES(O::Imm8(2), O::RegE),
        0x9b => I::RES(O::Imm8(3), O::RegE),
        0xa3 => I::RES(O::Imm8(4), O::RegE),
        0xab => I::RES(O::Imm8(5), O::RegE),
        0xb3 => I::RES(O::Imm8(6), O::RegE),
        0xbb => I::RES(O::Imm8(7), O::RegE),
        0x84 => I::RES(O::Imm8(0), O::RegH),
        0x8c => I::RES(O::Imm8(1), O::RegH),
        0x94 => I::RES(O::Imm8(2), O::RegH),
        0x9c => I::RES(O::Imm8(3), O::RegH),
        0xa4 => I::RES(O::Imm8(4), O::RegH),
        0xac => I::RES(O::Imm8(5), O::RegH),
        0xb4 => I::RES(O::Imm8(6), O::RegH),
        0xbc => I::RES(O::Imm8(7), O::RegH),
        0x85 => I::RES(O::Imm8(0), O::RegL),
        0x8d => I::RES(O::Imm8(1), O::RegL),
        0x95 => I::RES(O::Imm8(2), O::RegL),
        0x9d => I::RES(O::Imm8(3), O::RegL),
        0xa5 => I::RES(O::Imm8(4), O::RegL),
        0xad => I::RES(O::Imm8(5), O::RegL),
        0xb5 => I::RES(O::Imm8(6), O::RegL),
        0xbd => I::RES(O::Imm8(7), O::RegL),
        0x86 => I::RES(O::Imm8(0), O::AddrHL),
        0x8e => I::RES(O::Imm8(1), O::AddrHL),
        0x96 => I::RES(O::Imm8(2), O::AddrHL),
        0x9e => I::RES(O::Imm8(3), O::AddrHL),
        0xa6 => I::RES(O::Imm8(4), O::AddrHL),
        0xae => I::RES(O::Imm8(5), O::AddrHL),
        0xb6 => I::RES(O::Imm8(6), O::AddrHL),
        0xbe => I::RES(O::Imm8(7), O::AddrHL),

        0x17 => I::RL(O::RegA),
        0x10 => I::RL(O::RegB),
        0x11 => I::RL(O::RegC),
        0x12 => I::RL(O::RegD),
        0x13 => I::RL(O::RegE),
        0x14 => I::RL(O::RegH),
        0x15 => I::RL(O::RegL),
        0x16 => I::RL(O::AddrHL),

        0x07 => I::RLC(O::RegA),
        0x00 => I::RLC(O::RegB),
        0x01 => I::RLC(O::RegC),
        0x02 => I::RLC(O::RegD),
        0x03 => I::RLC(O::RegE),
        0x04 => I::RLC(O::RegH),
        0x05 => I::RLC(O::RegL),
        0x06 => I::RLC(O::AddrHL),

        0x1f => I::RR(O::RegA),
        0x18 => I::RR(O::RegB),
        0x19 => I::RR(O::RegC),
        0x1a => I::RR(O::RegD),
        0x1b => I::RR(O::RegE),
        0x1c => I::RR(O::RegH),
        0x1d => I::RR(O::RegL),
        0x1e => I::RR(O::AddrHL),

        0x0f => I::RRC(O::RegA),
        0x08 => I::RRC(O::RegB),
        0x09 => I::RRC(O::RegC),
        0x0a => I::RRC(O::RegD),
        0x0b => I::RRC(O::RegE),
        0x0c => I::RRC(O::RegH),
        0x0d => I::RRC(O::RegL),
        0x0e => I::RRC(O::AddrHL),

        0xc7 => I::SET(O::Imm8(0), O::RegA),
        0xcf => I::SET(O::Imm8(1), O::RegA),
        0xd7 => I::SET(O::Imm8(2), O::RegA),
        0xdf => I::SET(O::Imm8(3), O::RegA),
        0xe7 => I::SET(O::Imm8(4), O::RegA),
        0xef => I::SET(O::Imm8(5), O::RegA),
        0xf7 => I::SET(O::Imm8(6), O::RegA),
        0xff => I::SET(O::Imm8(7), O::RegA),
        0xc0 => I::SET(O::Imm8(0), O::RegB),
        0xc8 => I::SET(O::Imm8(1), O::RegB),
        0xd0 => I::SET(O::Imm8(2), O::RegB),
        0xd8 => I::SET(O::Imm8(3), O::RegB),
        0xe0 => I::SET(O::Imm8(4), O::RegB),
        0xe8 => I::SET(O::Imm8(5), O::RegB),
        0xf0 => I::SET(O::Imm8(6), O::RegB),
        0xf8 => I::SET(O::Imm8(7), O::RegB),
        0xc1 => I::SET(O::Imm8(0), O::RegC),
        0xc9 => I::SET(O::Imm8(1), O::RegC),
        0xd1 => I::SET(O::Imm8(2), O::RegC),
        0xd9 => I::SET(O::Imm8(3), O::RegC),
        0xe1 => I::SET(O::Imm8(4), O::RegC),
        0xe9 => I::SET(O::Imm8(5), O::RegC),
        0xf1 => I::SET(O::Imm8(6), O::RegC),
        0xf9 => I::SET(O::Imm8(7), O::RegC),
        0xc2 => I::SET(O::Imm8(0), O::RegD),
        0xca => I::SET(O::Imm8(1), O::RegD),
        0xd2 => I::SET(O::Imm8(2), O::RegD),
        0xda => I::SET(O::Imm8(3), O::RegD),
        0xe2 => I::SET(O::Imm8(4), O::RegD),
        0xea => I::SET(O::Imm8(5), O::RegD),
        0xf2 => I::SET(O::Imm8(6), O::RegD),
        0xfa => I::SET(O::Imm8(7), O::RegD),
        0xc3 => I::SET(O::Imm8(0), O::RegE),
        0xcb => I::SET(O::Imm8(1), O::RegE),
        0xd3 => I::SET(O::Imm8(2), O::RegE),
        0xdb => I::SET(O::Imm8(3), O::RegE),
        0xe3 => I::SET(O::Imm8(4), O::RegE),
        0xeb => I::SET(O::Imm8(5), O::RegE),
        0xf3 => I::SET(O::Imm8(6), O::RegE),
        0xfb => I::SET(O::Imm8(7), O::RegE),
        0xc4 => I::SET(O::Imm8(0), O::RegH),
        0xcc => I::SET(O::Imm8(1), O::RegH),
        0xd4 => I::SET(O::Imm8(2), O::RegH),
        0xdc => I::SET(O::Imm8(3), O::RegH),
        0xe4 => I::SET(O::Imm8(4), O::RegH),
        0xec => I::SET(O::Imm8(5), O::RegH),
        0xf4 => I::SET(O::Imm8(6), O::RegH),
        0xfc => I::SET(O::Imm8(7), O::RegH),
        0xc5 => I::SET(O::Imm8(0), O::RegL),
        0xcd => I::SET(O::Imm8(1), O::RegL),
        0xd5 => I::SET(O::Imm8(2), O::RegL),
        0xdd => I::SET(O::Imm8(3), O::RegL),
        0xe5 => I::SET(O::Imm8(4), O::RegL),
        0xed => I::SET(O::Imm8(5), O::RegL),
        0xf5 => I::SET(O::Imm8(6), O::RegL),
        0xfd => I::SET(O::Imm8(7), O::RegL),
        0xc6 => I::SET(O::Imm8(0), O::AddrHL),
        0xce => I::SET(O::Imm8(1), O::AddrHL),
        0xd6 => I::SET(O::Imm8(2), O::AddrHL),
        0xde => I::SET(O::Imm8(3), O::AddrHL),
        0xe6 => I::SET(O::Imm8(4), O::AddrHL),
        0xee => I::SET(O::Imm8(5), O::AddrHL),
        0xf6 => I::SET(O::Imm8(6), O::AddrHL),
        0xfe => I::SET(O::Imm8(7), O::AddrHL),

        0x27 => I::SLA(O::RegA),
        0x20 => I::SLA(O::RegB),
        0x21 => I::SLA(O::RegC),
        0x22 => I::SLA(O::RegD),
        0x23 => I::SLA(O::RegE),
        0x24 => I::SLA(O::RegH),
        0x25 => I::SLA(O::RegL),
        0x26 => I::SLA(O::AddrHL),

        0x2f => I::SRA(O::RegA),
        0x28 => I::SRA(O::RegB),
        0x29 => I::SRA(O::RegC),
        0x2a => I::SRA(O::RegD),
        0x2b => I::SRA(O::RegE),
        0x2c => I::SRA(O::RegH),
        0x2d => I::SRA(O::RegL),
        0x2e => I::SRA(O::AddrHL),

        0x3f => I::SRL(O::RegA),
        0x38 => I::SRL(O::RegB),
        0x39 => I::SRL(O::RegC),
        0x3a => I::SRL(O::RegD),
        0x3b => I::SRL(O::RegE),
        0x3c => I::SRL(O::RegH),
        0x3d => I::SRL(O::RegL),
        0x3e => I::SRL(O::AddrHL),

        0x37 => I::SWAP(O::RegA),
        0x30 => I::SWAP(O::RegB),
        0x31 => I::SWAP(O::RegC),
        0x32 => I::SWAP(O::RegD),
        0x33 => I::SWAP(O::RegE),
        0x34 => I::SWAP(O::RegH),
        0x35 => I::SWAP(O::RegL),
        0x36 => I::SWAP(O::AddrHL),

        _ => {
          panic!("instruction_at: 0xCB instruction not implemented: 0x{:02x}",
                 op)
        }
      };

      Ok((ins, pc))
    } else {
      let ins = match op {
        0x8f => I::ADC(O::RegA, O::RegA),
        0x88 => I::ADC(O::RegA, O::RegB),
        0x89 => I::ADC(O::RegA, O::RegC),
        0x8a => I::ADC(O::RegA, O::RegD),
        0x8b => I::ADC(O::RegA, O::RegE),
        0x8c => I::ADC(O::RegA, O::RegH),
        0x8d => I::ADC(O::RegA, O::RegL),
        0x8e => I::ADC(O::RegA, O::AddrHL),
        0xce => I::ADC(O::RegA, O::Imm8(try!(self.imm8(m, &mut pc, addr)))),

        0x87 => I::ADD8(O::RegA, O::RegA),
        0x80 => I::ADD8(O::RegA, O::RegB),
        0x81 => I::ADD8(O::RegA, O::RegC),
        0x82 => I::ADD8(O::RegA, O::RegD),
        0x83 => I::ADD8(O::RegA, O::RegE),
        0x84 => I::ADD8(O::RegA, O::RegH),
        0x85 => I::ADD8(O::RegA, O::RegL),
        0x86 => I::ADD8(O::RegA, O::AddrHL),
        0xc6 => I::ADD8(O::RegA, O::Imm8(try!(self.imm8(m, &mut pc, addr)))),
        0x09 => I::ADD_HL(O::RegBC),
        0x19 => I::ADD_HL(O::RegDE),
        0x29 => I::ADD_HL(O::RegHL),
        0x39 => I::ADD_HL(O::RegSP),
        0xe8 => I::ADD_SP(O::Imm8(try!(self.imm8(m, &mut pc, addr)))),

        0xa7 => I::AND(O::RegA),
        0xa0 => I::AND(O::RegB),
        0xa1 => I::AND(O::RegC),
        0xa2 => I::AND(O::RegD),
        0xa3 => I::AND(O::RegE),
        0xa4 => I::AND(O::RegH),
        0xa5 => I::AND(O::RegL),
        0xa6 => I::AND(O::AddrHL),
        0xe6 => I::AND(O::Imm8(try!(self.imm8(m, &mut pc, addr)))),

        0xc4 => I::CALL_cc(O::FlagNZ, O::Imm16(try!(self.imm16(m, &mut pc, addr)))),
        0xcc => I::CALL_cc(O::FlagZ, O::Imm16(try!(self.imm16(m, &mut pc, addr)))),
        0xd4 => I::CALL_cc(O::FlagNC, O::Imm16(try!(self.imm16(m, &mut pc, addr)))),
        0xdc => I::CALL_cc(O::FlagC, O::Imm16(try!(self.imm16(m, &mut pc, addr)))),
        0xcd => I::CALL(O::Imm16(try!(self.imm16(m, &mut pc, addr)))),

        0x3f => I::CCF,

        0xbf => I::CP(O::RegA),
        0xb8 => I::CP(O::RegB),
        0xb9 => I::CP(O::RegC),
        0xba => I::CP(O::RegD),
        0xbb => I::CP(O::RegE),
        0xbc => I::CP(O::RegH),
        0xbd => I::CP(O::RegL),
        0xbe => I::CP(O::AddrHL),
        0xfe => I::CP(O::Imm8(((try!(self.imm8(m, &mut pc, addr)))))),

        0x2f => I::CPL,

        0x27 => I::DAA,

        0x3d => I::DEC8(O::RegA),
        0x05 => I::DEC8(O::RegB),
        0x0d => I::DEC8(O::RegC),
        0x15 => I::DEC8(O::RegD),
        0x1d => I::DEC8(O::RegE),
        0x25 => I::DEC8(O::RegH),
        0x2d => I::DEC8(O::RegL),
        0x35 => I::DEC8(O::AddrHL),
        0x0b => I::DEC16(O::RegBC),
        0x1b => I::DEC16(O::RegDE),
        0x2b => I::DEC16(O::RegHL),
        0x3b => I::DEC16(O::RegSP),

        0xf3 => I::DI,

        0xfb => I::EI,

        0x76 => I::HALT,

        0x3c => I::INC8(O::RegA),
        0x04 => I::INC8(O::RegB),
        0x0c => I::INC8(O::RegC),
        0x14 => I::INC8(O::RegD),
        0x1c => I::INC8(O::RegE),
        0x24 => I::INC8(O::RegH),
        0x2c => I::INC8(O::RegL),
        0x34 => I::INC8(O::AddrHL),
        0x03 => I::INC16(O::RegBC),
        0x13 => I::INC16(O::RegDE),
        0x23 => I::INC16(O::RegHL),
        0x33 => I::INC16(O::RegSP),

        0xc2 => I::JP_cc(O::FlagNZ, O::Imm16(try!(self.imm16(m, &mut pc, addr)))),
        0xca => I::JP_cc(O::FlagZ, O::Imm16(try!(self.imm16(m, &mut pc, addr)))),
        0xd2 => I::JP_cc(O::FlagNC, O::Imm16(try!(self.imm16(m, &mut pc, addr)))),
        0xda => I::JP_cc(O::FlagC, O::Imm16(try!(self.imm16(m, &mut pc, addr)))),
        0xe9 => I::JP(O::RegHL),
        0xc3 => I::JP(O::Imm16(try!(self.imm16(m, &mut pc, addr)))),

        0x20 => I::JR_cc(O::FlagNZ, O::Imm8(try!(self.imm8(m, &mut pc, addr)))),
        0x28 => I::JR_cc(O::FlagZ, O::Imm8(try!(self.imm8(m, &mut pc, addr)))),
        0x30 => I::JR_cc(O::FlagNC, O::Imm8(try!(self.imm8(m, &mut pc, addr)))),
        0x38 => I::JR_cc(O::FlagC, O::Imm8(try!(self.imm8(m, &mut pc, addr)))),
        0x18 => I::JR(O::Imm8(try!(self.imm8(m, &mut pc, addr)))),

        0x7f => I::LD8(O::RegA, O::RegA),
        0x78 => I::LD8(O::RegA, O::RegB),
        0x79 => I::LD8(O::RegA, O::RegC),
        0x7a => I::LD8(O::RegA, O::RegD),
        0x7b => I::LD8(O::RegA, O::RegE),
        0x7c => I::LD8(O::RegA, O::RegH),
        0x7d => I::LD8(O::RegA, O::RegL),
        0x7e => I::LD8(O::RegA, O::AddrHL),
        0x47 => I::LD8(O::RegB, O::RegA),
        0x40 => I::LD8(O::RegB, O::RegB),
        0x41 => I::LD8(O::RegB, O::RegC),
        0x42 => I::LD8(O::RegB, O::RegD),
        0x43 => I::LD8(O::RegB, O::RegE),
        0x44 => I::LD8(O::RegB, O::RegH),
        0x45 => I::LD8(O::RegB, O::RegL),
        0x46 => I::LD8(O::RegB, O::AddrHL),
        0x4f => I::LD8(O::RegC, O::RegA),
        0x48 => I::LD8(O::RegC, O::RegB),
        0x49 => I::LD8(O::RegC, O::RegC),
        0x4a => I::LD8(O::RegC, O::RegD),
        0x4b => I::LD8(O::RegC, O::RegE),
        0x4c => I::LD8(O::RegC, O::RegH),
        0x4d => I::LD8(O::RegC, O::RegL),
        0x4e => I::LD8(O::RegC, O::AddrHL),
        0x57 => I::LD8(O::RegD, O::RegA),
        0x50 => I::LD8(O::RegD, O::RegB),
        0x51 => I::LD8(O::RegD, O::RegC),
        0x52 => I::LD8(O::RegD, O::RegD),
        0x53 => I::LD8(O::RegD, O::RegE),
        0x54 => I::LD8(O::RegD, O::RegH),
        0x55 => I::LD8(O::RegD, O::RegL),
        0x56 => I::LD8(O::RegD, O::AddrHL),
        0x5f => I::LD8(O::RegE, O::RegA),
        0x59 => I::LD8(O::RegE, O::RegC),
        0x58 => I::LD8(O::RegE, O::RegB),
        0x5a => I::LD8(O::RegE, O::RegD),
        0x5b => I::LD8(O::RegE, O::RegE),
        0x5c => I::LD8(O::RegE, O::RegH),
        0x5d => I::LD8(O::RegE, O::RegL),
        0x5e => I::LD8(O::RegE, O::AddrHL),
        0x67 => I::LD8(O::RegH, O::RegA),
        0x60 => I::LD8(O::RegH, O::RegB),
        0x61 => I::LD8(O::RegH, O::RegC),
        0x62 => I::LD8(O::RegH, O::RegD),
        0x63 => I::LD8(O::RegH, O::RegE),
        0x64 => I::LD8(O::RegH, O::RegH),
        0x65 => I::LD8(O::RegH, O::RegL),
        0x66 => I::LD8(O::RegH, O::AddrHL),
        0x6f => I::LD8(O::RegL, O::RegA),
        0x68 => I::LD8(O::RegL, O::RegB),
        0x69 => I::LD8(O::RegL, O::RegC),
        0x6a => I::LD8(O::RegL, O::RegD),
        0x6b => I::LD8(O::RegL, O::RegE),
        0x6c => I::LD8(O::RegL, O::RegH),
        0x6d => I::LD8(O::RegL, O::RegL),
        0x6e => I::LD8(O::RegL, O::AddrHL),
        0x3e => I::LD8(O::RegA, O::Imm8(try!(self.imm8(m, &mut pc, addr)))),
        0x06 => I::LD8(O::RegB, O::Imm8(try!(self.imm8(m, &mut pc, addr)))),
        0x0e => I::LD8(O::RegC, O::Imm8(try!(self.imm8(m, &mut pc, addr)))),
        0x16 => I::LD8(O::RegD, O::Imm8(try!(self.imm8(m, &mut pc, addr)))),
        0x1e => I::LD8(O::RegE, O::Imm8(try!(self.imm8(m, &mut pc, addr)))),
        0x26 => I::LD8(O::RegH, O::Imm8(try!(self.imm8(m, &mut pc, addr)))),
        0x2e => I::LD8(O::RegL, O::Imm8(try!(self.imm8(m, &mut pc, addr)))),
        0x36 => I::LD8(O::AddrHL, O::Imm8(try!(self.imm8(m, &mut pc, addr)))),
        0x77 => I::LD8(O::AddrHL, O::RegA),
        0x70 => I::LD8(O::AddrHL, O::RegB),
        0x71 => I::LD8(O::AddrHL, O::RegC),
        0x72 => I::LD8(O::AddrHL, O::RegD),
        0x73 => I::LD8(O::AddrHL, O::RegE),
        0x74 => I::LD8(O::AddrHL, O::RegH),
        0x75 => I::LD8(O::AddrHL, O::RegL),
        0x0a => I::LD8(O::RegA, O::AddrBC),
        0x02 => I::LD8(O::AddrBC, O::RegA),
        0x1a => I::LD8(O::RegA, O::AddrDE),
        0x12 => I::LD8(O::AddrDE, O::RegA),
        0xfa => I::LD8(O::RegA, O::AddrImm16(try!(self.imm16(m, &mut pc, addr)))),
        0xea => I::LD8(O::AddrImm16(try!(self.imm16(m, &mut pc, addr))), O::RegA),
        0xe0 => {
          I::LD8(O::AddrImm16(0xff00 + try!(self.imm8(m, &mut pc, addr)) as u16),
                 O::RegA)
        }
        0xf0 => {
          I::LD8(O::RegA,
                 O::AddrImm16(0xff00 + try!(self.imm8(m, &mut pc, addr)) as u16))
        }
        0xe2 => I::LD8(O::AddrIoPortC, O::RegA),
        0xf2 => I::LD8(O::RegA, O::AddrIoPortC),
        0x01 => I::LD16(O::RegBC, O::Imm16(try!(self.imm16(m, &mut pc, addr)))),
        0x11 => I::LD16(O::RegDE, O::Imm16(try!(self.imm16(m, &mut pc, addr)))),
        0x21 => I::LD16(O::RegHL, O::Imm16(try!(self.imm16(m, &mut pc, addr)))),
        0x31 => I::LD16(O::RegSP, O::Imm16(try!(self.imm16(m, &mut pc, addr)))),
        0xf9 => I::LD16(O::RegSP, O::RegHL),
        0xf8 => I::LD_HL(O::RegSP, O::Imm8(try!(self.imm8(m, &mut pc, addr)))),
        0x08 => I::LD16(O::AddrImm16(try!(self.imm16(m, &mut pc, addr))), O::RegSP),

        0x22 => I::LDI(O::AddrHL, O::RegA),
        0x2a => I::LDI(O::RegA, O::AddrHL),
        0x32 => I::LDD(O::AddrHL, O::RegA),
        0x3a => I::LDD(O::RegA, O::AddrHL),

        0xb7 => I::OR(O::RegA, O::RegA),
        0xb0 => I::OR(O::RegA, O::RegB),
        0xb1 => I::OR(O::RegA, O::RegC),
        0xb2 => I::OR(O::RegA, O::RegD),
        0xb3 => I::OR(O::RegA, O::RegE),
        0xb4 => I::OR(O::RegA, O::RegH),
        0xb5 => I::OR(O::RegA, O::RegL),
        0xb6 => I::OR(O::RegA, O::AddrHL),
        0xf6 => I::OR(O::RegA, O::Imm8(try!(self.imm8(m, &mut pc, addr)))),

        0xc1 => I::POP16(O::RegBC),
        0xd1 => I::POP16(O::RegDE),
        0xe1 => I::POP16(O::RegHL),
        0xf1 => I::POP16(O::RegAF),
        0xc5 => I::PUSH16(O::RegBC),
        0xd5 => I::PUSH16(O::RegDE),
        0xe5 => I::PUSH16(O::RegHL),
        0xf5 => I::PUSH16(O::RegAF),

        0xc9 => I::RET,
        0xc0 => I::RET_cc(O::FlagNZ),
        0xc8 => I::RET_cc(O::FlagZ),
        0xd0 => I::RET_cc(O::FlagNC),
        0xd8 => I::RET_cc(O::FlagC),
        0xd9 => I::RETI,

        0x17 => I::RLA,
        0x07 => I::RLCA,
        0x1f => I::RRA,
        0x0f => I::RRCA,

        0xc7 => I::RST(O::Imm8(0x00)),
        0xcf => I::RST(O::Imm8(0x08)),
        0xd7 => I::RST(O::Imm8(0x10)),
        0xdf => I::RST(O::Imm8(0x18)),
        0xe7 => I::RST(O::Imm8(0x20)),
        0xef => I::RST(O::Imm8(0x28)),
        0xf7 => I::RST(O::Imm8(0x30)),
        0xff => I::RST(O::Imm8(0x38)),

        0x9f => I::SBC(O::RegA),
        0x98 => I::SBC(O::RegB),
        0x99 => I::SBC(O::RegC),
        0x9a => I::SBC(O::RegD),
        0x9b => I::SBC(O::RegE),
        0x9c => I::SBC(O::RegH),
        0x9d => I::SBC(O::RegL),
        0x9e => I::SBC(O::AddrHL),
        0xde => I::SBC(O::Imm8(try!(self.imm8(m, &mut pc, addr)))),

        0x37 => I::SCF,

        0x10 => I::STOP,

        0x97 => I::SUB(O::RegA, O::RegA),
        0x90 => I::SUB(O::RegA, O::RegB),
        0x91 => I::SUB(O::RegA, O::RegC),
        0x92 => I::SUB(O::RegA, O::RegD),
        0x93 => I::SUB(O::RegA, O::RegE),
        0x94 => I::SUB(O::RegA, O::RegH),
        0x95 => I::SUB(O::RegA, O::RegL),
        0x96 => I::SUB(O::RegA, O::AddrHL),
        0xd6 => I::SUB(O::RegA, O::Imm8(try!(self.imm8(m, &mut pc, addr)))),

        0x00 => I::NOP,

        0xaf => I::XOR(O::RegA, O::RegA),
        0xa8 => I::XOR(O::RegA, O::RegB),
        0xa9 => I::XOR(O::RegA, O::RegC),
        0xaa => I::XOR(O::RegA, O::RegD),
        0xab => I::XOR(O::RegA, O::RegE),
        0xac => I::XOR(O::RegA, O::RegH),
        0xad => I::XOR(O::RegA, O::RegL),
        0xae => I::XOR(O::RegA, O::AddrHL),
        0xee => I::XOR(O::RegA, O::Imm8(try!(self.imm8(m, &mut pc, addr)))),

        _ => I::Invalid(op),
        // _ => panic!("instruction_at: instruction not implemented: 0x{:02x}", op),
      };

      Ok((ins, pc))
    }
  }
}
