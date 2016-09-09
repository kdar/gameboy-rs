extern crate sdl2;
extern crate gameboy;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::{Rect, Point};
use std::time::{Duration, Instant};

use gameboy::cpu::Cpu;
use gameboy::system;
use gameboy::video::{self, Pixels};
use gameboy::gamepad::Button;

fn load_rom<P: AsRef<Path>>(path: P) -> Box<[u8]> {
  let mut file = File::open(path).unwrap();
  let mut file_buf = Vec::new();
  file.read_to_end(&mut file_buf).unwrap();
  file_buf.into_boxed_slice()
}

fn main() {
  let system = system::Config::new()
    .cart_rom(load_rom("res/Tetris.gb"))
    .create()
    .unwrap();
  let mut cpu = Cpu::new(system);
  cpu.bootstrap();

  let scale = 4.0;

  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();

  let mut window = video_subsystem.window("Gameboy-rs", 160 * scale as u32, 144 * scale as u32)
    .position_centered()
    .opengl()
    .build()
    .unwrap();

  let mut renderer = window.renderer().build().unwrap();
  //   renderer.set_scale(scale, scale);
  let mut texture = renderer.create_texture_streaming(PixelFormatEnum::RGBA8888, 160, 144)
    .unwrap();

  let mut event_pump = sdl_context.event_pump().unwrap();

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

      if Instant::now() - start >= Duration::from_secs(1) {
        // window.set_title(format!("Gameboy-rs: {} fps", frame_count).as_str());
        println!("Gameboy-rs: {} fps", frame_count);
        frame_count = 0;
        start = Instant::now();
      }

      texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
          for (i, d) in pixels.iter().enumerate() {
            buffer[i * 4] = d[3];
            buffer[i * 4 + 1] = d[2];
            buffer[i * 4 + 2] = d[1];
            buffer[i * 4 + 3] = d[0];
          }
        })
        .unwrap();

      renderer.set_draw_color(Color::RGB(255, 255, 255));
      renderer.clear();
      renderer.copy(&texture,
                    None,
                    Some(Rect::new(0, 0, 160 * scale as u32, 144 * scale as u32)));
      renderer.present();
    }
  }
}