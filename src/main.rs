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
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use gameboy::debugger;
use gameboy::disassembler;
use gameboy::cpu::Cpu;
use gameboy::system;
use gameboy::ui::Display;
use gameboy::ui::piston::Win;

macro_rules! try_log {
  ($expr:expr) => (match $expr {
    std::result::Result::Ok(val) => val,
    std::result::Result::Err(err) => {
      error!("{}", err.to_string());
      exit(1);
    }
  })
}

struct ProxyDisplay {
  sender: Sender<ProxyEvent>,
}

enum ProxyEvent {
  SetPixel(u32, u32, [u8; 4]),
  Swap,
}

impl Display for ProxyDisplay {
  fn set_pixel(&mut self, x: u32, y: u32, color: [u8; 4]) {
    self.sender.send(ProxyEvent::SetPixel(x, y, color)).unwrap();
  }
  fn swap(&mut self) {
    self.sender.send(ProxyEvent::Swap).unwrap();
  }
}

fn main() {
  TermLogger::init(LogLevelFilter::Info).unwrap();

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
    disassembler::dump_all(cart_rom);
  } else {
    let mut bootstrap = false;
    let boot_rom = if let Some(boot_rom_path) = matches.value_of("boot-rom") {
      Some(load_rom(boot_rom_path))
    } else {
      bootstrap = true;
      None
    };

    let (sender, receiver) = mpsc::channel();

    let mut display = Win::new();
    let proxy_display = Box::new(ProxyDisplay { sender: sender });
    let system = try_log!(system::Config::new()
      .boot_rom(boot_rom)
      .cart_rom(cart_rom)
      .display(proxy_display)
      .create());
    let mut cpu = Cpu::new(system);
    if bootstrap {
      cpu.bootstrap();
    }

    if matches.is_present("debug") {
      let mut gb = debugger::Debugger::new(cpu);
      gb.run();
    } else {
      let t = thread::spawn(move || run_main(cpu));

      loop {
        match receiver.recv().unwrap() {
          ProxyEvent::SetPixel(x, y, color) => {
            display.set_pixel(x, y, color);
          }
          ProxyEvent::Swap => {
            display.swap();
          }
        }
      }

      t.join().unwrap();
    }
  }
}

fn run_main(mut cpu: Cpu) {
  loop {
    cpu.step();
  }
}

fn load_rom<P: AsRef<Path>>(path: P) -> Box<[u8]> {
  let mut file = try_log!(File::open(path));
  let mut file_buf = Vec::new();
  try_log!(file.read_to_end(&mut file_buf));
  file_buf.into_boxed_slice()
}
