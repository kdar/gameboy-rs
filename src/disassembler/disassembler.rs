use super::super::Reg;
use super::super::Flag;
use super::super::operand::{Operand, Addr, Imm};
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

    let imm8 = move |pc: &mut u16| -> Result<Operand, String> {
      let n = try!(m.read_u8(addr + *pc));
      *pc += 1;
      Ok(Operand::Imm(Imm::Imm8(n)))
    };

    let imm16 = move |pc: &mut u16| -> Result<Operand, String> {
      let nn = try!(m.read_u16(addr + *pc));
      *pc += 2;
      Ok(Operand::Imm(Imm::Imm16(nn)))
    };

    let imm_addr = move |pc: &mut u16| -> Result<Operand, String> {
      let nn = try!(m.read_u16(addr + *pc));
      *pc += 2;
      Ok(Operand::Addr(Addr::Imm16(nn)))
    };

    let op = try!(m.read_u8(addr + pc));
    pc += 1;

    if op == 0xCB {
      let op = try!(m.read_u8(addr + pc));
      pc += 1;
      let ins = match op {
        0x47 => Instruction::BIT(Operand::Imm(Imm::Imm8(0)), Operand::Reg(Reg::A)),
        0x4f => Instruction::BIT(Operand::Imm(Imm::Imm8(1)), Operand::Reg(Reg::A)),
        0x57 => Instruction::BIT(Operand::Imm(Imm::Imm8(2)), Operand::Reg(Reg::A)),
        0x5f => Instruction::BIT(Operand::Imm(Imm::Imm8(3)), Operand::Reg(Reg::A)),
        0x67 => Instruction::BIT(Operand::Imm(Imm::Imm8(4)), Operand::Reg(Reg::A)),
        0x6f => Instruction::BIT(Operand::Imm(Imm::Imm8(5)), Operand::Reg(Reg::A)),
        0x77 => Instruction::BIT(Operand::Imm(Imm::Imm8(6)), Operand::Reg(Reg::A)),
        0x7f => Instruction::BIT(Operand::Imm(Imm::Imm8(7)), Operand::Reg(Reg::A)),
        0x40 => Instruction::BIT(Operand::Imm(Imm::Imm8(0)), Operand::Reg(Reg::B)),
        0x48 => Instruction::BIT(Operand::Imm(Imm::Imm8(1)), Operand::Reg(Reg::B)),
        0x50 => Instruction::BIT(Operand::Imm(Imm::Imm8(2)), Operand::Reg(Reg::B)),
        0x58 => Instruction::BIT(Operand::Imm(Imm::Imm8(3)), Operand::Reg(Reg::B)),
        0x60 => Instruction::BIT(Operand::Imm(Imm::Imm8(4)), Operand::Reg(Reg::B)),
        0x68 => Instruction::BIT(Operand::Imm(Imm::Imm8(5)), Operand::Reg(Reg::B)),
        0x70 => Instruction::BIT(Operand::Imm(Imm::Imm8(6)), Operand::Reg(Reg::B)),
        0x78 => Instruction::BIT(Operand::Imm(Imm::Imm8(7)), Operand::Reg(Reg::B)),
        0x41 => Instruction::BIT(Operand::Imm(Imm::Imm8(0)), Operand::Reg(Reg::C)),
        0x49 => Instruction::BIT(Operand::Imm(Imm::Imm8(1)), Operand::Reg(Reg::C)),
        0x51 => Instruction::BIT(Operand::Imm(Imm::Imm8(2)), Operand::Reg(Reg::C)),
        0x59 => Instruction::BIT(Operand::Imm(Imm::Imm8(3)), Operand::Reg(Reg::C)),
        0x61 => Instruction::BIT(Operand::Imm(Imm::Imm8(4)), Operand::Reg(Reg::C)),
        0x69 => Instruction::BIT(Operand::Imm(Imm::Imm8(5)), Operand::Reg(Reg::C)),
        0x71 => Instruction::BIT(Operand::Imm(Imm::Imm8(6)), Operand::Reg(Reg::C)),
        0x79 => Instruction::BIT(Operand::Imm(Imm::Imm8(7)), Operand::Reg(Reg::C)),
        0x42 => Instruction::BIT(Operand::Imm(Imm::Imm8(0)), Operand::Reg(Reg::D)),
        0x4a => Instruction::BIT(Operand::Imm(Imm::Imm8(1)), Operand::Reg(Reg::D)),
        0x52 => Instruction::BIT(Operand::Imm(Imm::Imm8(2)), Operand::Reg(Reg::D)),
        0x5a => Instruction::BIT(Operand::Imm(Imm::Imm8(3)), Operand::Reg(Reg::D)),
        0x62 => Instruction::BIT(Operand::Imm(Imm::Imm8(4)), Operand::Reg(Reg::D)),
        0x6a => Instruction::BIT(Operand::Imm(Imm::Imm8(5)), Operand::Reg(Reg::D)),
        0x72 => Instruction::BIT(Operand::Imm(Imm::Imm8(6)), Operand::Reg(Reg::D)),
        0x7a => Instruction::BIT(Operand::Imm(Imm::Imm8(7)), Operand::Reg(Reg::D)),
        0x43 => Instruction::BIT(Operand::Imm(Imm::Imm8(0)), Operand::Reg(Reg::E)),
        0x4b => Instruction::BIT(Operand::Imm(Imm::Imm8(1)), Operand::Reg(Reg::E)),
        0x53 => Instruction::BIT(Operand::Imm(Imm::Imm8(2)), Operand::Reg(Reg::E)),
        0x5b => Instruction::BIT(Operand::Imm(Imm::Imm8(3)), Operand::Reg(Reg::E)),
        0x63 => Instruction::BIT(Operand::Imm(Imm::Imm8(4)), Operand::Reg(Reg::E)),
        0x6b => Instruction::BIT(Operand::Imm(Imm::Imm8(5)), Operand::Reg(Reg::E)),
        0x73 => Instruction::BIT(Operand::Imm(Imm::Imm8(6)), Operand::Reg(Reg::E)),
        0x7b => Instruction::BIT(Operand::Imm(Imm::Imm8(7)), Operand::Reg(Reg::E)),
        0x44 => Instruction::BIT(Operand::Imm(Imm::Imm8(0)), Operand::Reg(Reg::H)),
        0x4c => Instruction::BIT(Operand::Imm(Imm::Imm8(1)), Operand::Reg(Reg::H)),
        0x54 => Instruction::BIT(Operand::Imm(Imm::Imm8(2)), Operand::Reg(Reg::H)),
        0x5c => Instruction::BIT(Operand::Imm(Imm::Imm8(3)), Operand::Reg(Reg::H)),
        0x64 => Instruction::BIT(Operand::Imm(Imm::Imm8(4)), Operand::Reg(Reg::H)),
        0x6c => Instruction::BIT(Operand::Imm(Imm::Imm8(5)), Operand::Reg(Reg::H)),
        0x74 => Instruction::BIT(Operand::Imm(Imm::Imm8(6)), Operand::Reg(Reg::H)),
        0x7c => Instruction::BIT(Operand::Imm(Imm::Imm8(7)), Operand::Reg(Reg::H)),
        0x45 => Instruction::BIT(Operand::Imm(Imm::Imm8(0)), Operand::Reg(Reg::L)),
        0x4d => Instruction::BIT(Operand::Imm(Imm::Imm8(1)), Operand::Reg(Reg::L)),
        0x55 => Instruction::BIT(Operand::Imm(Imm::Imm8(2)), Operand::Reg(Reg::L)),
        0x5d => Instruction::BIT(Operand::Imm(Imm::Imm8(3)), Operand::Reg(Reg::L)),
        0x65 => Instruction::BIT(Operand::Imm(Imm::Imm8(4)), Operand::Reg(Reg::L)),
        0x6d => Instruction::BIT(Operand::Imm(Imm::Imm8(5)), Operand::Reg(Reg::L)),
        0x75 => Instruction::BIT(Operand::Imm(Imm::Imm8(6)), Operand::Reg(Reg::L)),
        0x7d => Instruction::BIT(Operand::Imm(Imm::Imm8(7)), Operand::Reg(Reg::L)),
        0x46 => Instruction::BIT(Operand::Imm(Imm::Imm8(0)), Operand::Addr(Addr::HL)),
        0x4e => Instruction::BIT(Operand::Imm(Imm::Imm8(1)), Operand::Addr(Addr::HL)),
        0x56 => Instruction::BIT(Operand::Imm(Imm::Imm8(2)), Operand::Addr(Addr::HL)),
        0x5e => Instruction::BIT(Operand::Imm(Imm::Imm8(3)), Operand::Addr(Addr::HL)),
        0x66 => Instruction::BIT(Operand::Imm(Imm::Imm8(4)), Operand::Addr(Addr::HL)),
        0x6e => Instruction::BIT(Operand::Imm(Imm::Imm8(5)), Operand::Addr(Addr::HL)),
        0x76 => Instruction::BIT(Operand::Imm(Imm::Imm8(6)), Operand::Addr(Addr::HL)),
        0x7e => Instruction::BIT(Operand::Imm(Imm::Imm8(7)), Operand::Addr(Addr::HL)),

        0x17 => Instruction::RL(Operand::Reg(Reg::A)),
        0x10 => Instruction::RL(Operand::Reg(Reg::B)),
        0x11 => Instruction::RL(Operand::Reg(Reg::C)),
        0x12 => Instruction::RL(Operand::Reg(Reg::D)),
        0x13 => Instruction::RL(Operand::Reg(Reg::E)),
        0x14 => Instruction::RL(Operand::Reg(Reg::H)),
        0x15 => Instruction::RL(Operand::Reg(Reg::L)),
        0x16 => Instruction::RL(Operand::Addr(Addr::HL)),

        0x1f => Instruction::RR(Operand::Reg(Reg::A)),
        0x18 => Instruction::RR(Operand::Reg(Reg::B)),
        0x19 => Instruction::RR(Operand::Reg(Reg::C)),
        0x1a => Instruction::RR(Operand::Reg(Reg::D)),
        0x1b => Instruction::RR(Operand::Reg(Reg::E)),
        0x1c => Instruction::RR(Operand::Reg(Reg::H)),
        0x1d => Instruction::RR(Operand::Reg(Reg::L)),
        0x1e => Instruction::RR(Operand::Addr(Addr::HL)),

        0x3f => Instruction::SRL(Operand::Reg(Reg::A)),
        0x38 => Instruction::SRL(Operand::Reg(Reg::B)),
        0x39 => Instruction::SRL(Operand::Reg(Reg::C)),
        0x3a => Instruction::SRL(Operand::Reg(Reg::D)),
        0x3b => Instruction::SRL(Operand::Reg(Reg::E)),
        0x3c => Instruction::SRL(Operand::Reg(Reg::H)),
        0x3d => Instruction::SRL(Operand::Reg(Reg::L)),
        0x3e => Instruction::SRL(Operand::Addr(Addr::HL)),

        0x37 => Instruction::SWAP(Operand::Reg(Reg::A)),
        0x30 => Instruction::SWAP(Operand::Reg(Reg::B)),
        0x31 => Instruction::SWAP(Operand::Reg(Reg::C)),
        0x32 => Instruction::SWAP(Operand::Reg(Reg::D)),
        0x33 => Instruction::SWAP(Operand::Reg(Reg::E)),
        0x34 => Instruction::SWAP(Operand::Reg(Reg::H)),
        0x35 => Instruction::SWAP(Operand::Reg(Reg::L)),
        0x36 => Instruction::SWAP(Operand::Addr(Addr::HL)),

        _ => {
          panic!("instruction_at: 0xCB instruction not implemented: 0x{:02x}",
                 op)
        }
      };

      Ok((ins, pc))
    } else {
      let ins = match op {
        0x8f => Instruction::ADC(Operand::Reg(Reg::A), Operand::Reg(Reg::A)),
        0x88 => Instruction::ADC(Operand::Reg(Reg::A), Operand::Reg(Reg::B)),
        0x89 => Instruction::ADC(Operand::Reg(Reg::A), Operand::Reg(Reg::C)),
        0x8a => Instruction::ADC(Operand::Reg(Reg::A), Operand::Reg(Reg::D)),
        0x8b => Instruction::ADC(Operand::Reg(Reg::A), Operand::Reg(Reg::E)),
        0x8c => Instruction::ADC(Operand::Reg(Reg::A), Operand::Reg(Reg::H)),
        0x8d => Instruction::ADC(Operand::Reg(Reg::A), Operand::Reg(Reg::L)),
        0x8e => Instruction::ADC(Operand::Reg(Reg::A), Operand::Addr(Addr::HL)),
        0xce => Instruction::ADC(Operand::Reg(Reg::A), try!(imm8(&mut pc))),

        0x86 => Instruction::ADD8(Operand::Reg(Reg::A), Operand::Addr(Addr::HL)),
        0xc6 => Instruction::ADD8(Operand::Reg(Reg::A), try!(imm8(&mut pc))),
        0x09 => Instruction::ADD16(Operand::Reg(Reg::HL), Operand::Reg(Reg::BC)),
        0x19 => Instruction::ADD16(Operand::Reg(Reg::HL), Operand::Reg(Reg::DE)),
        0x29 => Instruction::ADD16(Operand::Reg(Reg::HL), Operand::Reg(Reg::HL)),
        0x39 => Instruction::ADD16(Operand::Reg(Reg::HL), Operand::Reg(Reg::SP)),

        0xa7 => Instruction::AND(Operand::Reg(Reg::A)),
        0xa0 => Instruction::AND(Operand::Reg(Reg::B)),
        0xa1 => Instruction::AND(Operand::Reg(Reg::C)),
        0xa2 => Instruction::AND(Operand::Reg(Reg::D)),
        0xa3 => Instruction::AND(Operand::Reg(Reg::E)),
        0xa4 => Instruction::AND(Operand::Reg(Reg::H)),
        0xa5 => Instruction::AND(Operand::Reg(Reg::L)),
        0xa6 => Instruction::AND(Operand::Addr(Addr::HL)),
        0xe6 => Instruction::AND(try!(imm8(&mut pc))),

        0xc4 => Instruction::CALL_cc(Operand::Flag(Flag::NZ), try!(imm16(&mut pc))),
        0xcc => Instruction::CALL_cc(Operand::Flag(Flag::Z), try!(imm16(&mut pc))),
        0xd4 => Instruction::CALL_cc(Operand::Flag(Flag::NC), try!(imm16(&mut pc))),
        0xdc => Instruction::CALL_cc(Operand::Flag(Flag::C), try!(imm16(&mut pc))),
        0xcd => Instruction::CALL(try!(imm16(&mut pc))),

        0xbf => Instruction::CP(Operand::Reg(Reg::A)),
        0xb8 => Instruction::CP(Operand::Reg(Reg::B)),
        0xb9 => Instruction::CP(Operand::Reg(Reg::C)),
        0xba => Instruction::CP(Operand::Reg(Reg::D)),
        0xbb => Instruction::CP(Operand::Reg(Reg::E)),
        0xbc => Instruction::CP(Operand::Reg(Reg::H)),
        0xbd => Instruction::CP(Operand::Reg(Reg::L)),
        0xbe => Instruction::CP(Operand::Addr(Addr::HL)),
        0xfe => Instruction::CP(try!(imm8(&mut pc))),

        0x3d => Instruction::DEC8(Operand::Reg(Reg::A)),
        0x05 => Instruction::DEC8(Operand::Reg(Reg::B)),
        0x0d => Instruction::DEC8(Operand::Reg(Reg::C)),
        0x15 => Instruction::DEC8(Operand::Reg(Reg::D)),
        0x1d => Instruction::DEC8(Operand::Reg(Reg::E)),
        0x25 => Instruction::DEC8(Operand::Reg(Reg::H)),
        0x2d => Instruction::DEC8(Operand::Reg(Reg::L)),
        0x35 => Instruction::DEC8(Operand::Addr(Addr::HL)),
        0x0b => Instruction::DEC16(Operand::Reg(Reg::BC)),
        0x1b => Instruction::DEC16(Operand::Reg(Reg::DE)),
        0x2b => Instruction::DEC16(Operand::Reg(Reg::HL)),
        0x3b => Instruction::DEC16(Operand::Reg(Reg::SP)),

        0xf3 => Instruction::DI,

        0xfb => Instruction::EI,

        0x76 => Instruction::HALT,

        0x3c => Instruction::INC8(Operand::Reg(Reg::A)),
        0x04 => Instruction::INC8(Operand::Reg(Reg::B)),
        0x0c => Instruction::INC8(Operand::Reg(Reg::C)),
        0x14 => Instruction::INC8(Operand::Reg(Reg::D)),
        0x1c => Instruction::INC8(Operand::Reg(Reg::E)),
        0x24 => Instruction::INC8(Operand::Reg(Reg::H)),
        0x2c => Instruction::INC8(Operand::Reg(Reg::L)),
        0x34 => Instruction::INC8(Operand::Addr(Addr::HL)),
        0x03 => Instruction::INC16(Operand::Reg(Reg::BC)),
        0x13 => Instruction::INC16(Operand::Reg(Reg::DE)),
        0x23 => Instruction::INC16(Operand::Reg(Reg::HL)),
        0x33 => Instruction::INC16(Operand::Reg(Reg::SP)),

        0xc2 => Instruction::JP_cc(Operand::Flag(Flag::NZ), try!(imm16(&mut pc))),
        0xca => Instruction::JP_cc(Operand::Flag(Flag::Z), try!(imm16(&mut pc))),
        0xd2 => Instruction::JP_cc(Operand::Flag(Flag::NC), try!(imm16(&mut pc))),
        0xda => Instruction::JP_cc(Operand::Flag(Flag::C), try!(imm16(&mut pc))),
        0xe9 => Instruction::JP(Operand::Reg(Reg::HL)),
        0xc3 => Instruction::JP(try!(imm16(&mut pc))),

        0x20 => Instruction::JR_cc(Operand::Flag(Flag::NZ), try!(imm8(&mut pc))),
        0x28 => Instruction::JR_cc(Operand::Flag(Flag::Z), try!(imm8(&mut pc))),
        0x30 => Instruction::JR_cc(Operand::Flag(Flag::NC), try!(imm8(&mut pc))),
        0x38 => Instruction::JR_cc(Operand::Flag(Flag::C), try!(imm8(&mut pc))),
        0x18 => Instruction::JR(try!(imm8(&mut pc))),

        0x7f => Instruction::LD8(Operand::Reg(Reg::A), Operand::Reg(Reg::A)),
        0x78 => Instruction::LD8(Operand::Reg(Reg::A), Operand::Reg(Reg::B)),
        0x79 => Instruction::LD8(Operand::Reg(Reg::A), Operand::Reg(Reg::C)),
        0x7a => Instruction::LD8(Operand::Reg(Reg::A), Operand::Reg(Reg::D)),
        0x7b => Instruction::LD8(Operand::Reg(Reg::A), Operand::Reg(Reg::E)),
        0x7c => Instruction::LD8(Operand::Reg(Reg::A), Operand::Reg(Reg::H)),
        0x7d => Instruction::LD8(Operand::Reg(Reg::A), Operand::Reg(Reg::L)),
        0x7e => Instruction::LD8(Operand::Reg(Reg::A), Operand::Addr(Addr::HL)),
        0x47 => Instruction::LD8(Operand::Reg(Reg::B), Operand::Reg(Reg::A)),
        0x40 => Instruction::LD8(Operand::Reg(Reg::B), Operand::Reg(Reg::B)),
        0x41 => Instruction::LD8(Operand::Reg(Reg::B), Operand::Reg(Reg::C)),
        0x42 => Instruction::LD8(Operand::Reg(Reg::B), Operand::Reg(Reg::D)),
        0x43 => Instruction::LD8(Operand::Reg(Reg::B), Operand::Reg(Reg::E)),
        0x44 => Instruction::LD8(Operand::Reg(Reg::B), Operand::Reg(Reg::H)),
        0x45 => Instruction::LD8(Operand::Reg(Reg::B), Operand::Reg(Reg::L)),
        0x46 => Instruction::LD8(Operand::Reg(Reg::B), Operand::Addr(Addr::HL)),
        0x4f => Instruction::LD8(Operand::Reg(Reg::C), Operand::Reg(Reg::A)),
        0x48 => Instruction::LD8(Operand::Reg(Reg::C), Operand::Reg(Reg::B)),
        0x49 => Instruction::LD8(Operand::Reg(Reg::C), Operand::Reg(Reg::C)),
        0x4a => Instruction::LD8(Operand::Reg(Reg::C), Operand::Reg(Reg::D)),
        0x4b => Instruction::LD8(Operand::Reg(Reg::C), Operand::Reg(Reg::E)),
        0x4c => Instruction::LD8(Operand::Reg(Reg::C), Operand::Reg(Reg::H)),
        0x4d => Instruction::LD8(Operand::Reg(Reg::C), Operand::Reg(Reg::L)),
        0x4e => Instruction::LD8(Operand::Reg(Reg::C), Operand::Addr(Addr::HL)),
        0x57 => Instruction::LD8(Operand::Reg(Reg::D), Operand::Reg(Reg::A)),
        0x50 => Instruction::LD8(Operand::Reg(Reg::D), Operand::Reg(Reg::B)),
        0x51 => Instruction::LD8(Operand::Reg(Reg::D), Operand::Reg(Reg::C)),
        0x52 => Instruction::LD8(Operand::Reg(Reg::D), Operand::Reg(Reg::D)),
        0x53 => Instruction::LD8(Operand::Reg(Reg::D), Operand::Reg(Reg::E)),
        0x54 => Instruction::LD8(Operand::Reg(Reg::D), Operand::Reg(Reg::H)),
        0x55 => Instruction::LD8(Operand::Reg(Reg::D), Operand::Reg(Reg::L)),
        0x56 => Instruction::LD8(Operand::Reg(Reg::D), Operand::Addr(Addr::HL)),
        0x5f => Instruction::LD8(Operand::Reg(Reg::E), Operand::Reg(Reg::A)),
        0x59 => Instruction::LD8(Operand::Reg(Reg::E), Operand::Reg(Reg::C)),
        0x58 => Instruction::LD8(Operand::Reg(Reg::E), Operand::Reg(Reg::B)),
        0x5a => Instruction::LD8(Operand::Reg(Reg::E), Operand::Reg(Reg::D)),
        0x5b => Instruction::LD8(Operand::Reg(Reg::E), Operand::Reg(Reg::E)),
        0x5c => Instruction::LD8(Operand::Reg(Reg::E), Operand::Reg(Reg::H)),
        0x5d => Instruction::LD8(Operand::Reg(Reg::E), Operand::Reg(Reg::L)),
        0x5e => Instruction::LD8(Operand::Reg(Reg::E), Operand::Addr(Addr::HL)),
        0x67 => Instruction::LD8(Operand::Reg(Reg::H), Operand::Reg(Reg::A)),
        0x60 => Instruction::LD8(Operand::Reg(Reg::H), Operand::Reg(Reg::B)),
        0x61 => Instruction::LD8(Operand::Reg(Reg::H), Operand::Reg(Reg::C)),
        0x62 => Instruction::LD8(Operand::Reg(Reg::H), Operand::Reg(Reg::D)),
        0x63 => Instruction::LD8(Operand::Reg(Reg::H), Operand::Reg(Reg::E)),
        0x64 => Instruction::LD8(Operand::Reg(Reg::H), Operand::Reg(Reg::H)),
        0x65 => Instruction::LD8(Operand::Reg(Reg::H), Operand::Reg(Reg::L)),
        0x66 => Instruction::LD8(Operand::Reg(Reg::H), Operand::Addr(Addr::HL)),
        0x6f => Instruction::LD8(Operand::Reg(Reg::L), Operand::Reg(Reg::A)),
        0x68 => Instruction::LD8(Operand::Reg(Reg::L), Operand::Reg(Reg::B)),
        0x69 => Instruction::LD8(Operand::Reg(Reg::L), Operand::Reg(Reg::C)),
        0x6a => Instruction::LD8(Operand::Reg(Reg::L), Operand::Reg(Reg::D)),
        0x6b => Instruction::LD8(Operand::Reg(Reg::L), Operand::Reg(Reg::E)),
        0x6c => Instruction::LD8(Operand::Reg(Reg::L), Operand::Reg(Reg::H)),
        0x6d => Instruction::LD8(Operand::Reg(Reg::L), Operand::Reg(Reg::L)),
        0x6e => Instruction::LD8(Operand::Reg(Reg::L), Operand::Addr(Addr::HL)),
        0x3e => Instruction::LD8(Operand::Reg(Reg::A), try!(imm8(&mut pc))),
        0x06 => Instruction::LD8(Operand::Reg(Reg::B), try!(imm8(&mut pc))),
        0x0e => Instruction::LD8(Operand::Reg(Reg::C), try!(imm8(&mut pc))),
        0x16 => Instruction::LD8(Operand::Reg(Reg::D), try!(imm8(&mut pc))),
        0x1e => Instruction::LD8(Operand::Reg(Reg::E), try!(imm8(&mut pc))),
        0x26 => Instruction::LD8(Operand::Reg(Reg::H), try!(imm8(&mut pc))),
        0x2e => Instruction::LD8(Operand::Reg(Reg::L), try!(imm8(&mut pc))),
        0x36 => Instruction::LD8(Operand::Addr(Addr::HL), try!(imm8(&mut pc))),
        0x77 => Instruction::LD8(Operand::Addr(Addr::HL), Operand::Reg(Reg::A)),
        0x70 => Instruction::LD8(Operand::Addr(Addr::HL), Operand::Reg(Reg::B)),
        0x71 => Instruction::LD8(Operand::Addr(Addr::HL), Operand::Reg(Reg::C)),
        0x72 => Instruction::LD8(Operand::Addr(Addr::HL), Operand::Reg(Reg::D)),
        0x73 => Instruction::LD8(Operand::Addr(Addr::HL), Operand::Reg(Reg::E)),
        0x74 => Instruction::LD8(Operand::Addr(Addr::HL), Operand::Reg(Reg::H)),
        0x75 => Instruction::LD8(Operand::Addr(Addr::HL), Operand::Reg(Reg::L)),
        0x0a => Instruction::LD8(Operand::Reg(Reg::A), Operand::Addr(Addr::BC)),
        0x02 => Instruction::LD8(Operand::Addr(Addr::BC), Operand::Reg(Reg::A)),
        0x1a => Instruction::LD8(Operand::Reg(Reg::A), Operand::Addr(Addr::DE)),
        0x12 => Instruction::LD8(Operand::Addr(Addr::DE), Operand::Reg(Reg::A)),

        0xfa => Instruction::LD8(Operand::Reg(Reg::A), try!(imm_addr(&mut pc))),

        0x01 => Instruction::LD16(Operand::Reg(Reg::BC), try!(imm16(&mut pc))),
        0x11 => Instruction::LD16(Operand::Reg(Reg::DE), try!(imm16(&mut pc))),
        0x21 => Instruction::LD16(Operand::Reg(Reg::HL), try!(imm16(&mut pc))),
        0x31 => Instruction::LD16(Operand::Reg(Reg::SP), try!(imm16(&mut pc))),

        0xea => Instruction::LD8(try!(imm_addr(&mut pc)), Operand::Reg(Reg::A)),
        // 0xe0 => {
        //  let nn = try!(m.read_u16(addr + pc));
        //  pc += 2;
        //  Ok((inst(Operand::Addr(Addr::Imm16(nn)), operand2)
        // }
        //
        //
        //
        // 0xe2 Instruction::LD_·0xFF00C·_A, pc)),
        // 0xe0 => {
        //  let n = try!(m.read_u8(addr + pc));
        //  pc += 1;
        //  Ok((Instruction::LD_·0xFF00n·_A(n), pc))
        // }
        //
        // 0x02 Instruction::LD_·BC·_A, pc)),
        // 0x12 Instruction::LD_·DE·_A, pc)),
        //
        // 0x36 => {
        //  let n = try!(m.read_u8(addr + pc));
        //  pc += 1;
        //  Ok((Instruction::LD_·HL·_n(n), pc))
        // }
        //
        // 0x70 | 0x71 | 0x72 | 0x73 | 0x74 | 0x75 | 0x77 => {
        //  let r = op & 0b111;
        //  Ok((Instruction::LD_·HL·_r(Operand::Reg(Reg::from(r))), pc))
        // }
        //
        // 0xea => {
        //  let nn = try!(m.read_u16(addr + pc));
        //  pc += 2;
        //  Ok((Instruction::LD_·nn·_A(nn), pc))
        // }
        //
        // 0x08 => {
        //  let nn = try!(m.read_u16(addr + pc));
        //  pc += 2;
        //  Ok((Instruction::LD_·nn·_SP(nn), pc))
        // }
        //
        // 0x0a Instruction::LD_A_·BC·, pc)),
        //
        // 0x1a Instruction::LD_A_·DE·, pc)),
        //
        // 0xfa => {
        //  let nn = try!(m.read_u16(addr + pc));
        //  pc += 2;
        //  Ok((Instruction::LD_A_·nn·(nn), pc))
        // }
        //
        // 0xf0 => {
        //  let n = try!(m.read_u8(addr + pc));
        //  pc += 1;
        //  Ok((Instruction::LD_A_·0xFF00n·(n), pc))
        // }
        //
        // 0x01 | 0x11 | 0x21 | 0x31 => {
        //  let r = op >> 4 & 0b11;
        //  let nn = try!(m.read_u16(addr + pc));
        //  pc += 2;
        //  Ok((Instruction::LD_dd_nn(Reg::from_pair(r, false), nn), pc))
        // }
        // 0x46 | 0x4e | 0x56 | 0x5e | 0x66 | 0x6e | 0x7e => {
        //  let r = op >> 3 & 0b111;
        //  Ok((Instruction::LD_r_·HL·(Operand::Reg(Reg::from(r))), pc))
        // }
        //
        // 0x06 | 0x0e | 0x16 | 0x1e | 0x26 | 0x2e | 0x3e => {
        //  let r = op >> 3 & 0b111;
        //  let n = try!(m.read_u8(addr + pc));
        //  pc += 1;
        //  Ok((Instruction::LD_r_n(Operand::Reg(Reg::from(r)), n), pc))
        // }
        //
        // 0x40 | 0x41 | 0x42 | 0x43 | 0x44 | 0x45 | 0x47 | 0x48 | 0x49 | 0x4a | 0x4b | 0x4c | 0x4d | 0x4f | 0x50 | 0x51 | 0x52 | 0x53 |
        // 0x54 | 0x55 | 0x57 | 0x58 | 0x59 | 0x5a | 0x5b | 0x5c | 0x5d | 0x5f | 0x60 | 0x61 | 0x62 | 0x63 | 0x64 | 0x65 | 0x67 | 0x68 |
        // 0x69 | 0x6a | 0x6b | 0x6c | 0x6d | 0x6f | 0x78 | 0x79 | 0x7a | 0x7b | 0x7c | 0x7d | 0x7f => {
        //  let r1 = op >> 3 & 0b111;
        //  let r2 = op & 0b111;
        //  Ok((Instruction::LD_r_r(Operand::Reg(Reg::from(r1)), Operand::Reg(Reg::from(r2))), pc))
        // }
        0x2a => Instruction::LDI_A_·HL·,
        0x32 => Instruction::LDD_·HL·_A,
        0x22 => Instruction::LDI_·HL·_A,

        0xb6 => Instruction::OR_A_·HL·,

        0xb0 | 0xb1 | 0xb2 | 0xb3 | 0xb4 | 0xb5 | 0xb7 => {
          let r = op & 0b111;
          Instruction::OR_r(Reg::from(r))
        }

        0xc1 | 0xd1 | 0xe1 | 0xf1 => {
          let rr = op >> 4 & 0b11;
          Instruction::POP_rr(Reg::from_pair(rr, true))
        }

        0xc5 | 0xd5 | 0xe5 | 0xf5 => {
          let rr = op >> 4 & 0b11;
          Instruction::PUSH_rr(Reg::from_pair(rr, true))
        }

        0xc9 => Instruction::RET,

        0xc0 | 0xc8 | 0xd0 | 0xd8 => {
          let cc = op >> 3 & 0b111;
          Instruction::RET_cc(Flag::from(cc))
        }

        0x17 => Instruction::RLA,
        0x07 => Instruction::RLCA,

        0x1f => Instruction::RRA,

        0xc7 | 0xcf | 0xd7 | 0xdf | 0xe7 | 0xef | 0xf7 | 0xff => {
          let t = op >> 3 & 0b111;
          Instruction::RST_t(t)
        }

        0xd6 => {
          let n = try!(m.read_u8(addr + pc));
          pc += 1;
          Instruction::SUB_n(n)
        }

        0x90 | 0x91 | 0x92 | 0x93 | 0x94 | 0x95 | 0x97 => {
          let r = op & 0b111;
          Instruction::SUB_r(Reg::from(r))
        }

        0x00 => Instruction::NOP,
        0xae => Instruction::XOR_·HL·,
        0xee => {
          let n = try!(m.read_u8(addr + pc));
          pc += 1;
          Instruction::XOR_n(n)
        }
        0xa8 | 0xa9 | 0xaa | 0xab | 0xac | 0xad | 0xaf => {
          let r = op & 0b111;
          Instruction::XOR_r(Reg::from(r))
        }

        _ => Instruction::Invalid(op),
        // _ => panic!("instruction_at: instruction not implemented: 0x{:02x}", op),
      };

      Ok((ins, pc))
    }
  }
}
