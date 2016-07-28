#![feature(non_ascii_idents)]

extern crate md5;
#[cfg(test)]
extern crate difference;

mod gameboy;
pub mod cpu;
mod mem;
mod instruction;
mod reg;
mod flag;

pub use gameboy::GameBoy;
