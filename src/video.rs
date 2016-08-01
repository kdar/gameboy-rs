use piston_window::*;
use im;
use time::{Duration, SteadyTime};
use super::mem::{self, MemoryMap};

const LCD_CONTROL: u16 = 0xFF40;
const LCD_CONTROLLER_STATUS: u16 = 0xFF41;
const SCROLL_Y: u16 = 0xFF42;
const SCROLL_X: u16 = 0xFF43;
const LCD_CONTROLLER_Y_COORDINATE: u16 = 0xFF44;
const LY_COMPARE: u16 = 0xFF45;
const WINDOW_Y_POSITION: u16 = 0xFF4A;
const WINDOW_X_POSITION_MINUS_7: u16 = 0xFF4B;
const BG_PALETTE_DATA: u16 = 0xFF47;
const OBJECT_PALETTE0_DATA: u16 = 0xFF48;
const OBJECT_PALETTE1_DATA: u16 = 0xFF49;
const DMA_TRANSFER_AND_START_ADDRESS: u16 = 0xFF46;

pub struct Video {
  window: Option<PistonWindow>,
  // tilemap: [[u8; ]],
  bg_palette: [u8; 4],
  obj_palette0: [u8; 4],
  obj_palette1: [u8; 4],
  current_line: u8,
}

impl MemoryMap for Video {
  fn read_byte(&self, addr: u16) -> Option<u8> {
    println!("reading vid byte from: {:#04x}", addr);
    match addr {
      0xFF44 => Some(0x90), // Some(self.current_line),
      _ => Some(0),
    }
  }

  fn write_byte(&mut self, addr: u16, value: u8) {
    match addr {
      BG_PALETTE_DATA => {
        println!("video: bg palette: {:#04x}", addr);
        for i in 0..4 {
          match (value >> (i * 2)) & 3 {
            0 => self.bg_palette[i] = 255,
            1 => self.bg_palette[i] = 192,
            2 => self.bg_palette[i] = 96,
            3 => self.bg_palette[i] = 0,
            _ => (),
          }
        }
      }
      OBJECT_PALETTE0_DATA => {
        println!("video: obj 0 palette: {:#04x}", addr);
        for i in 0..4 {
          match (value >> (i * 2)) & 3 {
            0 => self.obj_palette0[i] = 255,
            1 => self.obj_palette0[i] = 192,
            2 => self.obj_palette0[i] = 96,
            3 => self.obj_palette0[i] = 0,
            _ => (),
          }
        }
      }
      OBJECT_PALETTE1_DATA => {
        println!("video: obj 1 palette: {:#04x}", addr);
        for i in 0..4 {
          match (value >> (i * 2)) & 3 {
            0 => self.obj_palette1[i] = 255,
            1 => self.obj_palette1[i] = 192,
            2 => self.obj_palette1[i] = 96,
            3 => self.obj_palette1[i] = 0,
            _ => (),
          }
        }
      }
      _ => (), // println!("video: non implemented range: {:#04x}", addr),
    };
  }
}

impl Default for Video {
  fn default() -> Video {
    Video {
      window: None,
      bg_palette: [0; 4],
      obj_palette0: [0; 4],
      obj_palette1: [0; 4],
      current_line: 0,
    }
  }
}

impl Video {
  pub fn new() -> Video {
    Video {
      window: Some(WindowSettings::new("Gameboy-rs", [160, 144])
        .exit_on_esc(true)
        .build()
        .unwrap()),
      ..Video::default()
    }
  }

  pub fn step() {}

  // pub fn run(&mut self) {
  //   let image = Image::new().rect([0.0, 0.0, 160.0, 144.0]);
  //
  //   // let mut img: im::ImageBuffer<im::Rgba<u8>, Vec<u8>> = im::ImageBuffer::new(200, 200);
  //   // img.put_pixel(10, 10, im::Rgba([255, 255, 255, 255]));
  //   let (w, h) = (160, 144);
  //   let mut img: im::ImageBuffer<im::Rgba<u8>, Vec<u8>> = im::ImageBuffer::new(w, h);
  //   for x in 0..w {
  //     for y in 0..h {
  //       img.put_pixel(x, y, im::Rgba([x as u8, x as u8, x as u8, 255]));
  //     }
  //   }
  //
  //   let texture = Texture::from_image(&mut self.window.factory, &img, &TextureSettings::new())
  //     .unwrap();
  //
  //   let mut frame_count = 0;
  //   let mut start = SteadyTime::now();
  //   while let Some(e) = self.window.next() {
  //     frame_count += 1;
  //
  //     if SteadyTime::now() - start >= Duration::seconds(1) {
  //       println!("fps: {}", frame_count);
  //       frame_count = 0;
  //       start = SteadyTime::now();
  //     }
  //
  //     self.window.draw_2d(&e, |c, g| {
  //       clear([0.0; 4], g);
  //
  //       image.draw(&texture, &draw_state::DrawState::default(), c.transform, g);
  //     });
  //   }
  // }
}