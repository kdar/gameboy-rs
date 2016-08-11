use std::fmt;
use piston_window::*;
use num::FromPrimitive;
// use im;
// use time::{Duration, SteadyTime};
use super::mem::MemoryIo;
use super::bit::Bit;

const VBLANK_CYCLES: isize = 114;
const HBLANK_CYCLES: isize = 50;
const READING_OAM_CYCLES: isize = 21;
const READING_VRAM_CYCLES: isize = 43;
const TILE_DATA_SIZE: usize = 192 * 2;
const TILE_MAP_SIZE: usize = 1024;

#[derive(Copy, Clone, NumFromPrimitive)]
enum Color {
  White = 0,
  LightGray = 1,
  DarkGray = 2,
  Black = 3,
}

struct Palette {
  colors: [Color; 4],
  value: u8,
}

impl Default for Palette {
  fn default() -> Palette {
    Palette {
      colors: [Color::White; 4],
      value: 0,
    }
  }
}

impl Palette {
  fn from_u8(value: u8) -> Palette {
    Palette {
      colors: [FromPrimitive::from_u8(value & 0b11).unwrap(),
               FromPrimitive::from_u8(value >> 2 & 0b11).unwrap(),
               FromPrimitive::from_u8(value >> 4 & 0b11).unwrap(),
               FromPrimitive::from_u8(value >> 6 & 0b11).unwrap()],
      value: value,
    }
  }
}

#[derive(Debug, PartialEq)]
enum LcdControl {
  DisplayOn = 1 << 7, // Bit 7 - LCD Display Enable             (0=Off, 1=On)
  WindowMap = 1 << 6, // Bit 6 - Window Tile Map Display Select (0=9800-9BFF, 1=9C00-9FFF)
  WindowOn = 1 << 5, // Bit 5 - Window Display Enable          (0=Off, 1=On)
  BgSelect = 1 << 4, // Bit 4 - BG & Window Tile Data Select   (0=8800-97FF, 1=8000-8FFF)
  BgMap = 1 << 3, // Bit 3 - BG Tile Map Display Select     (0=9800-9BFF, 1=9C00-9FFF)
  ObjSize = 1 << 2, // Bit 2 - OBJ (Sprite) Size              (0=8x8, 1=8x16)
  ObjOn = 1 << 1, // Bit 1 - OBJ (Sprite) Display Enable    (0=Off, 1=On)
  BgOn = 1, // Bit 0 - BG Display (for CGB see below) (0=Off, 1=On)
  None = 0,
}

#[derive(Debug, PartialEq)]
enum LcdStatus {
  CoincidenceInterrupt = 1 << 6, // (1=Enable) (Read/Write)
  OamInterrupt = 1 << 5, // (1=Enable) (Read/Write)
  VblankInterrupt = 1 << 4, // (1=Enable) (Read/Write)
  HblankInterrupt = 1 << 3, // (1=Enable) (Read/Write)
  CoincidenceFlag = 1 << 2, // (0:LYC<>LY, 1:LYC=LY) (Read Only)
  ModeFlag = 0b11, // (Mode 0-3, see below) (Read Only)
}

#[derive(Debug, PartialEq, NumFromPrimitive)]
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
  mode: LcdMode,
  cycles: isize,
  scroll_y: u8,
  scroll_x: u8,
  window_y: u8,
  window_x: u8,
  ly_compare: u8,
  bg_palette: Palette,
  obj_palette0: Palette,
  obj_palette1: Palette,
  current_line: u8,
  tile_data: [[u8; 16]; TILE_DATA_SIZE],
  tile_map1: [u8; TILE_MAP_SIZE],
  tile_map2: [u8; TILE_MAP_SIZE],
  oam: [u8; 160], // Sprite attribute table
}

impl Default for Video {
  fn default() -> Video {
    Video {
      window: None,
      control: 0,
      status: 0,
      mode: LcdMode::Hblank,
      cycles: READING_OAM_CYCLES,
      scroll_y: 0,
      scroll_x: 0,
      window_y: 0,
      window_x: 0,
      ly_compare: 0,
      bg_palette: Palette::default(),
      obj_palette0: Palette::default(),
      obj_palette1: Palette::default(),
      current_line: 0x0,
      tile_data: [[0; 16]; TILE_DATA_SIZE],
      tile_map1: [0; TILE_MAP_SIZE],
      tile_map2: [0; TILE_MAP_SIZE],
      oam: [0; 160],
    }
  }
}

impl fmt::Debug for Video {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    try!(write!(f, "\nVideo:"));
    write!(f, "\n")
  }
}

