use std::fmt;
use num::FromPrimitive;
use std::sync::mpsc::Sender;
// use time::{Duration, SteadyTime};

use super::mem::MemoryIo;
use super::GbEvent;

const VBLANK_CYCLES: isize = 114;
const HBLANK_CYCLES: isize = 50;
const READING_OAM_CYCLES: isize = 21;
const READING_VRAM_CYCLES: isize = 43;
const TILE_DATA_SIZE: usize = 192 * 2;
const TILE_MAP_SIZE: usize = 1024;
pub const SCREEN_WIDTH: u32 = 160;
pub const SCREEN_HEIGHT: u32 = 144;

#[derive(Copy, Clone, Debug, NumFromPrimitive)]
enum Color {
  White = 0,
  LightGray = 1,
  DarkGray = 2,
  Black = 3,
}

impl Color {
  fn pixel(&self) -> [u8; 4] {
    match *self {
      Color::White => [0xff, 0xff, 0xff, 0xff],
      Color::LightGray => [0xc0, 0xc0, 0xc0, 0xff],
      Color::DarkGray => [0x60, 0x60, 0x60, 0xff],
      Color::Black => [0x00, 0x00, 0x00, 0x00],
    }
  }
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

bitflags! {
    flags LcdControl: u8 {
      const LCD_DISPLAY_ON = 0b10000000, // Bit 7 - LCD Display Enable             (0=Off, 1=On)
      const LCD_WINDOW_MAP = 0b01000000, // Bit 6 - Window Tile Map Display Select (0=9800-9BFF, 1=9C00-9FFF)
      const LCD_WINDOW_ON =  0b00100000, // Bit 5 - Window Display Enable          (0=Off, 1=On)
      const LCD_BG_SELECT =  0b00010000, // Bit 4 - BG & Window Tile Data Select   (0=8800-97FF, 1=8000-8FFF)
      const LCD_BG_MAP =     0b00001000, // Bit 3 - BG Tile Map Display Select     (0=9800-9BFF, 1=9C00-9FFF)
      const LCD_OBJ_SIZE =   0b00000100, // Bit 2 - OBJ (Sprite) Size              (0=8x8, 1=8x16)
      const LCD_OBJ_ON =     0b00000010, // Bit 1 - OBJ (Sprite) Display Enable    (0=Off, 1=On)
      const LCD_BG_ON =      0b00000001, // Bit 0 - BG Display (for CGB see below) (0=Off, 1=On)
    }
}

bitflags! {
    flags LcdStatus: u8 {
      const LCD_COINCIDENCE_INTERRUPT = 0b01000000, // (1=Enable) (Read/Write)
      const LCD_OAM_INTERRUPT =         0b00100000, // (1=Enable) (Read/Write)
      const LCD_VBLANK_INTERRUPT =      0b00010000, // (1=Enable) (Read/Write)
      const LCD_HBLANK_INTERRUPT =      0b00001000, // (1=Enable) (Read/Write)
      const LCD_COINCIDENCE_FLAG =      0b00000100, // (0:LYC<>LY, 1:LYC=LY) (Read Only)
    }
}

#[derive(Debug, PartialEq, NumFromPrimitive)]
enum LcdMode {
  Hblank = 0, // During H-Blank
  Vblank = 1, // During V-Blank
  AccessOam = 2, // During Searching OAM-RAM
  AccessVram = 3, // During Transfering Data to LCD Driver
}

pub struct Video {
  control: LcdControl,
  status: LcdStatus,
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
  line: u8,
  tile_data: [[u8; 16]; TILE_DATA_SIZE],
  tile_map1: [u8; TILE_MAP_SIZE],
  tile_map2: [u8; TILE_MAP_SIZE],
  // Sprite attribute table
  oam: [u8; 160],
  event_sender: Option<Sender<GbEvent>>,
  pixels: [[u8; 4]; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize],
}

impl Default for Video {
  fn default() -> Video {
    Video {
      control: LcdControl::empty(),
      status: LcdStatus::empty(),
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
      line: 0x0,
      tile_data: [[0; 16]; TILE_DATA_SIZE],
      tile_map1: [0; TILE_MAP_SIZE],
      tile_map2: [0; TILE_MAP_SIZE],
      oam: [0; 160],
      event_sender: None,
      pixels: [Color::White.pixel(); SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize],
    }
  }
}

impl fmt::Debug for Video {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    write!(f, "\n")
  }
}

