use super::super::Reg;
use super::super::Flag;
use super::super::operand::Operand;
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

    let op = try!(m.read_u8(addr + pc));
    pc += 1;

    if op == 0xCB {
      let op = try!(m.read_u8(addr + pc));
      pc += 1;
      match op {
        0x47 => Ok((Instruction::BIT(Operand::Imm8(0), Operand::A), pc)),
        0x4f => Ok((Instruction::BIT(Operand::Imm8(1), Operand::A), pc)),
        0x57 => Ok((Instruction::BIT(Operand::Imm8(2), Operand::A), pc)),
        0x5f => Ok((Instruction::BIT(Operand::Imm8(3), Operand::A), pc)),
        0x67 => Ok((Instruction::BIT(Operand::Imm8(4), Operand::A), pc)),
        0x6f => Ok((Instruction::BIT(Operand::Imm8(5), Operand::A), pc)),
        0x77 => Ok((Instruction::BIT(Operand::Imm8(6), Operand::A), pc)),
        0x7f => Ok((Instruction::BIT(Operand::Imm8(7), Operand::A), pc)),
        0x40 => Ok((Instruction::BIT(Operand::Imm8(0), Operand::B), pc)),
        0x48 => Ok((Instruction::BIT(Operand::Imm8(1), Operand::B), pc)),
        0x50 => Ok((Instruction::BIT(Operand::Imm8(2), Operand::B), pc)),
        0x58 => Ok((Instruction::BIT(Operand::Imm8(3), Operand::B), pc)),
        0x60 => Ok((Instruction::BIT(Operand::Imm8(4), Operand::B), pc)),
        0x68 => Ok((Instruction::BIT(Operand::Imm8(5), Operand::B), pc)),
        0x70 => Ok((Instruction::BIT(Operand::Imm8(6), Operand::B), pc)),
        0x78 => Ok((Instruction::BIT(Operand::Imm8(7), Operand::B), pc)),
        0x41 => Ok((Instruction::BIT(Operand::Imm8(0), Operand::C), pc)),
        0x49 => Ok((Instruction::BIT(Operand::Imm8(1), Operand::C), pc)),
        0x51 => Ok((Instruction::BIT(Operand::Imm8(2), Operand::C), pc)),
        0x59 => Ok((Instruction::BIT(Operand::Imm8(3), Operand::C), pc)),
        0x61 => Ok((Instruction::BIT(Operand::Imm8(4), Operand::C), pc)),
        0x69 => Ok((Instruction::BIT(Operand::Imm8(5), Operand::C), pc)),
        0x71 => Ok((Instruction::BIT(Operand::Imm8(6), Operand::C), pc)),
        0x79 => Ok((Instruction::BIT(Operand::Imm8(7), Operand::C), pc)),
        0x42 => Ok((Instruction::BIT(Operand::Imm8(0), Operand::D), pc)),
        0x4a => Ok((Instruction::BIT(Operand::Imm8(1), Operand::D), pc)),
        0x52 => Ok((Instruction::BIT(Operand::Imm8(2), Operand::D), pc)),
        0x5a => Ok((Instruction::BIT(Operand::Imm8(3), Operand::D), pc)),
        0x62 => Ok((Instruction::BIT(Operand::Imm8(4), Operand::D), pc)),
        0x6a => Ok((Instruction::BIT(Operand::Imm8(5), Operand::D), pc)),
        0x72 => Ok((Instruction::BIT(Operand::Imm8(6), Operand::D), pc)),
        0x7a => Ok((Instruction::BIT(Operand::Imm8(7), Operand::D), pc)),
        0x43 => Ok((Instruction::BIT(Operand::Imm8(0), Operand::E), pc)),
        0x4b => Ok((Instruction::BIT(Operand::Imm8(1), Operand::E), pc)),
        0x53 => Ok((Instruction::BIT(Operand::Imm8(2), Operand::E), pc)),
        0x5b => Ok((Instruction::BIT(Operand::Imm8(3), Operand::E), pc)),
        0x63 => Ok((Instruction::BIT(Operand::Imm8(4), Operand::E), pc)),
        0x6b => Ok((Instruction::BIT(Operand::Imm8(5), Operand::E), pc)),
        0x73 => Ok((Instruction::BIT(Operand::Imm8(6), Operand::E), pc)),
        0x7b => Ok((Instruction::BIT(Operand::Imm8(7), Operand::E), pc)),
        0x44 => Ok((Instruction::BIT(Operand::Imm8(0), Operand::H), pc)),
        0x4c => Ok((Instruction::BIT(Operand::Imm8(1), Operand::H), pc)),
        0x54 => Ok((Instruction::BIT(Operand::Imm8(2), Operand::H), pc)),
        0x5c => Ok((Instruction::BIT(Operand::Imm8(3), Operand::H), pc)),
        0x64 => Ok((Instruction::BIT(Operand::Imm8(4), Operand::H), pc)),
        0x6c => Ok((Instruction::BIT(Operand::Imm8(5), Operand::H), pc)),
        0x74 => Ok((Instruction::BIT(Operand::Imm8(6), Operand::H), pc)),
        0x7c => Ok((Instruction::BIT(Operand::Imm8(7), Operand::H), pc)),
        0x45 => Ok((Instruction::BIT(Operand::Imm8(0), Operand::L), pc)),
        0x4d => Ok((Instruction::BIT(Operand::Imm8(1), Operand::L), pc)),
        0x55 => Ok((Instruction::BIT(Operand::Imm8(2), Operand::L), pc)),
        0x5d => Ok((Instruction::BIT(Operand::Imm8(3), Operand::L), pc)),
        0x65 => Ok((Instruction::BIT(Operand::Imm8(4), Operand::L), pc)),
        0x6d => Ok((Instruction::BIT(Operand::Imm8(5), Operand::L), pc)),
        0x75 => Ok((Instruction::BIT(Operand::Imm8(6), Operand::L), pc)),
        0x7d => Ok((Instruction::BIT(Operand::Imm8(7), Operand::L), pc)),
        0x46 => Ok((Instruction::BIT(Operand::Imm8(0), Operand::_HL_), pc)),
        0x4e => Ok((Instruction::BIT(Operand::Imm8(1), Operand::_HL_), pc)),
        0x56 => Ok((Instruction::BIT(Operand::Imm8(2), Operand::_HL_), pc)),
        0x5e => Ok((Instruction::BIT(Operand::Imm8(3), Operand::_HL_), pc)),
        0x66 => Ok((Instruction::BIT(Operand::Imm8(4), Operand::_HL_), pc)),
        0x6e => Ok((Instruction::BIT(Operand::Imm8(5), Operand::_HL_), pc)),
        0x76 => Ok((Instruction::BIT(Operand::Imm8(6), Operand::_HL_), pc)),
        0x7e => Ok((Instruction::BIT(Operand::Imm8(7), Operand::_HL_), pc)),

        0x17 => Ok((Instruction::RL(Operand::A), pc)),
        0x10 => Ok((Instruction::RL(Operand::B), pc)),
        0x11 => Ok((Instruction::RL(Operand::C), pc)),
        0x12 => Ok((Instruction::RL(Operand::D), pc)),
        0x13 => Ok((Instruction::RL(Operand::E), pc)),
        0x14 => Ok((Instruction::RL(Operand::H), pc)),
        0x15 => Ok((Instruction::RL(Operand::L), pc)),
        0x16 => Ok((Instruction::RL(Operand::_HL_), pc)),

        0x1f => Ok((Instruction::RR(Operand::A), pc)),
        0x18 => Ok((Instruction::RR(Operand::B), pc)),
        0x19 => Ok((Instruction::RR(Operand::C), pc)),
        0x1a => Ok((Instruction::RR(Operand::D), pc)),
        0x1b => Ok((Instruction::RR(Operand::E), pc)),
        0x1c => Ok((Instruction::RR(Operand::H), pc)),
        0x1d => Ok((Instruction::RR(Operand::L), pc)),
        0x1e => Ok((Instruction::RR(Operand::_HL_), pc)),

        0x3f => Ok((Instruction::SRL(Operand::A), pc)),
        0x38 => Ok((Instruction::SRL(Operand::B), pc)),
        0x39 => Ok((Instruction::SRL(Operand::C), pc)),
        0x3a => Ok((Instruction::SRL(Operand::D), pc)),
        0x3b => Ok((Instruction::SRL(Operand::E), pc)),
        0x3c => Ok((Instruction::SRL(Operand::H), pc)),
        0x3d => Ok((Instruction::SRL(Operand::L), pc)),
        0x3e => Ok((Instruction::SRL(Operand::_HL_), pc)),

        0x37 => Ok((Instruction::SWAP(Operand::A), pc)),
        0x30 => Ok((Instruction::SWAP(Operand::B), pc)),
        0x31 => Ok((Instruction::SWAP(Operand::C), pc)),
        0x32 => Ok((Instruction::SWAP(Operand::D), pc)),
        0x33 => Ok((Instruction::SWAP(Operand::E), pc)),
        0x34 => Ok((Instruction::SWAP(Operand::H), pc)),
        0x35 => Ok((Instruction::SWAP(Operand::L), pc)),
        0x36 => Ok((Instruction::SWAP(Operand::_HL_), pc)),

        _ => {
          panic!("instruction_at: 0xCB instruction not implemented: 0x{:02x}",
                 op)
        }
      }
    } else {
      match op {
        0x8f => Ok((Instruction::ADC(Operand::A, Operand::A), pc)),
        0x88 => Ok((Instruction::ADC(Operand::A, Operand::B), pc)),
        0x89 => Ok((Instruction::ADC(Operand::A, Operand::C), pc)),
        0x8a => Ok((Instruction::ADC(Operand::A, Operand::D), pc)),
        0x8b => Ok((Instruction::ADC(Operand::A, Operand::E), pc)),
        0x8c => Ok((Instruction::ADC(Operand::A, Operand::H), pc)),
        0x8d => Ok((Instruction::ADC(Operand::A, Operand::L), pc)),
        0x8e => Ok((Instruction::ADC(Operand::A, Operand::_HL_), pc)),
        0xce => imm!(Instruction::ADC[Operand::A, imm8], m, addr, pc),

        0x86 => Ok((Instruction::ADD8(Operand::A, Operand::_HL_), pc)),
        0xc6 => imm!(Instruction::ADD8[Operand::A, imm8], m, addr, pc),

        0x09 => Ok((Instruction::ADD16(Operand::HL, Operand::BC), pc)),
        0x19 => Ok((Instruction::ADD16(Operand::HL, Operand::DE), pc)),
        0x29 => Ok((Instruction::ADD16(Operand::HL, Operand::HL), pc)),
        0x39 => Ok((Instruction::ADD16(Operand::HL, Operand::SP), pc)),

        0xa7 => Ok((Instruction::AND(Operand::A), pc)),
        0xa0 => Ok((Instruction::AND(Operand::B), pc)),
        0xa1 => Ok((Instruction::AND(Operand::C), pc)),
        0xa2 => Ok((Instruction::AND(Operand::D), pc)),
        0xa3 => Ok((Instruction::AND(Operand::E), pc)),
        0xa4 => Ok((Instruction::AND(Operand::H), pc)),
        0xa5 => Ok((Instruction::AND(Operand::L), pc)),
        0xa6 => Ok((Instruction::AND(Operand::_HL_), pc)),
        0xe6 => imm!(Instruction::AND[imm8], m, addr, pc),

        0xc4 => imm!(Instruction::CALL_cc[Operand::FlagNZ, imm16], m, addr, pc),
        0xcc => imm!(Instruction::CALL_cc[Operand::FlagZ, imm16], m, addr, pc),
        0xd4 => imm!(Instruction::CALL_cc[Operand::FlagNC, imm16], m, addr, pc),
        0xdc => imm!(Instruction::CALL_cc[Operand::FlagC, imm16], m, addr, pc),
        0xcd => imm!(Instruction::CALL[imm16], m, addr, pc),

        0xbf => Ok((Instruction::CP(Operand::A), pc)),
        0xb8 => Ok((Instruction::CP(Operand::B), pc)),
        0xb9 => Ok((Instruction::CP(Operand::C), pc)),
        0xba => Ok((Instruction::CP(Operand::D), pc)),
        0xbb => Ok((Instruction::CP(Operand::E), pc)),
        0xbc => Ok((Instruction::CP(Operand::H), pc)),
        0xbd => Ok((Instruction::CP(Operand::L), pc)),
        0xbe => Ok((Instruction::CP(Operand::_HL_), pc)),
        0xfe => imm!(Instruction::CP[imm8], m, addr, pc),

        0x3d => Ok((Instruction::DEC8(Operand::A), pc)),
        0x05 => Ok((Instruction::DEC8(Operand::B), pc)),
        0x0d => Ok((Instruction::DEC8(Operand::C), pc)),
        0x15 => Ok((Instruction::DEC8(Operand::D), pc)),
        0x1d => Ok((Instruction::DEC8(Operand::E), pc)),
        0x25 => Ok((Instruction::DEC8(Operand::H), pc)),
        0x2d => Ok((Instruction::DEC8(Operand::L), pc)),
        0x35 => Ok((Instruction::DEC8(Operand::_HL_), pc)),

        0x0b => Ok((Instruction::DEC16(Operand::BC), pc)),
        0x1b => Ok((Instruction::DEC16(Operand::DE), pc)),
        0x2b => Ok((Instruction::DEC16(Operand::HL), pc)),
        0x3b => Ok((Instruction::DEC16(Operand::SP), pc)),

        0xf3 => Ok((Instruction::DI, pc)),

        0xfb => Ok((Instruction::EI, pc)),

        0x76 => Ok((Instruction::HALT, pc)),

        0x04 | 0x0c | 0x14 | 0x1c | 0x24 | 0x2c | 0x3c => {
          let r = op >> 3 & 0b111;
          Ok((Instruction::INC_r(Reg::from(r)), pc))
        }

        0x03 | 0x13 | 0x23 | 0x33 => {
          let ss = op >> 4 & 0b11;
          Ok((Instruction::INC_rr(Reg::from_pair(ss, false)), pc))
        }

        0xe9 => Ok((Instruction::JP_HL, pc)),

        0xc2 | 0xca | 0xd2 | 0xda => {
          let cc = op >> 3 & 0b111;
          let nn = try!(m.read_u16(addr + pc));
          pc += 2;
          Ok((Instruction::JP_cc_nn(Flag::from(cc), nn), pc))
        }

        0xc3 => {
          let nn = try!(m.read_u16(addr + pc));
          pc += 2;
          Ok((Instruction::JP_nn(nn), pc))
        }

        0x20 => {
          let e = try!(m.read_u8(addr + pc));
          pc += 1;
          Ok((Instruction::JR_cc_e(Flag::NZ, e as i8), pc))
        }
        0x28 => {
          let e = try!(m.read_u8(addr + pc));
          pc += 1;
          Ok((Instruction::JR_cc_e(Flag::Z, e as i8), pc))
        }
        0x30 => {
          let e = try!(m.read_u8(addr + pc));
          pc += 1;
          Ok((Instruction::JR_cc_e(Flag::NC, e as i8), pc))
        }
        0x38 => {
          let e = try!(m.read_u8(addr + pc));
          pc += 1;
          Ok((Instruction::JR_cc_e(Flag::C, e as i8), pc))
        }

        0x18 => {
          let e = try!(m.read_u8(addr + pc));
          pc += 1;
          Ok((Instruction::JR_e(e as i8), pc))
        }

        0xe2 => Ok((Instruction::LD_·0xFF00C·_A, pc)),
        0xe0 => {
          let n = try!(m.read_u8(addr + pc));
          pc += 1;
          Ok((Instruction::LD_·0xFF00n·_A(n), pc))
        }

        0x02 => Ok((Instruction::LD_·BC·_A, pc)),
        0x12 => Ok((Instruction::LD_·DE·_A, pc)),

        0x36 => {
          let n = try!(m.read_u8(addr + pc));
          pc += 1;
          Ok((Instruction::LD_·HL·_n(n), pc))
        }

        0x70 | 0x71 | 0x72 | 0x73 | 0x74 | 0x75 | 0x77 => {
          let r = op & 0b111;
          Ok((Instruction::LD_·HL·_r(Reg::from(r)), pc))
        }

        0xea => {
          let nn = try!(m.read_u16(addr + pc));
          pc += 2;
          Ok((Instruction::LD_·nn·_A(nn), pc))
        }

        0x08 => {
          let nn = try!(m.read_u16(addr + pc));
          pc += 2;
          Ok((Instruction::LD_·nn·_SP(nn), pc))
        }

        0x0a => Ok((Instruction::LD_A_·BC·, pc)),

        0x1a => Ok((Instruction::LD_A_·DE·, pc)),

        0xfa => {
          let nn = try!(m.read_u16(addr + pc));
          pc += 2;
          Ok((Instruction::LD_A_·nn·(nn), pc))
        }

        0xf0 => {
          let n = try!(m.read_u8(addr + pc));
          pc += 1;
          Ok((Instruction::LD_A_·0xFF00n·(n), pc))
        }

        0x01 | 0x11 | 0x21 | 0x31 => {
          let r = op >> 4 & 0b11;
          let nn = try!(m.read_u16(addr + pc));
          pc += 2;
          Ok((Instruction::LD_dd_nn(Reg::from_pair(r, false), nn), pc))
        }

        0x46 | 0x4e | 0x56 | 0x5e | 0x66 | 0x6e | 0x7e => {
          let r = op >> 3 & 0b111;
          Ok((Instruction::LD_r_·HL·(Reg::from(r)), pc))
        }

        0x06 | 0x0e | 0x16 | 0x1e | 0x26 | 0x2e | 0x3e => {
          let r = op >> 3 & 0b111;
          let n = try!(m.read_u8(addr + pc));
          pc += 1;
          Ok((Instruction::LD_r_n(Reg::from(r), n), pc))
        }

        0x40 | 0x41 | 0x42 | 0x43 | 0x44 | 0x45 | 0x47 | 0x48 | 0x49 | 0x4a | 0x4b | 0x4c | 0x4d | 0x4f | 0x50 | 0x51 | 0x52 | 0x53 |
        0x54 | 0x55 | 0x57 | 0x58 | 0x59 | 0x5a | 0x5b | 0x5c | 0x5d | 0x5f | 0x60 | 0x61 | 0x62 | 0x63 | 0x64 | 0x65 | 0x67 | 0x68 |
        0x69 | 0x6a | 0x6b | 0x6c | 0x6d | 0x6f | 0x78 | 0x79 | 0x7a | 0x7b | 0x7c | 0x7d | 0x7f => {
          let r1 = op >> 3 & 0b111;
          let r2 = op & 0b111;
          Ok((Instruction::LD_r_r(Reg::from(r1), Reg::from(r2)), pc))
        }

        0x2a => Ok((Instruction::LDI_A_·HL·, pc)),
        0x32 => Ok((Instruction::LDD_·HL·_A, pc)),
        0x22 => Ok((Instruction::LDI_·HL·_A, pc)),

        0xb6 => Ok((Instruction::OR_A_·HL·, pc)),

        0xb0 | 0xb1 | 0xb2 | 0xb3 | 0xb4 | 0xb5 | 0xb7 => {
          let r = op & 0b111;
          Ok((Instruction::OR_r(Reg::from(r)), pc))
        }

        0xc1 | 0xd1 | 0xe1 | 0xf1 => {
          let rr = op >> 4 & 0b11;
          Ok((Instruction::POP_rr(Reg::from_pair(rr, true)), pc))
        }

        0xc5 | 0xd5 | 0xe5 | 0xf5 => {
          let rr = op >> 4 & 0b11;
          Ok((Instruction::PUSH_rr(Reg::from_pair(rr, true)), pc))
        }

        0xc9 => Ok((Instruction::RET, pc)),

        0xc0 | 0xc8 | 0xd0 | 0xd8 => {
          let cc = op >> 3 & 0b111;
          Ok((Instruction::RET_cc(Flag::from(cc)), pc))
        }

        0x17 => Ok((Instruction::RLA, pc)),
        0x07 => Ok((Instruction::RLCA, pc)),

        0x1f => Ok((Instruction::RRA, pc)),

        0xc7 | 0xcf | 0xd7 | 0xdf | 0xe7 | 0xef | 0xf7 | 0xff => {
          let t = op >> 3 & 0b111;
          Ok((Instruction::RST_t(t), pc))
        }

        0xd6 => {
          let n = try!(m.read_u8(addr + pc));
          pc += 1;
          Ok((Instruction::SUB_n(n), pc))
        }

        0x90 | 0x91 | 0x92 | 0x93 | 0x94 | 0x95 | 0x97 => {
          let r = op & 0b111;
          Ok((Instruction::SUB_r(Reg::from(r)), pc))
        }

        0x00 => Ok((Instruction::NOP, pc)),
        0xae => Ok((Instruction::XOR_·HL·, pc)),
        0xee => {
          let n = try!(m.read_u8(addr + pc));
          pc += 1;
          Ok((Instruction::XOR_n(n), pc))
        }
        0xa8 | 0xa9 | 0xaa | 0xab | 0xac | 0xad | 0xaf => {
          let r = op & 0b111;
          Ok((Instruction::XOR_r(Reg::from(r)), pc))
        }

        _ => Ok((Instruction::Invalid(op), pc)),
        // _ => panic!("instruction_at: instruction not implemented: 0x{:02x}", op),
      }
    }
  }
}
