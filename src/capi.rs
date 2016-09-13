use std::fs::File;
use std::io::Read;
use std::path::Path;
use libc::{int8_t, uint8_t, c_char};
use std::thread;
use std::ffi::CStr;
use std::ptr;
use std::cmp;
use std::result;

use super::cpu::Cpu;
use super::system;
use super::gamepad::Button;
use super::debugger::Debugger;

const MAX_ERROR_SIZE: usize = 1024;

enum CApiErrors {
  Generic = 0,
}

macro_rules! try_api {
  ($err:expr, $err_ret:expr, $expr:expr) => (match $expr {
    result::Result::Ok(val) => val,
    result::Result::Err(err) => {
      println!("{}", err);
      //let s = err.to_string();
      //let p = s.as_ptr();
      //mem::forget(s);
      //return p;
      let mut e = {
        assert!(!$err.is_null());
        &mut *$err
      };
      e.code = CApiErrors::Generic as uint8_t;
      let s = err.to_string();
      let p = s.as_ptr() as *mut u8;
      ptr::copy_nonoverlapping(p, e.error, cmp::max(s.len(), MAX_ERROR_SIZE));
      //return mem::transmute($err_ret as *const u64);
      $err_ret;
    }
  })
}

#[repr(C)]
pub struct CApiError {
  code: uint8_t,
  error: *mut u8,
}

#[repr(C)]
#[derive(Debug)]
pub struct CApiGameboy {
  cpu: Cpu,
}

fn load_rom<P: AsRef<Path>>(path: P) -> Result<Box<[u8]>, String> {
  let mut file = match File::open(path) {
    Ok(f) => f,
    Err(e) => return Err(format!("{}", e)),
  };
  let mut file_buf = Vec::new();
  match file.read_to_end(&mut file_buf) {
    Ok(_) => (),
    Err(e) => return Err(format!("{}", e)),
  };
  Ok(file_buf.into_boxed_slice())
}

#[no_mangle]
pub unsafe extern "C" fn gb_new() -> *mut CApiGameboy {

  let system = system::System::new();
  let cpu = Cpu::new(Box::new(system));

  Box::into_raw(Box::new(CApiGameboy { cpu: cpu }))
}

#[no_mangle]
pub unsafe extern "C" fn gb_load_cartridge(gb: *mut CApiGameboy,
                                           cart_path: *const c_char,
                                           err_out: *mut CApiError) {
  let mut gb = {
    assert!(!gb.is_null());
    &mut *gb
  };
  let cart_path = try_api!(err_out, return, CStr::from_ptr(cart_path).to_str());
  let rom = try_api!(err_out, return, load_rom(cart_path));
  try_api!(err_out, return, gb.cpu.system.load_cartridge(rom));
  gb.cpu.bootstrap();
}

#[no_mangle]
pub unsafe extern "C" fn gb_run_threaded(gb: *mut CApiGameboy) {
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
pub unsafe extern "C" fn gb_updated_frame(gb: *mut CApiGameboy, dst: *mut uint8_t) -> int8_t {
  let mut gb = {
    assert!(!gb.is_null());
    &mut *gb
  };

  if let Some(d) = gb.cpu.system.updated_frame() {
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
pub unsafe extern "C" fn gb_set_button(gb: *mut CApiGameboy, btn: uint8_t, pressed: bool) {
  let mut gb = {
    assert!(!gb.is_null());
    &mut *gb
  };

  gb.cpu.system.set_button(Button::from_u8(btn as u8), pressed);
}

#[no_mangle]
pub unsafe extern "C" fn gb_drop(gb: *mut CApiGameboy) {
  if gb.is_null() {
    return;
  }
  Box::from_raw(gb);
}

pub struct CApiDebugger<'a, 'b>
  where 'a: 'b
{
  debugger: Debugger<'a, 'b>,
}

#[no_mangle]
pub unsafe extern "C" fn gb_dbg_new<'a, 'b>() -> *mut CApiDebugger<'a, 'b> {
  let system = system::System::new();
  let cpu = Cpu::new(Box::new(system));

  Box::into_raw(Box::new(CApiDebugger { debugger: Debugger::new(cpu) }))
}

#[no_mangle]
pub unsafe extern "C" fn gb_dbg_load_cartridge(dbg: *mut CApiDebugger,
                                               cart_path: *const c_char,
                                               err_out: *mut CApiError) {
  let mut dbg = {
    assert!(!dbg.is_null());
    &mut *dbg
  };

  let cart_path = try_api!(err_out, return, CStr::from_ptr(cart_path).to_str());
  let rom = try_api!(err_out, return, load_rom(cart_path));
  try_api!(err_out, return, dbg.debugger.cpu.system.load_cartridge(rom));
  dbg.debugger.cpu.bootstrap();
}

#[no_mangle]
pub unsafe extern "C" fn gb_dbg_drop(dbg: *mut CApiDebugger) {
  if dbg.is_null() {
    return;
  }
  Box::from_raw(dbg);
}