impl MemoryIo for Video {
  fn read_u8(&self, addr: u16) -> Result<u8, String> {
    // println!("reading vid byte from: {:#04x}", addr);
    match addr {
      0x8000...0x97ff => {
        // if self.mode == LcdMode::AccessVram {
        //  return Ok(0);
        // }

        let offset = addr - 0x8000;
        let tile = &self.tile_data[offset as usize / 16];
        Ok(tile[offset as usize % 16])
      }
      0x9800...0x9bff => {
        // if self.mode == LcdMode::AccessVram {
        //  return Ok(0);
        // }

        let offset = addr - 0x9800;
        Ok(self.tile_map1[offset as usize])
      }
      0x9c00...0x9fff => {
        // if self.mode == LcdMode::AccessVram {
        //  return Ok(0);
        // }

        let offset = addr - 0x9c00;
        Ok(self.tile_map2[offset as usize])
      }
      0xfe00...0xfe9f => Ok(self.oam[addr as usize - 0xfe00 as usize]),
      0xff40 => Ok(self.control.bits),
      0xff41 => Ok(self.status.bits),
      0xff42 => Ok(self.scroll_y),
      0xff43 => Ok(self.scroll_x),
      0xff44 => Ok(self.line),
      0xff45 => Ok(self.ly_compare),
      0xff47 => Ok(self.bg_palette.value),
      0xff48 => Ok(self.obj_palette0.value),
      0xff49 => Ok(self.obj_palette1.value),
      0xff4a => Ok(self.window_y),
      0xff4B => Ok(self.window_x),
      _ => panic!("video.read_u8: non implemented range: {:#04x}", addr),
    }
  }

  fn write_u8(&mut self, addr: u16, value: u8) -> Result<(), String> {
    match addr {
      0x8000...0x97ff => {
        // if self.mode == LcdMode::AccessVram {
        //  return Ok(());
        // }

        let offset = addr - 0x8000;
        let tile = &mut self.tile_data[offset as usize / 16];
        tile[offset as usize % 16] = value;
        // println!("write tile data {:x} @ {:x}", value, offset);
      }
      0x9800...0x9bff => {
        // if self.mode == LcdMode::AccessVram {
        //  return Ok(());
        // }

        let offset = addr - 0x9800;
        self.tile_map1[offset as usize] = value;
      }
      0x9c00...0x9fff => {
        // if self.mode == LcdMode::AccessVram {
        //  return Ok(());
        // }

        let offset = addr - 0x9c00;
        self.tile_map2[offset as usize] = value;
      }
      0xfe00...0xfe9f => {
        self.oam[addr as usize - 0xfe00 as usize] = value;
      }
      0xff40 => {
        let old_lcd_on = self.control.contains(LCD_DISPLAY_ON);
        let new_lcd_on = value & LCD_DISPLAY_ON.bits > 0;

        // The value coming in tells us to turn off the LCD, while the
        // previous value was on.
        if !new_lcd_on && !old_lcd_on {
          if self.mode == LcdMode::Vblank {
            panic!("The LCD should not be turned off while not in VBlank. This action can cause \
                    damage in a real Gameboy.");
          }
          self.line = 0;
        }

        // The value coming in tells us to turn on the LCD, while the
        // previous value was off.
        if new_lcd_on && !old_lcd_on {
          self.mode = LcdMode::Hblank;
          self.status.insert(LCD_COINCIDENCE_FLAG);
          self.cycles = READING_OAM_CYCLES;
        }

        // self.line = 153;
        self.control = LcdControl::from_bits(value).unwrap();
      }

      0xff41 => {
        // Bits 0-2 are read only.
        self.status = LcdStatus::from_bits((value & 0b11111000) | (self.status.bits & 0b00000111)).unwrap();
      }

      0xff42 => self.scroll_y = value,
      0xff43 => self.scroll_x = value,
      0xff44 => self.line = 0,
      0xff45 => self.ly_compare = value,

      // 0x8000...0x9fff => {
      // let offset = addr as usize - 0x8000 as usize;
      // self.vram[offset] = value;
      // }
      //
      // 0xff44 => self.line = value,
      0xff47 => {
        self.bg_palette = Palette::from_u8(value);
        // println!("video: bg palette: {:?}", self.bg_palette.colors);
      }
      0xff48 => {
        self.obj_palette0 = Palette::from_u8(value);
        // println!("video: obj 0 palette: {:?}", self.obj_palette0.colors);
      }
      0xff49 => {
        self.obj_palette1 = Palette::from_u8(value);
        // println!("video: obj 1 palette: {:?}", self.obj_palette1.colors);
      }

      0xff4a => self.window_y = value,
      0xff4b => self.window_x = value,

      0xff46 => {
        println!("DMA TRANSFER START");
      }

      _ => println!("video.write_u8: non implemented range: {:#04x}", addr),
    };

    Ok(())
  }
}

