use std::fmt;
use num::FromPrimitive;
use std::sync::mpsc::Sender;

mod sprite;

use super::mem::MemoryIo;
use super::GbEvent;
use super::pic::{Pic, Interrupt};
use self::sprite::Sprite;

const VBLANK_CYCLES: isize = 114;
const HBLANK_CYCLES: isize = 50;
const READING_OAM_CYCLES: isize = 21;
const READING_VRAM_CYCLES: isize = 43;
const TILE_DATA_SIZE: usize = 192 * 2;
const TILE_MAP_SIZE: usize = 1024;
pub const SCREEN_WIDTH: u32 = 160;
pub const SCREEN_HEIGHT: u32 = 144;

#[derive(Copy, Clone, Debug, PartialEq, NumFromPrimitive)]
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
    // Bit 7 - LCD Display Enable             (0=Off, 1=On)
    const LCD_DISPLAY_ON =  0b10000000,
    // Bit 6 - Window Tile Map Display Select (0=9800-9BFF, 1=9C00-9FFF)
    const LCD_WIN_MAP =     0b01000000,
    // Bit 5 - Window Display Enable          (0=Off, 1=On)
    const LCD_WIN_ON =      0b00100000,
    // Bit 4 - BG & Window Tile Data Select   (0=8800-97FF, 1=8000-8FFF)
    const LCD_DATA_SELECT = 0b00010000,
    // Bit 3 - BG Tile Map Display Select     (0=9800-9BFF, 1=9C00-9FFF)
    const LCD_BG_MAP =      0b00001000,
    // Bit 2 - OBJ (Sprite) Size              (0=8x8, 1=8x16)
    const LCD_OBJ_SIZE =    0b00000100,
    // Bit 1 - OBJ (Sprite) Display Enable    (0=Off, 1=On)
    const LCD_OBJ_ON =      0b00000010,
    // Bit 0 - BG Display (for CGB see below) (0=Off, 1=On)
    const LCD_BG_ON =       0b00000001,
  }
}

