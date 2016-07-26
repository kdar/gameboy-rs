// 16KB ROM Bank 00
// In cartridge, fixed at bank 00
const ROM_00_START: u16 = 0x0000;
const ROM_00_END: u16 = 0x3FFF;

// 16KB ROM Bank 01..NN
// In cartridge, switable bank number
const ROM_01_START: u16 = 0x4000;
const ROM_01_END: u16 = 0x7FFF;

// Swichable bank 0-1 in CGB Mode
const VIDEO_RAM_START: u16 = 0x8000;
const VIDEO_RAM_END: u16 = 0x9FFF;

// In cartridge, switchable bank, if any
const EXTERNAL_RAM_START: u16 = 0xA000;
const EXTERNAL_RAM_END: u16 = 0xBFFF;

// 4KB work RAM bank 0 (WRAM)
const WORK_RAM_0_START: u16 = 0xC000;
const WORK_RAM_0_END: u16 = 0xCFFF;
pub const WORK_RAM_0_LEN: usize = WORK_RAM_0_END as usize - WORK_RAM_0_START as usize;

// 4KB Work RAM Bank 1 (WRAM)
// switchable bank 1-7 in CGB Mode
const WORK_RAM_1_START: u16 = 0xD000;
const WORK_RAM_1_END: u16 = 0xDFFF;
pub const WORK_RAM_1_LEN: usize = WORK_RAM_1_END as usize - WORK_RAM_1_START as usize;

// Same as C000-DDFF (ECHO)
// typically not used
const ECHO_START: u16 = 0xE000;
const ECHO_END: u16 = 0xFDFF;

// Sprite attribute table (OAM)
const SPRITE_TABLE_START: u16 = 0xFE00;
const SPRITE_TABLE_END: u16 = 0xFE9F;

// Not usable
const UNUSABLE_START: u16 = 0xFEA0;
const UNUSABLE_END: u16 = 0xFEFF;

// I/O Ports (gamepad buttons)
const IO_PORTS_START: u16 = 0xFF00;
const IO_PORTS_END: u16 = 0xFF7F;

// High RAM (HRAM)
const HIGH_RAM_START: u16 = 0xFF80;
const HIGH_RAM_END: u16 = 0xFFFE;

// Interrupt Enable Register
const INTERRUPT_REGISTER_START: u16 = 0xFFFF;
const INTERRUPT_REGISTER_END: u16 = 0xFFFF;

#[derive(Debug)]
pub enum Addr {
  Rom00(u16),
  Rom01(u16),
  VideoRam(u16),
  ExternalRam(u16),
  WorkRam0(u16),
  WorkRam1(u16),
  SpriteTable(u16),
  IoPorts(u16),
  HighRam(u16),
  InterruptRegister,
}

pub fn memory_map(addr: u16) -> Addr {
  match addr {
    ROM_00_START...ROM_00_END => Addr::Rom00(addr - ROM_00_START),
    ROM_01_START...ROM_01_END => Addr::Rom01(addr - ROM_01_START),
    VIDEO_RAM_START...VIDEO_RAM_END => Addr::VideoRam(addr - VIDEO_RAM_START),
    EXTERNAL_RAM_START...EXTERNAL_RAM_END => Addr::ExternalRam(addr - EXTERNAL_RAM_START),
    WORK_RAM_0_START...WORK_RAM_0_END => Addr::WorkRam0(addr - WORK_RAM_0_START),
    WORK_RAM_1_START...WORK_RAM_1_END => Addr::WorkRam1(addr - WORK_RAM_1_START),
    ECHO_START...ECHO_END => memory_map(addr - ECHO_START + WORK_RAM_0_START),
    SPRITE_TABLE_START...SPRITE_TABLE_END => Addr::SpriteTable(addr - SPRITE_TABLE_START),
    UNUSABLE_START...UNUSABLE_END => panic!("unusable memory area!"),
    IO_PORTS_START...IO_PORTS_END => Addr::IoPorts(addr - IO_PORTS_START),
    HIGH_RAM_START...HIGH_RAM_END => Addr::HighRam(addr - HIGH_RAM_START),
    INTERRUPT_REGISTER_START...INTERRUPT_REGISTER_END => Addr::InterruptRegister,
    _ => {
      panic!("unrecognized memory mapped region");
    }
  }
}
