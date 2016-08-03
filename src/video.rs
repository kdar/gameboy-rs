use std::fmt;
use piston_window::*;
// use im;
// use time::{Duration, SteadyTime};
use super::mem::MemoryIo;
use super::bit::Bit;

// Swichable bank 0-1 in CGB Mode
pub const VIDEO_RAM_START: u16 = 0x8000;
pub const VIDEO_RAM_END: u16 = 0x9FFF;

// Sprite attribute table (OAM)
pub const SPRITE_TABLE_START: u16 = 0xFE00;
pub const SPRITE_TABLE_END: u16 = 0xFE9F;

pub const VIDEO_CONTROL_START: u16 = 0xFF40;
pub const VIDEO_CONTROL_END: u16 = 0xFF4C;

const LCD_CONTROL: u16 = 0xFF40;
const LCD_CONTROLLER_STATUS: u16 = 0xFF41;
// const SCROLL_Y: u16 = 0xFF42;
// const SCROLL_X: u16 = 0xFF43;
const LCD_CONTROLLER_Y_COORDINATE: u16 = 0xFF44;
// const LY_COMPARE: u16 = 0xFF45;
// const WINDOW_Y_POSITION: u16 = 0xFF4A;
// const WINDOW_X_POSITION_MINUS_7: u16 = 0xFF4B;
const BG_PALETTE_DATA: u16 = 0xFF47;
const OBJECT_PALETTE0_DATA: u16 = 0xFF48;
const OBJECT_PALETTE1_DATA: u16 = 0xFF49;
// const DMA_TRANSFER_AND_START_ADDRESS: u16 = 0xFF46;

const VBLANK_CYCLES: usize = 456;
const HBLANK_CYCLES: usize = 204;
const READING_OAM_CYCLES: usize = 80;
const READING_VRAM_CYCLES: usize = 172;

enum LcdControl {
  DisplayOn = 1 << 7, // Bit 7 - LCD Display Enable             (0=Off, 1=On)
  WindowMap = 1 << 6, // Bit 6 - Window Tile Map Display Select (0=9800-9BFF, 1=9C00-9FFF)
  WindowOn = 1 << 5, // Bit 5 - Window Display Enable          (0=Off, 1=On)
  BgSelect = 1 << 4, // Bit 4 - BG & Window Tile Data Select   (0=8800-97FF, 1=8000-8FFF)
  BgMap = 1 << 3, // Bit 3 - BG Tile Map Display Select     (0=9800-9BFF, 1=9C00-9FFF)
  ObjSize = 1 << 2, // Bit 2 - OBJ (Sprite) Size              (0=8x8, 1=8x16)
  ObjOn = 1 << 1, // Bit 1 - OBJ (Sprite) Display Enable    (0=Off, 1=On)
  BgOn = 1, // Bit 0 - BG Display (for CGB see below) (0=Off, 1=On)
}

enum LcdStatus {
  LYCoincidenceInterrupt = 1 << 6, // (1=Enable) (Read/Write)
  OamInterrupt = 1 << 5, // (1=Enable) (Read/Write)
  VblankInterrupt = 1 << 4, // (1=Enable) (Read/Write)
  HblankInterrupt = 1 << 3, // (1=Enable) (Read/Write)
  CoincidenceFlag = 1 << 2, // (0:LYC<>LY, 1:LYC=LY) (Read Only)
  ModeFlag = 0b11, // (Mode 0-3, see below) (Read Only)
}

enum LcdMode {
  Hblank = 0, // During H-Blank
  Vblank = 1, // During V-Blank
  AccessOam = 2, // During Searching OAM-RAM
  AccessVram = 3, // During Transfering Data to LCD Driver
}

pub struct Video {
  window: Option<PistonWindow>,
  // tilemap: [[u8; ]],
  control: u8,
  status: u8,
  mode: u8,
  cycles: usize,
  bg_palette: [u8; 4],
  obj_palette0: [u8; 4],
  obj_palette1: [u8; 4],
  current_line: u8,
  vram: [u8; 8192], // Video ram
  oam: [u8; 160], // Sprite attribute table
}

impl Default for Video {
  fn default() -> Video {
    Video {
      window: None,
      control: 0,
      status: 0,
      mode: LcdMode::Hblank as u8,
      cycles: 0,
      bg_palette: [0; 4],
      obj_palette0: [0; 4],
      obj_palette1: [0; 4],
      current_line: 0,
      vram: [0; 8192],
      oam: [0; 160],
    }
  }
}

impl fmt::Debug for Video {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    try!(write!(f, "\nVRAM: {:?}", &&self.vram[..]));
    write!(f, "\n")
  }
}

impl MemoryIo for Video {
  fn read_byte(&self, addr: u16) -> Result<u8, String> {
    println!("reading vid byte from: {:#04x}", addr);
    match addr {
      LCD_CONTROL => Ok(self.control),
      LCD_CONTROLLER_Y_COORDINATE => Ok(self.current_line),
      _ => Ok(0),
    }
  }

  fn write_byte(&mut self, addr: u16, value: u8) -> Result<(), String> {
    match addr {
      LCD_CONTROL => {
        let old_lcd_on = self.control.has_bit(LcdControl::DisplayOn as usize);
        let new_lcd_on = value.has_bit(LcdControl::DisplayOn as usize);

        // The value coming in tells us to turn off the LCD, while the
        // previous value was on.
        if !new_lcd_on && !old_lcd_on {
          if self.mode == LcdMode::Vblank as u8 {
            panic!("The LCD should not be turned off while not in VBlank. This action can cause \
                    damage in a real Gameboy.");
          }
          self.current_line = 0;
        }

        // The value coming in tells us to turn on the LCD, while the
        // previous value was off.
        if new_lcd_on && !old_lcd_on {
          self.mode = LcdMode::Hblank as u8;
          self.status.set_bit(LcdStatus::CoincidenceFlag as usize);
          self.cycles = READING_OAM_CYCLES;
        }
      }

      LCD_CONTROLLER_STATUS => {}

      VIDEO_RAM_START...VIDEO_RAM_END => {
        let offset = addr as usize - VIDEO_RAM_START as usize;
        self.vram[offset] = value;
      }

      // LCD_CONTROLLER_Y_COORDINATE => self.current_line = value,
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

    Ok(())
  }
}

impl Video {
  pub fn new() -> Video {
    Video {
      // window: Some(WindowSettings::new("Gameboy-rs", [160, 144])
      //   .exit_on_esc(true)
      //   .build()
      //   .unwrap()),
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
