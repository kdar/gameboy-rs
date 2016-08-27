use piston_window::*;
use time::{Duration, SteadyTime};
use std::sync::mpsc::{self, Receiver};

use super::super::GbEvent;
use super::super::video;

pub struct PistonUi {
  win: Option<PistonWindow>,
  initial_scale: f64,
  width: u32,
  height: u32,
  doflush: bool,
  event_receiver: Receiver<GbEvent>,
}

impl Default for PistonUi {
  fn default() -> PistonUi {
    let (_, null_receiver) = mpsc::channel();
    PistonUi {
      win: None,
      initial_scale: 4.0,
      width: video::SCREEN_WIDTH,
      height: video::SCREEN_HEIGHT,
      doflush: false,
      event_receiver: null_receiver,
    }
  }
}

impl PistonUi {
  pub fn new(r: Receiver<GbEvent>) -> PistonUi {
    let mut u = PistonUi { event_receiver: r, ..PistonUi::default() };

    u.win = Some(WindowSettings::new("Gameboy-rs",
                                     [((u.width as f64) * u.initial_scale) as u32,
                                      ((u.height as f64) * u.initial_scale) as u32])
      .exit_on_esc(true)
      .build()
      .unwrap());
    u
  }

  pub fn run(&mut self) {
    let mut win = self.win.as_mut().unwrap();
    let mut data = vec![];

    let mut frame_count = 0;
    let mut start = SteadyTime::now();
    while let Some(e) = win.next() {
      frame_count += 1;

      if let Ok(evt) = self.event_receiver.try_recv() {
        let GbEvent::Frame(d) = evt;
        data = d;
        self.doflush = true;
      }

      if SteadyTime::now() - start >= Duration::seconds(1) {
        win.set_title(format!("Gameboy-rs: {} fps", frame_count));
        frame_count = 0;
        start = SteadyTime::now();
      }

      if let Event::Render(rargs) = e {
        let scale_x = rargs.draw_width as f64 / video::SCREEN_WIDTH as f64;
        let scale_y = rargs.draw_height as f64 / video::SCREEN_HEIGHT as f64;
        if self.doflush {
          win.draw_2d(&e, |c, g| {
            clear([0.0; 4], g);

            for (i, d) in data.iter().enumerate() {
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
          self.doflush = false;
        }
      }
    }
  }
}