impl MemoryIo for Video {
  fn read_u8(&self, addr: u16) -> Result<u8, String> {
    // println!("reading vid byte from: {:#04x}", addr);
    match addr {
      0x8000...0x97ff => {
        if self.mode == LcdMode::AccessVram {
          return Ok(0);
        }

        let offset = addr - 0x8000;
        let tile = &self.tile_data[offset as usize / 16];
        Ok(tile[offset as usize % 16])
      }
      0x9800...0x9bff => {
        if self.mode == LcdMode::AccessVram {
          return Ok(0);
        }

        let offset = addr - 0x9800;
        Ok(self.tile_map1[offset as usize])
      }
      0x9c00...0x9fff => {
        if self.mode == LcdMode::AccessVram {
          return Ok(0);
        }

        let offset = addr - 0x9c00;
        Ok(self.tile_map2[offset as usize])
      }
      0xfe00...0xfe9f => Ok(self.oam[addr as usize - 0xfe00 as usize]),
      0xff40 => Ok(self.control),
      0xff41 => Ok(self.status),
      0xff42 => Ok(self.scroll_y),
      0xff43 => Ok(self.scroll_x),
      0xff44 => {
        // println!("read: {}", self.current_line);
        Ok(self.current_line)
      }
      0xff45 => Ok(self.ly_compare),
      0xff47 => Ok(self.bg_palette.value),
      0xff48 => Ok(self.obj_palette0.value),
      0xff49 => Ok(self.obj_palette1.value),
      0xff4a => Ok(self.window_y),
      0xff4B => Ok(self.window_x),
      _ => Ok(0),
    }
  }

  fn write_u8(&mut self, addr: u16, value: u8) -> Result<(), String> {
    match addr {
      0x8000...0x97ff => {
        if self.mode == LcdMode::AccessVram {
          return Ok(());
        }

        let offset = addr - 0x8000;
        let tile = &mut self.tile_data[offset as usize / 16];
        tile[offset as usize % 16] = value;
      }
      0x9800...0x9bff => {
        if self.mode == LcdMode::AccessVram {
          return Ok(());
        }

        let offset = addr - 0x9800;
        self.tile_map1[offset as usize] = value;
      }
      0x9c00...0x9fff => {
        if self.mode == LcdMode::AccessVram {
          return Ok(());
        }

        let offset = addr - 0x9c00;
        self.tile_map2[offset as usize] = value;
      }
      0xff40 => {
        let old_lcd_on = self.control.has_bit(LcdControl::DisplayOn as usize);
        let new_lcd_on = value.has_bit(LcdControl::DisplayOn as usize);

        // The value coming in tells us to turn off the LCD, while the
        // previous value was on.
        if !new_lcd_on && !old_lcd_on {
          if self.mode == LcdMode::Vblank {
            panic!("The LCD should not be turned off while not in VBlank. This action can cause \
                    damage in a real Gameboy.");
          }
          self.current_line = 0;
        }

        // The value coming in tells us to turn on the LCD, while the
        // previous value was off.
        if new_lcd_on && !old_lcd_on {
          self.mode = LcdMode::Hblank;
          self.status.set_bit(LcdStatus::CoincidenceFlag as usize);
          self.cycles = READING_OAM_CYCLES;
        }

        // self.current_line = 153;
        self.control = value;
      }

      0xff41 => {
        // Bits 0-2 are read only.
        self.status = (value & 0b11111000) | (self.status & 0b00000111);
      }

      0xff42 => self.scroll_y = value,
      0xff43 => self.scroll_x = value,
      0xff44 => self.current_line = 0,
      0xff45 => self.ly_compare = value,

      0x8000...0x9fff => {
        // let offset = addr as usize - 0x8000 as usize;
        // self.vram[offset] = value;
      }

      // 0xff44 => self.current_line = value,
      0xff47 => {
        // println!("video: bg palette: {:#04x}", addr);
        self.bg_palette = Palette::from_u8(value);
      }
      0xff48 => {
        // println!("video: obj 0 palette: {:#04x}", addr);
        self.obj_palette0 = Palette::from_u8(value);
      }
      0xff49 => {
        // println!("video: obj 1 palette: {:#04x}", addr);
        self.obj_palette1 = Palette::from_u8(value);
      }

      0xff4a => self.window_y = value,
      0xff4B => self.window_x = value,

      0xff46 => {
        println!("DMA TRANSFER START");
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

  pub fn step(&mut self) {
    if !self.control.has_bit(LcdControl::DisplayOn as usize) {
      return;
    }

    self.cycles -= 1;

    // println!("{}", self.cycles);

    if self.cycles > 0 {
      return;
    }

    match self.mode {
      LcdMode::AccessOam => {
        // println!("access oam");
        self.mode = LcdMode::AccessVram;
        self.cycles += READING_VRAM_CYCLES;
      }
      LcdMode::AccessVram => {
        // println!("access vram");
        self.render_line();
        self.mode = LcdMode::Hblank;
        self.cycles += HBLANK_CYCLES;
      }
      LcdMode::Hblank => {
        // println!("hblank");
        self.current_line += 1;
        if self.current_line < 144 {
          self.mode = LcdMode::AccessOam;
          self.cycles += READING_OAM_CYCLES;
        } else {
          self.mode = LcdMode::Vblank;
          self.cycles += VBLANK_CYCLES;
        }
      }
      LcdMode::Vblank => {
        // println!("vblank");
        self.current_line += 1;
        if self.current_line > 153 {
          self.current_line = 0;
          self.mode = LcdMode::AccessOam;
          self.cycles += READING_OAM_CYCLES;
        } else {
          self.cycles += VBLANK_CYCLES;
        }
      }
    }
  }

  fn render_line(&self) {
    // println!("render_line: {}", self.current_line);
  }

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
