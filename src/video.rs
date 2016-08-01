use piston_window::*;
use im;
use time::{Duration, SteadyTime};
use super::mem::MemoryMap;

pub struct Video {
  window: PistonWindow,
}

impl MemoryMap for Video {
  fn read_byte(&self, addr: u16) -> Option<u8> {
    println!("reading vid byte from: {}", addr);
    Some(0)
  }

  fn write_byte(&mut self, addr: u16, value: u8) {
    println!("write vid byte to: {}", addr);
  }
}

impl Video {
  pub fn new() -> Video {
    Video {
      window: WindowSettings::new("Gameboy-rs", [160, 144])
        .exit_on_esc(true)
        .build()
        .unwrap(),
    }
  }

  pub fn run(&mut self) {
    let image = Image::new().rect([0.0, 0.0, 160.0, 144.0]);

    // let mut img: im::ImageBuffer<im::Rgba<u8>, Vec<u8>> = im::ImageBuffer::new(200, 200);
    // img.put_pixel(10, 10, im::Rgba([255, 255, 255, 255]));
    let (w, h) = (160, 144);
    let mut img: im::ImageBuffer<im::Rgba<u8>, Vec<u8>> = im::ImageBuffer::new(w, h);
    for x in 0..w {
      for y in 0..h {
        img.put_pixel(x, y, im::Rgba([x as u8, x as u8, x as u8, 255]));
      }
    }

    let texture = Texture::from_image(&mut self.window.factory, &img, &TextureSettings::new())
      .unwrap();

    let mut frame_count = 0;
    let mut start = SteadyTime::now();
    while let Some(e) = self.window.next() {
      frame_count += 1;

      if SteadyTime::now() - start >= Duration::seconds(1) {
        println!("fps: {}", frame_count);
        frame_count = 0;
        start = SteadyTime::now();
      }

      self.window.draw_2d(&e, |c, g| {
        clear([0.0; 4], g);

        image.draw(&texture, &draw_state::DrawState::default(), c.transform, g);
      });
    }
  }
}
