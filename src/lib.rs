#![feature(non_ascii_idents)]
#![feature(custom_derive, plugin)]
#![plugin(num_macros)]
#![feature(step_by)]

#[cfg(test)]
extern crate difference;
#[cfg(test)]
extern crate yaml_rust;

extern crate md5;
#[macro_use]
extern crate nom;
extern crate piston_window;
extern crate image as im;
extern crate time;
extern crate num;
extern crate libc;

pub mod mem;
pub mod cpu;
pub mod reg;
pub mod flag;
pub mod disassembler;
pub mod debugger;
pub mod video;
pub mod audio;
pub mod cartridge;
pub mod bit;
pub mod linkport;
pub mod operand;
pub mod bios;
pub mod system;

pub use reg::Reg;
pub use flag::Flag;

#[cfg(test)]
mod test {
  use super::mem;
  use super::disassembler::Disassembler;
  use super::disassembler::Instruction;

  #[test]
  #[ignore]
  fn test_unimplemented() {
    let mut m: Box<mem::Memory> = Box::new(mem::Mem::new());
    let d = Disassembler::new();
    for i in 0..(0xFF as usize) + 1 {
      m.write_byte(0, i as u8).unwrap();
      match d.at(&m, 0) {
        Ok((Instruction::Invalid(opcode), _)) => {
          println!("{:#02x}", opcode);
        }
        _ => (),
      };
    }

    for i in 0..(0xFF as usize) + 1 {
      m.write_byte(0, i as u8).unwrap();
      match d.at(&m, 0) {
        Ok((Instruction::Invalid(opcode), _)) => {
          println!("{:#02x}", opcode);
        }
        _ => (),
      };
    }
  }
}
