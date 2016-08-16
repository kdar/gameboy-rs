use piston_window::*;
use im;
use time::{Duration, SteadyTime};
use std::sync::mpsc::{self, Receiver};
use super::GbEvent;

pub struct Ui {
  win: Option<PistonWindow>,
  scale: f64,
  width: u32,
  height: u32,
  buffer: im::ImageBuffer<im::Rgba<u8>, Vec<u8>>,
  doflush: bool,
  event_receiver: Receiver<GbEvent>,
}

impl Default for Ui {
  fn default() -> Ui {
    let (_, null_receiver) = mpsc::channel();
    Ui {
      win: None,
      scale: 2.0,
      width: 160,
      height: 144,
      buffer: im::ImageBuffer::new(160, 144),
      doflush: false,
      event_receiver: null_receiver,
    }
  }
}

impl Ui {
  pub fn new(r: Receiver<GbEvent>) -> Ui {
    let mut u = Ui { event_receiver: r, ..Ui::default() };

    u.win = Some(WindowSettings::new("Gameboy-rs",
                                     [((u.width as f64) * u.scale) as u32, ((u.height as f64) * u.scale) as u32])
      .exit_on_esc(true)
      .build()
      .unwrap());
    u
  }

  pub fn run(&mut self) {
    let mut win = self.win.as_mut().unwrap();
    let image = Image::new().rect([0.0, 0.0, (self.width as f64) * self.scale, (self.height as f64) * self.scale]);

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
    let mut start = SteadyTime::now();
    while let Some(e) = win.next() {
      frame_count += 1;

      if let Ok(e) = self.event_receiver.try_recv() {
        match e {
          GbEvent::Frame(d) => {
            for (x, y, pixel) in self.buffer.enumerate_pixels_mut() {
              let v = d[(y as usize) * (self.width as usize) + (x as usize)];
              let (r, g, b, a) = ((v >> 24 & 0xff) as u8, (v >> 16 & 0xff) as u8, (v >> 8 & 0xff) as u8, (v & 0xff) as u8);
              *pixel = im::Rgba([r, g, b, a])
            }
            // im::imageops::resize(&self.buffer,
            //                     ((self.width as f64) * self.scale) as u32,
            //                     ((self.height as f64) * self.scale) as u32,
            //                     im::FilterType::CatmullRom);
            self.doflush = true;
          }
        }
      }

      if SteadyTime::now() - start >= Duration::seconds(1) {
        win.set_title(format!("Gameboy-rs: {} fps", frame_count));
        frame_count = 0;
        start = SteadyTime::now();
      }

      match e {
        Event::Render(_) => {
          if self.doflush {
            texture.update(&mut win.encoder, &self.buffer).unwrap();
            let scale = self.scale;
            win.draw_2d(&e, |c, g| {
              clear([0.0; 4], g);
              image.draw(&texture,
                         &draw_state::DrawState::default(),
                         c.zoom(scale).transform,
                         g);
            });
            self.doflush = false;
          }
        }
        Event::Update(_) => {}
        _ => {}
      };
    }
  }
}