impl Video {
  pub fn new() -> Video {
    Video::default()
  }

  pub fn set_event_sender(&mut self, s: Sender<GbEvent>) {
    self.event_sender = Some(s);
  }

  pub fn debug(&self) {
    for x in 0..16 {
      println!("{:08b} ", self.tile_data[0][x]);
    }
    println!("");
    panic!("");
  }

  pub fn step(&mut self) {
    if !self.control.contains(LCD_DISPLAY_ON) {
      return;
    }

    self.cycles -= 1;

    // println!("{}", self.cycles);

    if self.cycles > 0 {
      return;
    }

    match self.mode {
      // Mode 2
      LcdMode::AccessOam => {
        // println!("access oam");
        self.mode = LcdMode::AccessVram;
        self.cycles += READING_VRAM_CYCLES;
      }
      // Mode 3
      LcdMode::AccessVram => {
        // println!("access vram");
        self.render_line();
        self.mode = LcdMode::Hblank;
        self.cycles += HBLANK_CYCLES;
      }
      // Mode 0
      LcdMode::Hblank => {
        // println!("hblank");
        self.line += 1;
        if self.line < 144 {
          self.mode = LcdMode::AccessOam;
          self.cycles += READING_OAM_CYCLES;
        } else {
          self.mode = LcdMode::Vblank;
          self.cycles += VBLANK_CYCLES;
          self.render_image();
        }
      }
      // Mode 1
      LcdMode::Vblank => {
        // println!("vblank");
        self.line += 1;
        if self.line > 153 {
          self.line = 0;
          self.mode = LcdMode::AccessOam;
          self.cycles += READING_OAM_CYCLES;
        } else {
          self.cycles += VBLANK_CYCLES;
        }
      }
    }
  }

  fn render_image(&mut self) {
    if let &Some(ref s) = &self.event_sender {
      s.send(GbEvent::Frame(self.pixels.to_vec())).unwrap();
    }
  }

  fn render_line(&mut self) {
    self.render_bg_line();
  }

  fn render_bg_line(&mut self) {
    if !self.control.contains(LCD_BG_ON) {
      // If the background is disabled we just render the line
      // as white.
      for x in 0..SCREEN_WIDTH {
        self.pixels[(self.line as usize) * (SCREEN_WIDTH as usize) + x as usize] = Color::White.pixel();
      }
      return;
    }

    // Get the tile map depending on what the bg map select is set
    // to in the control.
    let tile_map = if self.control.contains(LCD_BG_MAP) {
      &self.tile_map2
    } else {
      &self.tile_map1
    };

    // Get the offset into the tile data based on which area is selected
    // by the bg select in the control.
    let tile_data_offset = if self.control.contains(LCD_BG_SELECT) {
      0
    } else {
      128
    };

    // Find out our true bg Y coordinate by adding the y scroll position
    // to our current line.
    let bg_y = self.line.wrapping_add(self.scroll_y) as usize;
    // There are 32x32 tiles, where each tile is 8x8 pixels. So we
    // need to find the y component of the tile we're in by which
    // line+scroll_y we're on.
    let tile_y = (bg_y / 8) % 32;

    for x in 0..SCREEN_WIDTH {
      let bg_x = (x as usize).wrapping_add(self.scroll_x as usize);
      // Again, we get the x component of the tile we're in based on
      // x+scroll_x.
      let tile_x = (bg_x / 8) % 32;

      let tile_map_num = tile_map[tile_y * 32 + tile_x] as usize;

      // Select the tile based on control.
      let single_tile = if self.control.contains(LCD_BG_SELECT) {
        self.tile_data[tile_data_offset + tile_map_num]
      } else {
        self.tile_data[(tile_data_offset as i16 + (tile_map_num as i8 as i16)) as usize]
      };

      // Tile date is 16 bytes long, with each line being 2 bytes. We grab the correct bytes
      // by our current line, and then do some math to find out what color it is.
      let b1 = single_tile[(bg_y % 8) * 2];
      let b2 = single_tile[(bg_y % 8) * 2 + 1];
      let bit = (7 as usize).wrapping_sub(bg_x) % 8;
      let color: Color = FromPrimitive::from_u8(((b1 >> bit) & 0b1) | ((b2 >> bit) & 0b1) << 1).unwrap();

      self.pixels[(self.line as usize) * (SCREEN_WIDTH as usize) + (x as usize)] = color.pixel();
    }
  }
}