bitflags! {
  flags LcdStatus: u8 {
    const STAT_UNUSED =               0b10000000,
    const STAT_COINCIDENCE_INTERRUPT = 0b01000000, // (1=Enable) (Read/Write)
    const STAT_OAM_INTERRUPT =         0b00100000, // (1=Enable) (Read/Write)
    const STAT_VBLANK_INTERRUPT =      0b00010000, // (1=Enable) (Read/Write)
    const STAT_HBLANK_INTERRUPT =      0b00001000, // (1=Enable) (Read/Write)
    const STAT_COINCIDENCE_FLAG =      0b00000100, // (0:LYC<>LY, 1:LYC=LY) (Read Only)
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
  win_y: u8,
  win_x: u8,
  ly_compare: u8,
  bg_palette: Palette,
  obj_palette0: Palette,
  obj_palette1: Palette,
  line: u8,
  tile_data: [[u8; 16]; TILE_DATA_SIZE],
  tile_map1: [u8; TILE_MAP_SIZE],
  tile_map2: [u8; TILE_MAP_SIZE],
  sprites: [Sprite; 40],
  event_sender: Option<Sender<GbEvent>>,
  pixels: [[u8; 4]; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize],
  bg_priority: [bool; SCREEN_WIDTH as usize],
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
      win_y: 0,
      win_x: 0,
      ly_compare: 0,
      bg_palette: Palette::default(),
      obj_palette0: Palette::default(),
      obj_palette1: Palette::default(),
      line: 0x0,
      tile_data: [[0; 16]; TILE_DATA_SIZE],
      tile_map1: [0; TILE_MAP_SIZE],
      tile_map2: [0; TILE_MAP_SIZE],
      sprites: [Sprite::default(); 40],
      event_sender: None,
      pixels: [Color::White.pixel(); SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize],
      bg_priority: [false; SCREEN_WIDTH as usize],
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
        // TODO: Uncomment these checks to AccessVram when we have timing correct.
        // if self.mode == LcdMode::AccessVram {
        //  return Ok(0);
        // }

        let offset = (addr as usize) - 0x8000;
        let tile = &self.tile_data[offset / 16];
        Ok(tile[offset % 16])
      }
      0x9800...0x9bff => {
        // if self.mode == LcdMode::AccessVram {
        //  return Ok(0);
        // }

        let offset = (addr as usize) - 0x9800;
        Ok(self.tile_map1[offset])
      }
      0x9c00...0x9fff => {
        // if self.mode == LcdMode::AccessVram {
        //  return Ok(0);
        // }

        let offset = (addr as usize) - 0x9c00;
        Ok(self.tile_map2[offset])
      }
      0xfe00...0xfe9f => {
        let offset = (addr as usize) - 0xfe00;
        let sprite = &self.sprites[offset / 4];
        Ok(match offset % 4 {
          0 => sprite.y,
          1 => sprite.x,
          2 => sprite.tile,
          3 => sprite.flags(),
          _ => panic!("video.read_u8: unexpected sprite attribute"),
        })
      }
      0xff40 => Ok(self.control.bits),
      0xff41 => {
        if self.control.contains(LCD_DISPLAY_ON) {
          Ok(self.status.bits | STAT_UNUSED.bits)
        } else {
          // Bits 0-2 return 0 when lcd is off.
          Ok(self.status.bits | STAT_UNUSED.bits & 0b11111000)
        }
      }
      0xff42 => Ok(self.scroll_y),
      0xff43 => Ok(self.scroll_x),
      0xff44 => Ok(self.line),
      0xff45 => Ok(self.ly_compare),
      0xff47 => Ok(self.bg_palette.value),
      0xff48 => Ok(self.obj_palette0.value),
      0xff49 => Ok(self.obj_palette1.value),
      0xff4a => Ok(self.win_y),
      0xff4b => Ok(self.win_x),
      _ => panic!("video.read_u8: non implemented range: {:#04x}", addr),
    }
  }

  fn write_u8(&mut self, addr: u16, value: u8) -> Result<(), String> {
    match addr {
      0x8000...0x97ff => {
        // if self.mode == LcdMode::AccessVram {
        //  return Ok(());
        // }

        let offset = (addr as usize) - 0x8000;
        let tile = &mut self.tile_data[offset / 16];
        tile[offset % 16] = value;
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
        let offset = (addr as usize) - 0xfe00;
        let mut sprite = &mut self.sprites[offset / 4];

        match offset % 4 {
          0 => sprite.y = value,
          1 => sprite.x = value,
          2 => sprite.tile = value,
          3 => sprite.set_flags(value),
          _ => panic!("video.write_u8: unexpected sprite attribute"),
        };
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
          self.status.insert(STAT_COINCIDENCE_FLAG);
          self.cycles = READING_OAM_CYCLES;
        }

        // self.line = 153;
        self.control = LcdControl::from_bits(value).unwrap();
      }

      0xff41 => {
        // Bits 0-2 are read only.
        self.status = LcdStatus::from_bits_truncate((value & 0b11111000) |
                                                    (self.status.bits & 0b00000111));
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

      0xff4a => self.win_y = value,
      0xff4b => self.win_x = value,

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

  fn set_mode(&mut self, mode: LcdMode, pic: &mut Pic) {
    self.mode = mode;
    match self.mode {
      LcdMode::AccessOam => {
        if self.status.contains(STAT_OAM_INTERRUPT) {
          pic.interrupt(Interrupt::LcdStat);
        }
      }
      LcdMode::Vblank => {
        pic.interrupt(Interrupt::Vblank);
        if self.status.contains(STAT_VBLANK_INTERRUPT) || self.status.contains(STAT_OAM_INTERRUPT) {
          pic.interrupt(Interrupt::LcdStat);
        }
      }
      _ => (),
    }
  }

  pub fn step(&mut self, pic: &mut Pic) {
    if !self.control.contains(LCD_DISPLAY_ON) {
      return;
    }

    self.cycles -= 1;
    if self.cycles == 1 && self.mode == LcdMode::AccessVram &&
       self.status.contains(STAT_HBLANK_INTERRUPT) {
      pic.interrupt(Interrupt::LcdStat);
    }
    if self.cycles > 0 {
      return;
    }

    match self.mode {
      // Mode 2
      LcdMode::AccessOam => {
        // println!("access oam");
        self.set_mode(LcdMode::AccessVram, pic);
        self.cycles += READING_VRAM_CYCLES;
      }
      // Mode 3
      LcdMode::AccessVram => {
        // println!("access vram");
        self.render_line();
        self.set_mode(LcdMode::Hblank, pic);
        self.cycles += HBLANK_CYCLES;
      }
      // Mode 0
      LcdMode::Hblank => {
        // println!("hblank");
        self.line += 1;
        if self.line < 144 {
          self.set_mode(LcdMode::AccessOam, pic);
          self.cycles += READING_OAM_CYCLES;
        } else {
          self.set_mode(LcdMode::Vblank, pic);
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
          self.set_mode(LcdMode::AccessOam, pic);
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

    if self.control.contains(LCD_WIN_ON) {
      self.render_win_line();
    }

    if self.control.contains(LCD_OBJ_ON) {
      self.render_obj_line();
    }
  }

  fn render_bg_line(&mut self) {
    if !self.control.contains(LCD_BG_ON) {
      // If the background is disabled we just render the line
      // as white.
      for x in 0..SCREEN_WIDTH {
        self.pixels[(self.line as usize) * (SCREEN_WIDTH as usize) + x as usize] =
          Color::White.pixel();
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

    // Find out our true bg Y coordinate by adding the y scroll position
    // to our current line.
    let bg_y = self.line.wrapping_add(self.scroll_y) as usize;
    // There are 32x32 tiles, where each tile is 8x8 pixels. So we
    // need to find the y component of the tile we're in by which
    // line+scroll_y we're on.
    let tile_y = (bg_y / 8) % 32;

    for x in 0..(SCREEN_WIDTH as usize) {
      let bg_x = x.wrapping_add(self.scroll_x as usize);
      // Again, we get the x component of the tile we're in based on
      // x+scroll_x.
      let tile_x = (bg_x / 8) % 32;

      let tile_map_num = tile_map[tile_y * 32 + tile_x] as usize;

      // Select the tile based on control.
      let single_tile = if self.control.contains(LCD_DATA_SELECT) {
        self.tile_data[tile_map_num]
      } else {
        self.tile_data[(128 + (tile_map_num as i8 as i16)) as usize]
      };

      // Tile date is 16 bytes long, with each line being 2 bytes. We grab the correct bytes
      // by our current line, and then do some math to find out what color it is.
      let b1 = single_tile[(bg_y % 8) * 2];
      let b2 = single_tile[(bg_y % 8) * 2 + 1];
      let bit = (7 as usize).wrapping_sub(bg_x) % 8;
      let color_num = ((b1 >> bit) & 0b1) | ((b2 >> bit) & 0b1) << 1;
      let color: Color = self.bg_palette.colors[color_num as usize];
      self.bg_priority[x] = color != Color::White;

      self.pixels[(self.line as usize) * (SCREEN_WIDTH as usize) + x] = color.pixel();
    }
  }

  fn render_win_line(&mut self) {
    let win_y = self.line as isize - self.win_y as isize;
    if win_y < 0 {
      return;
    }

    let win_y = win_y as usize;
    let win_x = self.win_x.wrapping_sub(7) as usize;

    let tile_map = if self.control.contains(LCD_WIN_MAP) {
      &self.tile_map2
    } else {
      &self.tile_map1
    };

    let tile_y = win_y / 8;

    for x in 0..(SCREEN_WIDTH as usize) {
      if x < win_x {
        continue;
      }

      let tile_x = x.wrapping_sub(win_x) / 8;
      let tile_map_num = tile_map[tile_y * 32 + tile_x] as usize;

      let single_tile = if self.control.contains(LCD_DATA_SELECT) {
        self.tile_data[tile_map_num]
      } else {
        self.tile_data[(128 + (tile_map_num as i8 as i16)) as usize]
      };

      let b1 = single_tile[(win_y % 8) * 2];
      let b2 = single_tile[(win_y % 8) * 2 + 1];
      let bit = (7 as usize).wrapping_sub(x) % 8;
      let color_num = ((b1 >> bit) & 0b1) | ((b2 >> bit) & 0b1) << 1;
      let color: Color = self.bg_palette.colors[color_num as usize];
      self.bg_priority[x] = color != Color::White;

      self.pixels[(self.line as usize) * (SCREEN_WIDTH as usize) + x] = color.pixel();
    }
  }

  fn render_obj_line(&mut self) {
    let sprite_height: u8 = if self.control.contains(LCD_OBJ_SIZE) {
      16
    } else {
      8
    };

    let sprites: Vec<&Sprite> = self.sprites
      .iter()
      .filter(|sprite| {
        self.line >= sprite.screen_y() && self.line < sprite.screen_y().wrapping_add(sprite_height)
      })
      .take(10)
      .collect();

    for sprite in sprites {
      let palette = if sprite.has_palette1() {
        &self.obj_palette1
      } else {
        &self.obj_palette0
      };

      let mut sprite_tile = sprite.tile as usize;
      let mut line = if sprite.has_yflip() {
        sprite_height.wrapping_sub(self.line.wrapping_sub(sprite.screen_y())).wrapping_sub(1)
      } else {
        self.line.wrapping_sub(sprite.screen_y())
      };

      if line >= 8 {
        sprite_tile = sprite_tile.wrapping_add(1);
        line -= 8;
      }

      line *= 2;

      if sprite_height == 16 {
        sprite_tile &= 0xFE;
      }

      let single_tile = &self.tile_data[sprite_tile];
      let b1 = single_tile[line as usize];
      let b2 = single_tile[line as usize + 1];

      for x in (0..8).rev() {
        let bit = if sprite.has_xflip() { 7 - x } else { x };

        let color_num = ((b1 >> bit) & 0b1) | ((b2 >> bit) & 0b1) << 1;
        let color: Color = palette.colors[color_num as usize];
        let dest = sprite.screen_x().wrapping_add(7 - x) as usize;
        if dest < SCREEN_WIDTH as usize && (sprite.has_low_priority() || !self.bg_priority[dest]) {
          self.pixels[(self.line as usize) * (SCREEN_WIDTH as usize) + dest] = color.pixel();
        }
      }
    }
  }
}
