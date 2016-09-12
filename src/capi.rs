use std::fs::File;
use std::io::Read;
use std::path::Path;
use libc::{int8_t, uint8_t, c_char};
use std::thread;
use std::ffi::CStr;

use super::cpu::Cpu;
use super::system;
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
pub unsafe extern "C" fn gb_new(cart_path: *const c_char) -> *mut Gameboy {
  let cart_path = CStr::from_ptr(cart_path).to_str().unwrap();

  let system = system::Config::new()
    .cart_rom(load_rom(cart_path))
    .create()
    .unwrap();

  let mut cpu = Cpu::new(system);
  cpu.bootstrap();

  Box::into_raw(Box::new(Gameboy { cpu: cpu }))
}

#[no_mangle]
pub unsafe extern "C" fn gb_run_threaded(gb: *mut Gameboy) {
  let mut gb = {
    assert!(!gb.is_null());
    &mut *gb
  };

  thread::spawn(move || {
    loop {
      gb.cpu.step();
    }
  });
}

#[no_mangle]
pub unsafe extern "C" fn gb_updated_frame(gb: *mut Gameboy, dst: *mut uint8_t) -> int8_t {
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

  0
}

#[no_mangle]
pub unsafe extern "C" fn gb_set_button(gb: *mut Gameboy, btn: uint8_t, pressed: bool) {
  let mut gb = {
    assert!(!gb.is_null());
    &mut *gb
  };

  gb.cpu.set_button(Button::from_u8(btn as u8), pressed);
}

#[no_mangle]
pub unsafe extern "C" fn gb_drop(gb: *mut Gameboy) {
  if gb.is_null() {
    return;
  }
  Box::from_raw(gb);
}
