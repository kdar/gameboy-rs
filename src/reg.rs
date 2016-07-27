#[derive(Debug)]
pub enum Reg {
  B = 0b000,
  C = 0b001,
  D = 0b010,
  E = 0b011,
  H = 0b100,
  L = 0b101,
  F = 0b110,
  A = 0b111, /* BC,
              * DE,
              * HL,
              * SP, */
}

impl From<u8> for Reg {
  fn from(v: u8) -> Reg {
    match v {
      0b000 => Reg::B,
      0b001 => Reg::C,
      0b010 => Reg::D,
      0b011 => Reg::E,
      0b100 => Reg::H,
      0b101 => Reg::L,
      0b110 => Reg::F,
      0b111 => Reg::A,
      _ => panic!("reg.from_raw_byte unknown register: {}", v),
    }
  }
}

// impl Reg {
//   pub fn from_byte(r: u8) -> Reg {
//     match r {
//       0b000 => Reg::B,
//       0b001 => Reg::C,
//       0b010 => Reg::D,
//       0b011 => Reg::E,
//       0b100 => Reg::H,
//       0b101 => Reg::L,
//       0b110 => Reg::F,
//       0b111 => Reg::A,
//       _ => panic!("reg.from_raw_byte unknown register: {}", r),
//     }
//   }
//
//   pub fn from_word(r: u16) -> Reg {
//     match r {
//       0b00 => Reg::BC,
//       0b01 => Reg::DE,
//       0b10 => Reg::HL,
//       0b11 => Reg::SP,
//       _ => panic!("reg.from_raw_byte unknown register: {}", r),
//     }
//   }
// }
