use super::super::Reg;
use super::super::Flag;
use super::super::operand::{Addr, Imm};
use super::instruction::Instruction;
use super::super::mem::MemoryIo;

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

  pub fn at(&self, m: &MemoryIo, addr: u16) -> Result<(Instruction, u16), String> {
    let mut pc = 0u16;

    use super::instruction::Instruction as I;
    use super::super::operand::Operand as O;

    let imm8 = move |pc: &mut u16| -> Result<u8, String> {
      let n = try!(m.read_u8(addr + *pc));
      *pc += 1;
      Ok(n)
    };

    let imm16 = move |pc: &mut u16| -> Result<u16, String> {
      let nn = try!(m.read_u16(addr + *pc));
      *pc += 2;
      Ok(nn)
    };

    let op = try!(m.read_u8(addr + pc));
    pc += 1;

    if op == 0xCB {
      let op = try!(m.read_u8(addr + pc));
      pc += 1;
      let ins = match op {
        0x47 => I::BIT(O::Imm(Imm::Imm8(0)), O::Reg(Reg::A)),
        0x4f => I::BIT(O::Imm(Imm::Imm8(1)), O::Reg(Reg::A)),
        0x57 => I::BIT(O::Imm(Imm::Imm8(2)), O::Reg(Reg::A)),
        0x5f => I::BIT(O::Imm(Imm::Imm8(3)), O::Reg(Reg::A)),
        0x67 => I::BIT(O::Imm(Imm::Imm8(4)), O::Reg(Reg::A)),
        0x6f => I::BIT(O::Imm(Imm::Imm8(5)), O::Reg(Reg::A)),
        0x77 => I::BIT(O::Imm(Imm::Imm8(6)), O::Reg(Reg::A)),
        0x7f => I::BIT(O::Imm(Imm::Imm8(7)), O::Reg(Reg::A)),
        0x40 => I::BIT(O::Imm(Imm::Imm8(0)), O::Reg(Reg::B)),
        0x48 => I::BIT(O::Imm(Imm::Imm8(1)), O::Reg(Reg::B)),
        0x50 => I::BIT(O::Imm(Imm::Imm8(2)), O::Reg(Reg::B)),
        0x58 => I::BIT(O::Imm(Imm::Imm8(3)), O::Reg(Reg::B)),
        0x60 => I::BIT(O::Imm(Imm::Imm8(4)), O::Reg(Reg::B)),
        0x68 => I::BIT(O::Imm(Imm::Imm8(5)), O::Reg(Reg::B)),
        0x70 => I::BIT(O::Imm(Imm::Imm8(6)), O::Reg(Reg::B)),
        0x78 => I::BIT(O::Imm(Imm::Imm8(7)), O::Reg(Reg::B)),
        0x41 => I::BIT(O::Imm(Imm::Imm8(0)), O::Reg(Reg::C)),
        0x49 => I::BIT(O::Imm(Imm::Imm8(1)), O::Reg(Reg::C)),
        0x51 => I::BIT(O::Imm(Imm::Imm8(2)), O::Reg(Reg::C)),
        0x59 => I::BIT(O::Imm(Imm::Imm8(3)), O::Reg(Reg::C)),
        0x61 => I::BIT(O::Imm(Imm::Imm8(4)), O::Reg(Reg::C)),
        0x69 => I::BIT(O::Imm(Imm::Imm8(5)), O::Reg(Reg::C)),
        0x71 => I::BIT(O::Imm(Imm::Imm8(6)), O::Reg(Reg::C)),
        0x79 => I::BIT(O::Imm(Imm::Imm8(7)), O::Reg(Reg::C)),
        0x42 => I::BIT(O::Imm(Imm::Imm8(0)), O::Reg(Reg::D)),
        0x4a => I::BIT(O::Imm(Imm::Imm8(1)), O::Reg(Reg::D)),
        0x52 => I::BIT(O::Imm(Imm::Imm8(2)), O::Reg(Reg::D)),
        0x5a => I::BIT(O::Imm(Imm::Imm8(3)), O::Reg(Reg::D)),
        0x62 => I::BIT(O::Imm(Imm::Imm8(4)), O::Reg(Reg::D)),
        0x6a => I::BIT(O::Imm(Imm::Imm8(5)), O::Reg(Reg::D)),
        0x72 => I::BIT(O::Imm(Imm::Imm8(6)), O::Reg(Reg::D)),
        0x7a => I::BIT(O::Imm(Imm::Imm8(7)), O::Reg(Reg::D)),
        0x43 => I::BIT(O::Imm(Imm::Imm8(0)), O::Reg(Reg::E)),
        0x4b => I::BIT(O::Imm(Imm::Imm8(1)), O::Reg(Reg::E)),
        0x53 => I::BIT(O::Imm(Imm::Imm8(2)), O::Reg(Reg::E)),
        0x5b => I::BIT(O::Imm(Imm::Imm8(3)), O::Reg(Reg::E)),
        0x63 => I::BIT(O::Imm(Imm::Imm8(4)), O::Reg(Reg::E)),
        0x6b => I::BIT(O::Imm(Imm::Imm8(5)), O::Reg(Reg::E)),
        0x73 => I::BIT(O::Imm(Imm::Imm8(6)), O::Reg(Reg::E)),
        0x7b => I::BIT(O::Imm(Imm::Imm8(7)), O::Reg(Reg::E)),
        0x44 => I::BIT(O::Imm(Imm::Imm8(0)), O::Reg(Reg::H)),
        0x4c => I::BIT(O::Imm(Imm::Imm8(1)), O::Reg(Reg::H)),
        0x54 => I::BIT(O::Imm(Imm::Imm8(2)), O::Reg(Reg::H)),
        0x5c => I::BIT(O::Imm(Imm::Imm8(3)), O::Reg(Reg::H)),
        0x64 => I::BIT(O::Imm(Imm::Imm8(4)), O::Reg(Reg::H)),
        0x6c => I::BIT(O::Imm(Imm::Imm8(5)), O::Reg(Reg::H)),
        0x74 => I::BIT(O::Imm(Imm::Imm8(6)), O::Reg(Reg::H)),
        0x7c => I::BIT(O::Imm(Imm::Imm8(7)), O::Reg(Reg::H)),
        0x45 => I::BIT(O::Imm(Imm::Imm8(0)), O::Reg(Reg::L)),
        0x4d => I::BIT(O::Imm(Imm::Imm8(1)), O::Reg(Reg::L)),
        0x55 => I::BIT(O::Imm(Imm::Imm8(2)), O::Reg(Reg::L)),
        0x5d => I::BIT(O::Imm(Imm::Imm8(3)), O::Reg(Reg::L)),
        0x65 => I::BIT(O::Imm(Imm::Imm8(4)), O::Reg(Reg::L)),
        0x6d => I::BIT(O::Imm(Imm::Imm8(5)), O::Reg(Reg::L)),
        0x75 => I::BIT(O::Imm(Imm::Imm8(6)), O::Reg(Reg::L)),
        0x7d => I::BIT(O::Imm(Imm::Imm8(7)), O::Reg(Reg::L)),
        0x46 => I::BIT(O::Imm(Imm::Imm8(0)), O::Addr(Addr::HL)),
        0x4e => I::BIT(O::Imm(Imm::Imm8(1)), O::Addr(Addr::HL)),
        0x56 => I::BIT(O::Imm(Imm::Imm8(2)), O::Addr(Addr::HL)),
        0x5e => I::BIT(O::Imm(Imm::Imm8(3)), O::Addr(Addr::HL)),
        0x66 => I::BIT(O::Imm(Imm::Imm8(4)), O::Addr(Addr::HL)),
        0x6e => I::BIT(O::Imm(Imm::Imm8(5)), O::Addr(Addr::HL)),
        0x76 => I::BIT(O::Imm(Imm::Imm8(6)), O::Addr(Addr::HL)),
        0x7e => I::BIT(O::Imm(Imm::Imm8(7)), O::Addr(Addr::HL)),

        0x17 => I::RL(O::Reg(Reg::A)),
        0x10 => I::RL(O::Reg(Reg::B)),
        0x11 => I::RL(O::Reg(Reg::C)),
        0x12 => I::RL(O::Reg(Reg::D)),
        0x13 => I::RL(O::Reg(Reg::E)),
        0x14 => I::RL(O::Reg(Reg::H)),
        0x15 => I::RL(O::Reg(Reg::L)),
        0x16 => I::RL(O::Addr(Addr::HL)),

        0x1f => I::RR(O::Reg(Reg::A)),
        0x18 => I::RR(O::Reg(Reg::B)),
        0x19 => I::RR(O::Reg(Reg::C)),
        0x1a => I::RR(O::Reg(Reg::D)),
        0x1b => I::RR(O::Reg(Reg::E)),
        0x1c => I::RR(O::Reg(Reg::H)),
        0x1d => I::RR(O::Reg(Reg::L)),
        0x1e => I::RR(O::Addr(Addr::HL)),

        0x3f => I::SRL(O::Reg(Reg::A)),
        0x38 => I::SRL(O::Reg(Reg::B)),
        0x39 => I::SRL(O::Reg(Reg::C)),
        0x3a => I::SRL(O::Reg(Reg::D)),
        0x3b => I::SRL(O::Reg(Reg::E)),
        0x3c => I::SRL(O::Reg(Reg::H)),
        0x3d => I::SRL(O::Reg(Reg::L)),
        0x3e => I::SRL(O::Addr(Addr::HL)),

        0x37 => I::SWAP(O::Reg(Reg::A)),
        0x30 => I::SWAP(O::Reg(Reg::B)),
        0x31 => I::SWAP(O::Reg(Reg::C)),
        0x32 => I::SWAP(O::Reg(Reg::D)),
        0x33 => I::SWAP(O::Reg(Reg::E)),
        0x34 => I::SWAP(O::Reg(Reg::H)),
        0x35 => I::SWAP(O::Reg(Reg::L)),
        0x36 => I::SWAP(O::Addr(Addr::HL)),

        _ => {
          panic!("instruction_at: 0xCB instruction not implemented: 0x{:02x}",
                 op)
        }
      };

      Ok((ins, pc))
    } else {
      let ins = match op {
        0x8f => I::ADC(O::Reg(Reg::A), O::Reg(Reg::A)),
        0x88 => I::ADC(O::Reg(Reg::A), O::Reg(Reg::B)),
        0x89 => I::ADC(O::Reg(Reg::A), O::Reg(Reg::C)),
        0x8a => I::ADC(O::Reg(Reg::A), O::Reg(Reg::D)),
        0x8b => I::ADC(O::Reg(Reg::A), O::Reg(Reg::E)),
        0x8c => I::ADC(O::Reg(Reg::A), O::Reg(Reg::H)),
        0x8d => I::ADC(O::Reg(Reg::A), O::Reg(Reg::L)),
        0x8e => I::ADC(O::Reg(Reg::A), O::Addr(Addr::HL)),
        0xce => I::ADC(O::Reg(Reg::A), O::Imm(Imm::Imm8(try!(imm8(&mut pc))))),

        0x86 => I::ADD8(O::Reg(Reg::A), O::Addr(Addr::HL)),
        0xc6 => I::ADD8(O::Reg(Reg::A), O::Imm(Imm::Imm8(try!(imm8(&mut pc))))),
        0x09 => I::ADD16(O::Reg(Reg::HL), O::Reg(Reg::BC)),
        0x19 => I::ADD16(O::Reg(Reg::HL), O::Reg(Reg::DE)),
        0x29 => I::ADD16(O::Reg(Reg::HL), O::Reg(Reg::HL)),
        0x39 => I::ADD16(O::Reg(Reg::HL), O::Reg(Reg::SP)),

        0xa7 => I::AND(O::Reg(Reg::A)),
        0xa0 => I::AND(O::Reg(Reg::B)),
        0xa1 => I::AND(O::Reg(Reg::C)),
        0xa2 => I::AND(O::Reg(Reg::D)),
        0xa3 => I::AND(O::Reg(Reg::E)),
        0xa4 => I::AND(O::Reg(Reg::H)),
        0xa5 => I::AND(O::Reg(Reg::L)),
        0xa6 => I::AND(O::Addr(Addr::HL)),
        0xe6 => I::AND(O::Imm(Imm::Imm8(try!(imm8(&mut pc))))),

        0xc4 => I::CALL_cc(O::Flag(Flag::NZ), O::Imm(Imm::Imm16(try!(imm16(&mut pc))))),
        0xcc => I::CALL_cc(O::Flag(Flag::Z), O::Imm(Imm::Imm16(try!(imm16(&mut pc))))),
        0xd4 => I::CALL_cc(O::Flag(Flag::NC), O::Imm(Imm::Imm16(try!(imm16(&mut pc))))),
        0xdc => I::CALL_cc(O::Flag(Flag::C), O::Imm(Imm::Imm16(try!(imm16(&mut pc))))),
        0xcd => I::CALL(O::Imm(Imm::Imm16(try!(imm16(&mut pc))))),

        0xbf => I::CP(O::Reg(Reg::A)),
        0xb8 => I::CP(O::Reg(Reg::B)),
        0xb9 => I::CP(O::Reg(Reg::C)),
        0xba => I::CP(O::Reg(Reg::D)),
        0xbb => I::CP(O::Reg(Reg::E)),
        0xbc => I::CP(O::Reg(Reg::H)),
        0xbd => I::CP(O::Reg(Reg::L)),
        0xbe => I::CP(O::Addr(Addr::HL)),
        0xfe => I::CP(O::Imm(Imm::Imm8(((try!(imm8(&mut pc))))))),

        0x3d => I::DEC8(O::Reg(Reg::A)),
        0x05 => I::DEC8(O::Reg(Reg::B)),
        0x0d => I::DEC8(O::Reg(Reg::C)),
        0x15 => I::DEC8(O::Reg(Reg::D)),
        0x1d => I::DEC8(O::Reg(Reg::E)),
        0x25 => I::DEC8(O::Reg(Reg::H)),
        0x2d => I::DEC8(O::Reg(Reg::L)),
        0x35 => I::DEC8(O::Addr(Addr::HL)),
        0x0b => I::DEC16(O::Reg(Reg::BC)),
        0x1b => I::DEC16(O::Reg(Reg::DE)),
        0x2b => I::DEC16(O::Reg(Reg::HL)),
        0x3b => I::DEC16(O::Reg(Reg::SP)),

        0xf3 => I::DI,

        0xfb => I::EI,

        0x76 => I::HALT,

        0x3c => I::INC8(O::Reg(Reg::A)),
        0x04 => I::INC8(O::Reg(Reg::B)),
        0x0c => I::INC8(O::Reg(Reg::C)),
        0x14 => I::INC8(O::Reg(Reg::D)),
        0x1c => I::INC8(O::Reg(Reg::E)),
        0x24 => I::INC8(O::Reg(Reg::H)),
        0x2c => I::INC8(O::Reg(Reg::L)),
        0x34 => I::INC8(O::Addr(Addr::HL)),
        0x03 => I::INC16(O::Reg(Reg::BC)),
        0x13 => I::INC16(O::Reg(Reg::DE)),
        0x23 => I::INC16(O::Reg(Reg::HL)),
        0x33 => I::INC16(O::Reg(Reg::SP)),

        0xc2 => I::JP_cc(O::Flag(Flag::NZ), O::Imm(Imm::Imm16(try!(imm16(&mut pc))))),
        0xca => I::JP_cc(O::Flag(Flag::Z), O::Imm(Imm::Imm16(try!(imm16(&mut pc))))),
        0xd2 => I::JP_cc(O::Flag(Flag::NC), O::Imm(Imm::Imm16(try!(imm16(&mut pc))))),
        0xda => I::JP_cc(O::Flag(Flag::C), O::Imm(Imm::Imm16(try!(imm16(&mut pc))))),
        0xe9 => I::JP(O::Reg(Reg::HL)),
        0xc3 => I::JP(O::Imm(Imm::Imm16(try!(imm16(&mut pc))))),

        0x20 => I::JR_cc(O::Flag(Flag::NZ), O::Imm(Imm::Imm8(try!(imm8(&mut pc))))),
        0x28 => I::JR_cc(O::Flag(Flag::Z), O::Imm(Imm::Imm8(try!(imm8(&mut pc))))),
        0x30 => I::JR_cc(O::Flag(Flag::NC), O::Imm(Imm::Imm8(try!(imm8(&mut pc))))),
        0x38 => I::JR_cc(O::Flag(Flag::C), O::Imm(Imm::Imm8(try!(imm8(&mut pc))))),
        0x18 => I::JR(O::Imm(Imm::Imm8(try!(imm8(&mut pc))))),

        0x7f => I::LD8(O::Reg(Reg::A), O::Reg(Reg::A)),
        0x78 => I::LD8(O::Reg(Reg::A), O::Reg(Reg::B)),
        0x79 => I::LD8(O::Reg(Reg::A), O::Reg(Reg::C)),
        0x7a => I::LD8(O::Reg(Reg::A), O::Reg(Reg::D)),
        0x7b => I::LD8(O::Reg(Reg::A), O::Reg(Reg::E)),
        0x7c => I::LD8(O::Reg(Reg::A), O::Reg(Reg::H)),
        0x7d => I::LD8(O::Reg(Reg::A), O::Reg(Reg::L)),
        0x7e => I::LD8(O::Reg(Reg::A), O::Addr(Addr::HL)),
        0x47 => I::LD8(O::Reg(Reg::B), O::Reg(Reg::A)),
        0x40 => I::LD8(O::Reg(Reg::B), O::Reg(Reg::B)),
        0x41 => I::LD8(O::Reg(Reg::B), O::Reg(Reg::C)),
        0x42 => I::LD8(O::Reg(Reg::B), O::Reg(Reg::D)),
        0x43 => I::LD8(O::Reg(Reg::B), O::Reg(Reg::E)),
        0x44 => I::LD8(O::Reg(Reg::B), O::Reg(Reg::H)),
        0x45 => I::LD8(O::Reg(Reg::B), O::Reg(Reg::L)),
        0x46 => I::LD8(O::Reg(Reg::B), O::Addr(Addr::HL)),
        0x4f => I::LD8(O::Reg(Reg::C), O::Reg(Reg::A)),
        0x48 => I::LD8(O::Reg(Reg::C), O::Reg(Reg::B)),
        0x49 => I::LD8(O::Reg(Reg::C), O::Reg(Reg::C)),
        0x4a => I::LD8(O::Reg(Reg::C), O::Reg(Reg::D)),
        0x4b => I::LD8(O::Reg(Reg::C), O::Reg(Reg::E)),
        0x4c => I::LD8(O::Reg(Reg::C), O::Reg(Reg::H)),
        0x4d => I::LD8(O::Reg(Reg::C), O::Reg(Reg::L)),
        0x4e => I::LD8(O::Reg(Reg::C), O::Addr(Addr::HL)),
        0x57 => I::LD8(O::Reg(Reg::D), O::Reg(Reg::A)),
        0x50 => I::LD8(O::Reg(Reg::D), O::Reg(Reg::B)),
        0x51 => I::LD8(O::Reg(Reg::D), O::Reg(Reg::C)),
        0x52 => I::LD8(O::Reg(Reg::D), O::Reg(Reg::D)),
        0x53 => I::LD8(O::Reg(Reg::D), O::Reg(Reg::E)),
        0x54 => I::LD8(O::Reg(Reg::D), O::Reg(Reg::H)),
        0x55 => I::LD8(O::Reg(Reg::D), O::Reg(Reg::L)),
        0x56 => I::LD8(O::Reg(Reg::D), O::Addr(Addr::HL)),
        0x5f => I::LD8(O::Reg(Reg::E), O::Reg(Reg::A)),
        0x59 => I::LD8(O::Reg(Reg::E), O::Reg(Reg::C)),
        0x58 => I::LD8(O::Reg(Reg::E), O::Reg(Reg::B)),
        0x5a => I::LD8(O::Reg(Reg::E), O::Reg(Reg::D)),
        0x5b => I::LD8(O::Reg(Reg::E), O::Reg(Reg::E)),
        0x5c => I::LD8(O::Reg(Reg::E), O::Reg(Reg::H)),
        0x5d => I::LD8(O::Reg(Reg::E), O::Reg(Reg::L)),
        0x5e => I::LD8(O::Reg(Reg::E), O::Addr(Addr::HL)),
        0x67 => I::LD8(O::Reg(Reg::H), O::Reg(Reg::A)),
        0x60 => I::LD8(O::Reg(Reg::H), O::Reg(Reg::B)),
        0x61 => I::LD8(O::Reg(Reg::H), O::Reg(Reg::C)),
        0x62 => I::LD8(O::Reg(Reg::H), O::Reg(Reg::D)),
        0x63 => I::LD8(O::Reg(Reg::H), O::Reg(Reg::E)),
        0x64 => I::LD8(O::Reg(Reg::H), O::Reg(Reg::H)),
        0x65 => I::LD8(O::Reg(Reg::H), O::Reg(Reg::L)),
        0x66 => I::LD8(O::Reg(Reg::H), O::Addr(Addr::HL)),
        0x6f => I::LD8(O::Reg(Reg::L), O::Reg(Reg::A)),
        0x68 => I::LD8(O::Reg(Reg::L), O::Reg(Reg::B)),
        0x69 => I::LD8(O::Reg(Reg::L), O::Reg(Reg::C)),
        0x6a => I::LD8(O::Reg(Reg::L), O::Reg(Reg::D)),
        0x6b => I::LD8(O::Reg(Reg::L), O::Reg(Reg::E)),
        0x6c => I::LD8(O::Reg(Reg::L), O::Reg(Reg::H)),
        0x6d => I::LD8(O::Reg(Reg::L), O::Reg(Reg::L)),
        0x6e => I::LD8(O::Reg(Reg::L), O::Addr(Addr::HL)),
        0x3e => I::LD8(O::Reg(Reg::A), O::Imm(Imm::Imm8(try!(imm8(&mut pc))))),
        0x06 => I::LD8(O::Reg(Reg::B), O::Imm(Imm::Imm8(try!(imm8(&mut pc))))),
        0x0e => I::LD8(O::Reg(Reg::C), O::Imm(Imm::Imm8(try!(imm8(&mut pc))))),
        0x16 => I::LD8(O::Reg(Reg::D), O::Imm(Imm::Imm8(try!(imm8(&mut pc))))),
        0x1e => I::LD8(O::Reg(Reg::E), O::Imm(Imm::Imm8(try!(imm8(&mut pc))))),
        0x26 => I::LD8(O::Reg(Reg::H), O::Imm(Imm::Imm8(try!(imm8(&mut pc))))),
        0x2e => I::LD8(O::Reg(Reg::L), O::Imm(Imm::Imm8(try!(imm8(&mut pc))))),
        0x36 => I::LD8(O::Addr(Addr::HL), O::Imm(Imm::Imm8(try!(imm8(&mut pc))))),
        0x77 => I::LD8(O::Addr(Addr::HL), O::Reg(Reg::A)),
        0x70 => I::LD8(O::Addr(Addr::HL), O::Reg(Reg::B)),
        0x71 => I::LD8(O::Addr(Addr::HL), O::Reg(Reg::C)),
        0x72 => I::LD8(O::Addr(Addr::HL), O::Reg(Reg::D)),
        0x73 => I::LD8(O::Addr(Addr::HL), O::Reg(Reg::E)),
        0x74 => I::LD8(O::Addr(Addr::HL), O::Reg(Reg::H)),
        0x75 => I::LD8(O::Addr(Addr::HL), O::Reg(Reg::L)),
        0x0a => I::LD8(O::Reg(Reg::A), O::Addr(Addr::BC)),
        0x02 => I::LD8(O::Addr(Addr::BC), O::Reg(Reg::A)),
        0x1a => I::LD8(O::Reg(Reg::A), O::Addr(Addr::DE)),
        0x12 => I::LD8(O::Addr(Addr::DE), O::Reg(Reg::A)),
        0xfa => I::LD8(O::Reg(Reg::A), O::Addr(Addr::Imm16(try!(imm16(&mut pc))))),
        0x01 => I::LD16(O::Reg(Reg::BC), O::Imm(Imm::Imm16(try!(imm16(&mut pc))))),
        0x11 => I::LD16(O::Reg(Reg::DE), O::Imm(Imm::Imm16(try!(imm16(&mut pc))))),
        0x21 => I::LD16(O::Reg(Reg::HL), O::Imm(Imm::Imm16(try!(imm16(&mut pc))))),
        0x31 => I::LD16(O::Reg(Reg::SP), O::Imm(Imm::Imm16(try!(imm16(&mut pc))))),

        0xea => I::LD8(O::Addr(Addr::Imm16(try!(imm16(&mut pc)))), O::Reg(Reg::A)),
        0xe0 => {
          I::LD8(O::Addr(Addr::Imm16(0xff00 + try!(imm8(&mut pc)) as u16)),
                 O::Reg(Reg::A))
        }
        0xf0 => {
          I::LD8(O::Reg(Reg::A),
                 O::Addr(Addr::Imm16(0xff00 + try!(imm8(&mut pc)) as u16)))
        }

        0x22 => I::LDI(O::Addr(Addr::HL), O::Reg(Reg::A)),
        0x2a => I::LDI(O::Reg(Reg::A), O::Addr(Addr::HL)),
        0x32 => I::LDD(O::Addr(Addr::HL), O::Reg(Reg::A)),
        0x3a => I::LDD(O::Reg(Reg::A), O::Addr(Addr::HL)),

        0xb7 => I::OR(O::Reg(Reg::A), O::Reg(Reg::A)),
        0xb0 => I::OR(O::Reg(Reg::A), O::Reg(Reg::B)),
        0xb1 => I::OR(O::Reg(Reg::A), O::Reg(Reg::C)),
        0xb2 => I::OR(O::Reg(Reg::A), O::Reg(Reg::D)),
        0xb3 => I::OR(O::Reg(Reg::A), O::Reg(Reg::E)),
        0xb4 => I::OR(O::Reg(Reg::A), O::Reg(Reg::H)),
        0xb5 => I::OR(O::Reg(Reg::A), O::Reg(Reg::L)),
        0xb6 => I::OR(O::Reg(Reg::A), O::Addr(Addr::HL)),
        0xf6 => I::OR(O::Reg(Reg::A), O::Imm(Imm::Imm8(try!(imm8(&mut pc))))),

        0xc1 => I::POP16(O::Reg(Reg::BC)),
        0xd1 => I::POP16(O::Reg(Reg::DE)),
        0xe1 => I::POP16(O::Reg(Reg::HL)),
        0xf1 => I::POP16(O::Reg(Reg::AF)),
        0xc5 => I::PUSH16(O::Reg(Reg::BC)),
        0xd5 => I::PUSH16(O::Reg(Reg::DE)),
        0xe5 => I::PUSH16(O::Reg(Reg::HL)),
        0xf5 => I::PUSH16(O::Reg(Reg::AF)),

        0xc9 => I::RET,

        0xc0 => I::RET_cc(O::Flag(Flag::NZ)),
        0xc8 => I::RET_cc(O::Flag(Flag::Z)),
        0xd0 => I::RET_cc(O::Flag(Flag::NC)),
        0xd8 => I::RET_cc(O::Flag(Flag::C)),

        0x17 => I::RLA,
        0x07 => I::RLCA,

        0x1f => I::RRA,

        0xc7 | 0xcf | 0xd7 | 0xdf | 0xe7 | 0xef | 0xf7 | 0xff => {
          let t = op >> 3 & 0b111;
          I::RST_t(t)
        }

        0xd6 => {
          let n = try!(m.read_u8(addr + pc));
          pc += 1;
          I::SUB_n(n)
        }

        0x90 | 0x91 | 0x92 | 0x93 | 0x94 | 0x95 | 0x97 => {
          let r = op & 0b111;
          I::SUB_r(Reg::from(r))
        }

        0x00 => I::NOP,
        0xae => I::XOR_·HL·,
        0xee => {
          let n = try!(m.read_u8(addr + pc));
          pc += 1;
          I::XOR_n(n)
        }
        0xa8 | 0xa9 | 0xaa | 0xab | 0xac | 0xad | 0xaf => {
          let r = op & 0b111;
          I::XOR_r(Reg::from(r))
        }

        _ => I::Invalid(op),
        // _ => panic!("instruction_at: instruction not implemented: 0x{:02x}", op),
      };

      Ok((ins, pc))
    }
  }
}
