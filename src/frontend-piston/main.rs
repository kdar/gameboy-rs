#![feature(non_ascii_idents)]

extern crate gameboy;
extern crate clap;
#[macro_use]
extern crate log;
extern crate simplelog;
extern crate piston_window;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use clap::{Arg, App};
use simplelog::{TermLogger, LogLevelFilter};
use std::process::exit;
use std::sync::mpsc;
use std::thread;
use piston_window::*;
use std::time::{Duration, Instant};

use gameboy::video::{self, Pixels};
use gameboy::gamepad::Button as GButton;
use gameboy::disassembler;
use gameboy::cpu::Cpu;
use gameboy::system;
use gameboy::debugger;

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
      // thread::spawn(move || {
      //   // use std::time::{Instant, Duration};
      //   // let mut hz = 0;
      //   // let mut now = Instant::now();
      //   loop {
      //     // let n = Instant::now();
      //     cpu.step();
      //     // hz += 1;
      //     // if Instant::now().duration_since(now).as_secs() >= 1 {
      //     //  println!("{} hz", hz);
      //     //  hz = 0;
      //     //  now = Instant::now();
      //     // }
      //     // println!("{:?}", n.elapsed());
      //   }
      // });
    }
  }
}

fn run(mut cpu: Cpu) {
  let mut win: PistonWindow = WindowSettings::new("Gameboy-rs",
                                                  [((video::SCREEN_WIDTH as f64) * 4f64) as u32,
                                                   ((video::SCREEN_HEIGHT as f64) * 4f64) as u32])
    .exit_on_esc(true)
    .build()
    .unwrap();

  let one_sec = Duration::from_secs(1);
  let cpu_time = Duration::new(0, 1000000000 / 60);

  win.set_max_fps(60);

  use std::sync::mpsc;
  let (sender, receiver) = mpsc::channel();
  thread::spawn(move || {
    loop {
      cpu.step();
      if let Some(f) = cpu.updated_frame() {
        sender.send(f.to_vec()).unwrap();
      }
    }
  });

  let mut frame_count = 0;
  let mut start = Instant::now();
  let mut start2 = Instant::now();
  loop {
    // frame_count += 1;

    // if now - start >= one_sec {
    //   win.set_title(format!("Gameboy-rs: {} fps", frame_count));
    //   frame_count = 0;
    //   start = Instant::now();
    // }

    if let Some(e) = win.next() {
      if let Some(button) = e.press_args() {
        // match button {
        //   Button::Keyboard(Key::A) |
        //   Button::Keyboard(Key::Left) => {
        //     cpu.set_button(GButton::Left, true);
        //   }
        //   Button::Keyboard(Key::S) |
        //   Button::Keyboard(Key::Down) => {
        //     cpu.set_button(GButton::Down, true);
        //   }
        //   Button::Keyboard(Key::D) |
        //   Button::Keyboard(Key::Right) => {
        //     cpu.set_button(GButton::Right, true);
        //   }
        //   Button::Keyboard(Key::W) |
        //   Button::Keyboard(Key::Up) => {
        //     cpu.set_button(GButton::Up, true);
        //   }
        //   _ => (),
        // };
      }

      // if let Some(button) = e.release_args() {
      //   match button {
      //     Button::Keyboard(Key::A) |
      //     Button::Keyboard(Key::Left) => {
      //       cpu.set_button(GButton::Left, false);
      //     }
      //     Button::Keyboard(Key::S) |
      //     Button::Keyboard(Key::Down) => {
      //       cpu.set_button(GButton::Down, false);
      //     }
      //     Button::Keyboard(Key::D) |
      //     Button::Keyboard(Key::Right) => {
      //       cpu.set_button(GButton::Right, false);
      //     }
      //     Button::Keyboard(Key::W) |
      //     Button::Keyboard(Key::Up) => {
      //       cpu.set_button(GButton::Up, false);
      //     }
      //     _ => (),
      //   };
      // }

      if let Event::Render(rargs) = e {
        frame_count += 1;

        if Instant::now() - start2 >= Duration::from_secs(1) {
          win.set_title(format!("Gameboy-rs: {} fps", frame_count));
          frame_count = 0;
          start2 = Instant::now();
        }

        if let Ok(pixels) = receiver.try_recv() {
          win.draw_2d(&e, |c, g| {
            clear([0.0; 4], g);

            let scale_x = rargs.draw_width as f64 / video::SCREEN_WIDTH as f64;
            let scale_y = rargs.draw_height as f64 / video::SCREEN_HEIGHT as f64;

            for (i, d) in pixels.iter().enumerate() {
              let x = i % video::SCREEN_WIDTH as usize;
              let y = (i - x) / video::SCREEN_WIDTH as usize;
              Rectangle::new([d[0] as f32 / 255.0,
                              d[1] as f32 / 255.0,
                              d[2] as f32 / 255.0,
                              d[3] as f32 / 255.0])
                .draw([x as f64 * scale_x, y as f64 * scale_y, scale_x, scale_y],
                      &c.draw_state,
                      c.transform,
                      g);
            }
          });
        }
      }
    }
  }
}

fn load_rom<P: AsRef<Path>>(path: P) -> Box<[u8]> {
  let mut file = try_log!(File::open(path));
  let mut file_buf = Vec::new();
  try_log!(file.read_to_end(&mut file_buf));
  file_buf.into_boxed_slice()
}
