#![feature(non_ascii_idents)]

extern crate gameboy;
extern crate clap;
#[macro_use]
extern crate log;
extern crate simplelog;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use clap::{Arg, App};
use simplelog::{TermLogger, LogLevelFilter};
use std::process::exit;

use gameboy::debugger;
use gameboy::disassembler;
use gameboy::mem;
use gameboy::video;

macro_rules! try_log {
  ($expr:expr) => (match $expr {
    std::result::Result::Ok(val) => val,
    std::result::Result::Err(err) => {
      error!("{}", err.to_string());
      exit(1);
    }
  })
}

fn main() {
  TermLogger::init(LogLevelFilter::Info).unwrap();

  // let v = video::Video::new();
  // v.run();

  let matches = App::new("gameboy-rs")
    .version("0.1.0")
    .author("Kevin Darlington <kevin@outroot.com>")
    .about("Emulates GameBoy")
    .arg(Arg::with_name("cart-rom")
      .help("The cartridge rom to load.")
      .value_name("FILE")
      .use_delimiter(false)
      .required(true)
      .index(1))
    .arg(Arg::with_name("debug")
      .long("debug")
      .use_delimiter(false)
      .help("Go into debug mode"))
    .arg(Arg::with_name("disassemble")
      .long("disassemble")
      .use_delimiter(false)
      .help("Disassemble the file"))
    .arg(Arg::with_name("boot-rom")
      .short("b")
      .long("boot-rom")
      .use_delimiter(false)
      .value_name("FILE")
      .help("The boot rom to load.")
      .takes_value(true))
    .get_matches();

  let cart_rom = load_rom(matches.value_of("cart-rom").unwrap());

  if matches.is_present("disassemble") {
    let mut m: Box<mem::Memory> = Box::new(mem::Mem::new());
    m.set_boot_rom(cart_rom);
    let d = disassembler::Disassembler::new();
    d.print_all(&m);
  } else if matches.is_present("debug") {
    let mut gb = debugger::Debugger::new();
    gb.set_cart_rom(cart_rom);
    if let Some(boot_rom_path) = matches.value_of("boot-rom") {
      gb.set_boot_rom(load_rom(boot_rom_path));
    }

    gb.run();
  } else {
    let mut gb = gameboy::GameBoy::new(cart_rom);
    if let Some(boot_rom_path) = matches.value_of("boot-rom") {
      gb.set_boot_rom(load_rom(boot_rom_path));
    }

    gb.run();
  }
}

fn load_rom<P: AsRef<Path>>(path: P) -> Box<[u8]> {
  let mut file = try_log!(File::open(path));
  let mut file_buf = Vec::new();
  try_log!(file.read_to_end(&mut file_buf));
  file_buf.into_boxed_slice()
}
