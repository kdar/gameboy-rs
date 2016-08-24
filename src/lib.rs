#![feature(non_ascii_idents)]
#![feature(custom_derive, plugin)]
#![plugin(num_macros)]
#![feature(concat_idents)]

#[cfg(test)]
extern crate difference;
#[cfg(test)]
extern crate yaml_rust;

extern crate md5;
extern crate piston_window;
extern crate image as im;
extern crate time;
extern crate num;
extern crate libc;
extern crate rand;
#[macro_use]
extern crate bitflags;
extern crate ctrlc;

extern crate clap;
extern crate term_grid;
extern crate terminal_size;

#[macro_use]
pub mod macros;
pub mod mem;
pub mod cpu;
pub mod disassembler;
pub mod debugger;
pub mod video;
pub mod audio;
pub mod cartridge;
pub mod linkport;
pub mod operand;
pub mod bios;
pub mod system;
pub mod ui;
pub mod pic;
pub mod timer;

pub enum GbEvent {
  Frame(Vec<[u8; 4]>),
}

#[cfg(test)]
mod test {
  use super::mem;
  use super::system::{SystemCtrl, System};
  use super::disassembler::Disassembler;
  use super::disassembler::Instruction;

  #[test]
  #[ignore]
  fn test_unimplemented() {
    let mut s: Box<SystemCtrl> = Box::new(System::new());
    let d = Disassembler::new();
    for i in 0..(0xFF as usize) + 1 {
      s.write_u8(0, i as u8).unwrap();
      match d.at(s.as_memoryio(), 0) {
        Ok((Instruction::Invalid(opcode), _)) => {
          println!("{:#02x}", opcode);
        }
        _ => (),
      };
    }

    for i in 0..(0xFF as usize) + 1 {
      s.write_u8(0, i as u8).unwrap();
      match d.at(s.as_memoryio(), 0) {
        Ok((Instruction::Invalid(opcode), _)) => {
          println!("{:#02x}", opcode);
        }
        _ => (),
      };
    }
  }
}
