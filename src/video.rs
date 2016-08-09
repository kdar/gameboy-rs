use std::fmt;
use piston_window::*;
use num::FromPrimitive;
// use im;
// use time::{Duration, SteadyTime};
use super::mem::MemoryIo;
use super::bit::Bit;

// Swichable bank 0-1 in CGB Mode
pub const VIDEO_RAM_START: u16 = 0x8000;
pub const VIDEO_RAM_END: u16 = 0x9FFF;

pub const TILE_DATA_START: u16 = 0x8000;
pub const TILE_DATA_END: u16 = 0x97FF;

pub const TILE_MAP_1_START: u16 = 0x9800;
pub const TILE_MAP_1_END: u16 = 0x9BFF;

pub const TILE_MAP_2_START: u16 = 0x9C00;
pub const TILE_MAP_2_END: u16 = 0x9FFF;

// Sprite attribute table (OAM)
pub const SPRITE_TABLE_START: u16 = 0xFE00;
pub const SPRITE_TABLE_END: u16 = 0xFE9F;

pub const VIDEO_CONTROL_START: u16 = 0xFF40;
pub const VIDEO_CONTROL_END: u16 = 0xFF4C;

const LCD_CONTROL: u16 = 0xFF40;
const LCD_CONTROLLER_STATUS: u16 = 0xFF41;
const SCROLL_Y: u16 = 0xFF42;
const SCROLL_X: u16 = 0xFF43;
const LCD_CONTROLLER_Y_COORDINATE: u16 = 0xFF44;
const LY_COMPARE: u16 = 0xFF45;
const DMA_TRANSFER_AND_START_ADDRESS: u16 = 0xFF46;
const BG_PALETTE_DATA: u16 = 0xFF47;
const OBJECT_PALETTE0_DATA: u16 = 0xFF48;
const OBJECT_PALETTE1_DATA: u16 = 0xFF49;
// WY - Window Y Position (R/W)
const WINDOW_Y_POSITION: u16 = 0xFF4A;
// WX - Window X Position minus 7 (R/W)
const WINDOW_X_POSITION: u16 = 0xFF4B;

const VBLANK_CYCLES: isize = 114;
const HBLANK_CYCLES: isize = 50;
const READING_OAM_CYCLES: isize = 21;
const READING_VRAM_CYCLES: isize = 43;
const TILE_DATA_SIZE: usize = 192*2;
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
  character_tiles: [[u8; 16]; TILE_DATA_SIZE],
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
      character_tiles: [[0; 16]; TILE_DATA_SIZE],
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
  fn read_byte(&self, addr: u16) -> Result<u8, String> {
    // println!("reading vid byte from: {:#04x}", addr);
    match addr {
      TILE_DATA_START...TILE_DATA_END => {
        if self.mode == LcdMode::AccessVram {
          return Ok(0)
        }

        let offset = addr - TILE_DATA_START;
        let tile = &self.character_tiles[offset as usize / 16];
        Ok(tile[offset as usize % 16])
      }
      TILE_MAP_1_START...TILE_MAP_1_END => {
        if self.mode == LcdMode::AccessVram {
          return Ok(0)
        }

        let offset = addr - TILE_MAP_1_START;
        Ok(self.tile_map1[offset as usize])
      }
      TILE_MAP_2_START...TILE_MAP_2_END => {
        if self.mode == LcdMode::AccessVram {
          return Ok(0)
        }

        let offset = addr - TILE_MAP_2_START;
        Ok(self.tile_map2[offset as usize])
      }
      SPRITE_TABLE_START...SPRITE_TABLE_END => Ok(self.oam[addr as usize - SPRITE_TABLE_START as usize]),
      LCD_CONTROL => Ok(self.control),
      LCD_CONTROLLER_STATUS => Ok(self.status),
      SCROLL_Y => Ok(self.scroll_y),
      SCROLL_X => Ok(self.scroll_x),
      LCD_CONTROLLER_Y_COORDINATE => {
        // println!("read: {}", self.current_line);
        Ok(self.current_line)
      },
      LY_COMPARE => Ok(self.ly_compare),
      BG_PALETTE_DATA => Ok(self.bg_palette.value),
      OBJECT_PALETTE0_DATA => Ok(self.obj_palette0.value),
      OBJECT_PALETTE1_DATA => Ok(self.obj_palette1.value),
      WINDOW_Y_POSITION => Ok(self.window_y),
      WINDOW_X_POSITION => Ok(self.window_x),
      _ => Ok(0),
    }
  }

  fn write_byte(&mut self, addr: u16, value: u8) -> Result<(), String> {
    match addr {
      TILE_DATA_START...TILE_DATA_END => {
        if self.mode == LcdMode::AccessVram {
          return Ok(());
        }

        let offset = addr - TILE_DATA_START;
        let tile = &mut self.character_tiles[offset as usize / 16];
        tile[offset as usize % 16] = value;
      }
      TILE_MAP_1_START...TILE_MAP_1_END => {
        if self.mode == LcdMode::AccessVram {
          return Ok(())
        }

        let offset = addr - TILE_MAP_1_START;
        self.tile_map1[offset as usize] = value;
      }
      TILE_MAP_2_START...TILE_MAP_2_END => {
        if self.mode == LcdMode::AccessVram {
          return Ok(())
        }

        let offset = addr - TILE_MAP_2_START;
        self.tile_map2[offset as usize] = value;
      }
      LCD_CONTROL => {
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

      LCD_CONTROLLER_STATUS => {
        // Bits 0-2 are read only.
        self.status = (value & 0b11111000) | (self.status & 0b00000111);
      }

      SCROLL_Y => self.scroll_y = value,
      SCROLL_X => self.scroll_x = value,
      LCD_CONTROLLER_Y_COORDINATE => self.current_line = 0,
      LY_COMPARE => self.ly_compare = value,

      VIDEO_RAM_START...VIDEO_RAM_END => {
        // let offset = addr as usize - VIDEO_RAM_START as usize;
        // self.vram[offset] = value;
      }

      // LCD_CONTROLLER_Y_COORDINATE => self.current_line = value,
      BG_PALETTE_DATA => {
        // println!("video: bg palette: {:#04x}", addr);
        self.bg_palette = Palette::from_u8(value);
      }
      OBJECT_PALETTE0_DATA => {
        // println!("video: obj 0 palette: {:#04x}", addr);
        self.obj_palette0 = Palette::from_u8(value);
      }
      OBJECT_PALETTE1_DATA => {
        // println!("video: obj 1 palette: {:#04x}", addr);
        self.obj_palette1 = Palette::from_u8(value);
      }

      WINDOW_Y_POSITION => self.window_y = value,
      WINDOW_X_POSITION => self.window_x = value,

      DMA_TRANSFER_AND_START_ADDRESS => {
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

  pub fn step(&mut self, clock_t: u32) {
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
