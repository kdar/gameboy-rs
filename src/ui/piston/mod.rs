use piston_window::*;
use im;

use super::Display;

pub struct Win {
  win: Option<PistonWindow>,
  width: u32,
  height: u32,
  buffer: im::ImageBuffer<im::Rgba<u8>, Vec<u8>>,
  doflush: bool,
}

impl Default for Win {
  fn default() -> Win {
    Win {
      win: None,
      width: 256,
      height: 256,
      buffer: im::ImageBuffer::new(256, 256),
      doflush: false,
    }
  }
}

impl Display for Win {
  fn set_pixel(&mut self, x: u32, y: u32, color: [u8; 4]) {
    self.buffer.put_pixel(x, y, im::Rgba(color));
  }

  fn swap(&mut self) {
    self.doflush = true;
  }
}

impl Win {
  pub fn new() -> Win {
    Win {
      win: Some(WindowSettings::new("Gameboy-rs", [256, 256]) // [160, 144])
        .exit_on_esc(true)
        .build()
        .unwrap()),
      ..Win::default()
    }
  }

  pub fn run(&mut self) {
    let mut win = self.win.as_mut().unwrap();
    let image = Image::new().rect([0.0, 0.0, self.width as f64, self.height as f64]);

    // let mut img: im::ImageBuffer<im::Rgba<u8>, Vec<u8>> = im::ImageBuffer::new(200, 200);
    // img.put_pixel(10, 10, im::Rgba([255, 255, 255, 255]));
    // let (w, h) = (160, 144);
    // let mut img: im::ImageBuffer<im::Rgba<u8>, Vec<u8>> = im::ImageBuffer::new(w, h);
    // for x in 0..w {
    //  for y in 0..h {
    //    img.put_pixel(x, y, im::Rgba([x as u8, x as u8, x as u8, 255]));
    //  }
    // }

    let mut texture = Texture::from_image(&mut win.factory, &self.buffer, &TextureSettings::new()).unwrap();

    let mut frame_count = 0;
    // let mut start = SteadyTime::now();
    while let Some(e) = win.next() {
      frame_count += 1;

      // if SteadyTime::now() - start >= Duration::seconds(1) {
      //  println!("fps: {}", frame_count);
      //  frame_count = 0;
      //  start = SteadyTime::now();
      // }

      match e {
        Event::Render(_) => {
          if self.doflush {
            texture.update(&mut win.encoder, &self.buffer).unwrap();
            win.draw_2d(&e, |c, g| {
              clear([0.0; 4], g);
              image.draw(&texture, &draw_state::DrawState::default(), c.transform, g);
            });
            self.doflush = false;
          }
        }
        Event::Update(_) => {
          println!("update");
        }
        _ => {}
      };
    }
  }
}
