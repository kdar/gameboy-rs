use piston_window::*;
use im;
use std::f64;
use time::{Duration, SteadyTime};

pub struct Video;

impl Video {
  pub fn new() -> Video {
    Video
  }

  pub fn run(&self) {
    let mut window: PistonWindow = WindowSettings::new("Gameboy-rs", [640, 480])
      .exit_on_esc(true)
      .build()
      .unwrap();

    let image = Image::new().rect([0.0, 0.0, 200.0, 200.0]);

    // let mut img: im::ImageBuffer<im::Rgba<u8>, Vec<u8>> = im::ImageBuffer::new(200, 200);
    // img.put_pixel(10, 10, im::Rgba([255, 255, 255, 255]));
    let (w, h) = (280, 240);
    let mut img: im::ImageBuffer<im::Rgba<u8>, Vec<u8>> = im::ImageBuffer::new(w, h);
    for x in 0..w {
      for y in 0..h {
        img.put_pixel(x, y, im::Rgba([x as u8, x as u8, x as u8, 255]));
      }
    }

    let texture = Texture::from_image(&mut window.factory, &img, &TextureSettings::new()).unwrap();

    let mut frame_count = 0;
    let mut start = SteadyTime::now();
    while let Some(e) = window.next() {
      frame_count += 1;

      if SteadyTime::now() - start >= Duration::seconds(1) {
        println!("fps: {}", frame_count);
        frame_count = 0;
        start = SteadyTime::now();
      }

      window.draw_2d(&e, |c, g| {
        clear([0.0; 4], g);

        image.draw(&texture, &draw_state::DrawState::default(), c.transform, g);
      });
    }
  }
}
