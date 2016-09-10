use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::mem;
use libc::{int8_t, uint8_t, c_char};
use std::thread;
use std::ffi::CStr;

use super::cpu::Cpu;
use super::system::{self, System};
use super::gamepad::Button;

#[repr(C)]
#[derive(Debug)]
pub struct Gameboy {
  cpu: Cpu,
}

fn load_rom<P: AsRef<Path>>(path: P) -> Box<[u8]> {
  let mut file = File::open(path).unwrap();
  let mut file_buf = Vec::new();
  file.read_to_end(&mut file_buf).unwrap();
  file_buf.into_boxed_slice()
}

#[no_mangle]
pub extern "C" fn gameboy_new(cart_path: *const c_char) -> *mut Gameboy {
  let cart_path = unsafe { CStr::from_ptr(cart_path).to_str().unwrap() };

  let system = system::Config::new()
    //.boot_rom(Some(load_rom("./res/DMG_ROM.bin")))
    //.cart_rom(load_rom("../../../res/opus5.gb"))
    .cart_rom(load_rom(cart_path))
    .create()
    .unwrap();

  let mut cpu = Cpu::new(system);
  cpu.bootstrap();

  Box::into_raw(Box::new(Gameboy { cpu: cpu }))
}

// #[no_mangle]
// pub unsafe extern "C" fn gameboy_step(gb: *mut Gameboy) {
//   let mut gb = {
//     assert!(!gb.is_null());
//     &mut *gb
//   };

//   gb.step();
// }

#[no_mangle]
pub unsafe extern "C" fn gameboy_run_threaded(gb: *mut Gameboy) {
  let mut gb = {
    assert!(!gb.is_null());
    &mut *gb
  };

  thread::spawn(move || {
    // use std::time::{Instant, Duration};
    // let mut hz = 0;
    // let mut now = Instant::now();
    loop {
      gb.cpu.step();
      // hz += 1;
      // if Instant::now().duration_since(now).as_secs() >= 1 {
      //   println!("{} hz", hz);
      //   hz = 0;
      //   now = Instant::now();
      // }
    }
  });
}

#[no_mangle]
pub unsafe extern "C" fn gameboy_updated_frame(gb: *mut Gameboy, dst: *mut uint8_t) -> int8_t {
  let mut gb = {
    assert!(!gb.is_null());
    &mut *gb
  };

  if let Some(d) = gb.cpu.updated_frame() {
    for (i, v) in d.iter().enumerate() {
      *dst.offset(i as isize * 4) = v[0];
      *dst.offset(i as isize * 4 + 1) = v[1];
      *dst.offset(i as isize * 4 + 2) = v[2];
      *dst.offset(i as isize * 4 + 3) = v[3];
    }
    return 1;
  }

  return 0;
}

#[no_mangle]
pub unsafe extern "C" fn gameboy_set_button(gb: *mut Gameboy, btn: uint8_t, pressed: bool) {
  let mut gb = {
    assert!(!gb.is_null());
    &mut *gb
  };

  gb.cpu.set_button(Button::from_u8(btn as u8), pressed);
}

#[no_mangle]
pub unsafe extern "C" fn gameboy_drop(gb: *mut Gameboy) {
  if gb.is_null() {
    return;
  }
  Box::from_raw(gb);
}
