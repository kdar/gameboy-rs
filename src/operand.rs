use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Operand {
  RegA,
  RegF,
  RegB,
  RegC,
  RegD,
  RegE,
  RegH,
  RegL,
  RegBC,
  RegDE,
  RegHL,
  RegAF,
  RegSP,
  RegPC,

  AddrBC,
  AddrDE,
  AddrHL,
  AddrSP,
  AddrImm16(u16),
  AddrIoPortC,

  FlagZ, // zero flag
  FlagN, // add/sub flag
  FlagH, // half carry flag
  FlagC, // carry flag
  FlagNZ, // non-zero (uses zero flag)
  FlagNC, // non-carry (uses carry flag)

  Imm8(u8),
  Imm16(u16),
}

impl fmt::Display for Operand {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Operand::RegA => write!(f, "A"),
      Operand::RegF => write!(f, "F"),
      Operand::RegB => write!(f, "B"),
      Operand::RegC => write!(f, "C"),
      Operand::RegD => write!(f, "D"),
      Operand::RegE => write!(f, "E"),
      Operand::RegH => write!(f, "H"),
      Operand::RegL => write!(f, "L"),
      Operand::RegBC => write!(f, "BC"),
      Operand::RegDE => write!(f, "DE"),
      Operand::RegHL => write!(f, "HL"),
      Operand::RegAF => write!(f, "AF"),
      Operand::RegSP => write!(f, "SP"),
      Operand::RegPC => write!(f, "PC"),
      Operand::FlagZ => write!(f, "Z"),
      Operand::FlagN => write!(f, "N"),
      Operand::FlagH => write!(f, "H"),
      Operand::FlagC => write!(f, "C"),
      Operand::FlagNZ => write!(f, "NZ"),
      Operand::FlagNC => write!(f, "NC"),
      Operand::AddrBC => write!(f, "(BC)"),
      Operand::AddrDE => write!(f, "(DE)"),
      Operand::AddrHL => write!(f, "(HL)"),
      Operand::AddrSP => write!(f, "(SP)"),
      Operand::AddrImm16(i) => write!(f, "(${:04x})", i),
      Operand::AddrIoPortC => write!(f, "($ff00+C)"),
      Operand::Imm8(i) => write!(f, "${:02x}", i),
      Operand::Imm16(i) => write!(f, "${:04x}", i),
    }
  }
}
