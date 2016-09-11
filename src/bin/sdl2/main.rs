extern crate sdl2;
extern crate gameboy;
extern crate clap;
#[macro_use]
extern crate log;
extern crate simplelog;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::{Duration, Instant};
use std::process::exit;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use clap::{Arg, App};
use simplelog::{TermLogger, LogLevelFilter};

use gameboy::cpu::Cpu;
use gameboy::system;
use gameboy::gamepad::Button;
use gameboy::debugger;
use gameboy::disassembler;

macro_rules! try_log {
  ($expr:expr) => (match $expr {
    std::result::Result::Ok(val) => val,
    std::result::Result::Err(err) => {
      error!("{}", err.to_string());
      exit(1);
    }
  })
}

fn load_rom<P: AsRef<Path>>(path: P) -> Box<[u8]> {
  let mut file = try_log!(File::open(path));
  let mut file_buf = Vec::new();
  try_log!(file.read_to_end(&mut file_buf));
  file_buf.into_boxed_slice()
}

fn main() {
  TermLogger::init(LogLevelFilter::Info).unwrap();

  let matches = App::new("gameboy-rs")
    .version("0.1.0")
    .author("Kevin Darlington <kevin@outroot.com>")
    .about("Emulates GameBoy DMG")
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

    let system = try_log!(system::Config::new()
      .boot_rom(boot_rom)
      .cart_rom(cart_rom)
      .create());
    let mut cpu = Cpu::new(system);

    if bootstrap {
      cpu.bootstrap();
    }

    if matches.is_present("debug") {
      // TODO: this doesn't work with the UI just yet.
      let mut gb = debugger::Debugger::new(cpu);
      gb.run();
      exit(0);
    } else {
      run(cpu);
    }
  }
}

fn run(mut cpu: Cpu) {
  let scale = 4.0f64;

  let sdl_context = try_log!(sdl2::init());
  let video_subsystem = try_log!(sdl_context.video());

  let window = try_log!(video_subsystem.window("Gameboy-rs", 160 * scale as u32, 144 * scale as u32)
    .position_centered()
    .resizable()
    .opengl()
    .build());

  let mut renderer = try_log!(window.renderer().build());
  //   renderer.set_scale(scale, scale);
  let mut texture =
    try_log!(renderer.create_texture_streaming(PixelFormatEnum::RGBA8888, 160, 144));

  let mut event_pump = try_log!(sdl_context.event_pump());

  let mut frame_count = 0;
  let mut start = Instant::now();
  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. } => break 'running,
        Event::KeyDown { keycode: Some(keycode), .. } => {
          match keycode {
            Keycode::Escape => break 'running,
            Keycode::A | Keycode::Left => cpu.set_button(Button::Left, true),
            Keycode::W | Keycode::Up => cpu.set_button(Button::Up, true),
            Keycode::S | Keycode::Down => cpu.set_button(Button::Down, true),
            Keycode::D | Keycode::Right => cpu.set_button(Button::Right, true),
            Keycode::Return => cpu.set_button(Button::Start, true),
            Keycode::RShift => cpu.set_button(Button::Select, true),
            Keycode::Space => cpu.set_button(Button::A, true),
            Keycode::LCtrl => cpu.set_button(Button::B, true),
            _ => {}
          };
        }
        Event::KeyUp { keycode: Some(keycode), .. } => {
          match keycode {
            Keycode::A | Keycode::Left => cpu.set_button(Button::Left, false),
            Keycode::W | Keycode::Up => cpu.set_button(Button::Up, false),
            Keycode::S | Keycode::Down => cpu.set_button(Button::Down, false),
            Keycode::D | Keycode::Right => cpu.set_button(Button::Right, false),
            Keycode::Return => cpu.set_button(Button::Start, false),
            Keycode::RShift => cpu.set_button(Button::Select, false),
            Keycode::Space => cpu.set_button(Button::A, false),
            Keycode::LCtrl => cpu.set_button(Button::B, false),
            _ => {}
          };
        }    
        _ => {}
      };
    }

    cpu.step();

    if let Some(pixels) = cpu.updated_frame() {
      frame_count += 1;

      try_log!(texture.with_lock(None, |buffer: &mut [u8], _: usize| {
        for (i, d) in pixels.iter().enumerate() {
          buffer[i * 4] = d[3];
          buffer[i * 4 + 1] = d[2];
          buffer[i * 4 + 2] = d[1];
          buffer[i * 4 + 3] = d[0];
        }
      }));

      renderer.set_draw_color(Color::RGB(255, 255, 255));
      renderer.clear();
      let size = {
        let mut window = renderer.window_mut().unwrap();
        if Instant::now() - start >= Duration::from_secs(1) {
          try_log!(window.set_title(format!("Gameboy-rs: {} fps", frame_count).as_str()));
          frame_count = 0;
          start = Instant::now();
        }
        window.size()
      };
      renderer.copy(&texture, None, Some(Rect::new(0, 0, size.0, size.1)));
      renderer.present();
    }
  }
}