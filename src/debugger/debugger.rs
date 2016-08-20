use libc;

use super::rustyline::error::ReadlineError;
use super::rustyline::Editor;
use std::process::exit;

use super::super::cpu::{Cpu, Reg};
use super::ast::Command;
use super::cmd;
use super::super::system::SystemCtrl;

extern "C" {
  pub static stdout: *mut libc::FILE;
  pub static stderr: *mut libc::FILE;
  pub static stdin: *mut libc::FILE;
}

pub struct Debugger {
  cpu: Cpu,
  breakpoints: Vec<usize>,
  break_after_inst: bool,
}

impl Default for Debugger {
  fn default() -> Debugger {
    Debugger {
      cpu: Cpu::default(),
      breakpoints: vec![],
      break_after_inst: false,
    }
  }
}

impl Debugger {
  pub fn new(cpu: Cpu) -> Debugger {
    Debugger { cpu: cpu, ..Debugger::default() }
  }

  fn step(&mut self, display_instructions: bool) -> bool {
    // let result = panic::catch_unwind(|| {
    //   return self.cpu.step();
    // });
    //
    // let (inst, pc) = match result {
    //   Ok(v) => v,
    //   Err(e) => {
    //     println!("{:?}", e);
    //     return true;
    //   }
    // };

    // if self.break_after_inst {
    //   let pc = self.cpu.pc();
    //
    //   let (inst, pc_at_inst) = self.cpu.step();
    //   if display_instructions {
    //     println!("{:#04x}: {:?}", pc_at_inst, inst);
    //   }
    //
    //   for &b in &self.breakpoints {
    //     if pc as usize == b {
    //       println!("Breakpoint hit @ {:#04x}: {:?}", pc, self.cpu.peek_at(pc));
    //       return true;
    //     }
    //   }
    // } else {
    let (inst, pc_at_inst) = self.cpu.step();

    if display_instructions {
      println!("{:#04x}: {:?}",
               self.cpu.pc(),
               self.cpu.peek_at(self.cpu.pc()));
    }

    for &b in &self.breakpoints {
      if self.cpu.pc() as usize == b {
        println!("Breakpoint hit @ {:#04x}: {:?}",
                 self.cpu.pc(),
                 self.cpu.peek_at(self.cpu.pc()));
        return true;
      }
    }
    // }

    false
  }

  pub fn run(&mut self) {
    unsafe {
      libc::setbuf(stdout as *mut libc::FILE, 0 as *mut i8);
      libc::setbuf(stderr as *mut libc::FILE, 0 as *mut i8);
      libc::setbuf(stdin as *mut libc::FILE, 0 as *mut i8);
    }

    let mut rl = Editor::<()>::new();
    if let Err(_) = rl.load_history("history.txt") {
      println!("No previous history.");
    }

    loop {
      let readline = rl.readline("(gameboy) ");
      match readline {
        Ok(line) => {
          if line.is_empty() {
            continue;
          }

          rl.add_history_entry(&line);

          let c = match cmd::parse_cmd(&line) {
            Ok(c) => c,
            Err(e) => {
              println!("Unable to parse \"{}\": {:?}", line, e);
              continue;
            }
          };

          match c {
            Command::Continue => {
              loop {
                if self.step(false) {
                  break;
                }
              }
            }
            Command::Debug => {
              println!("{:?}", self.cpu);
            }
            Command::Set(s, v) => {
              match s.as_str() {
                "a" => self.cpu.write_reg_u8(Reg::A, v as u8),
                "f" => self.cpu.write_reg_u8(Reg::F, v as u8),
                "b" => self.cpu.write_reg_u8(Reg::B, v as u8),
                "c" => self.cpu.write_reg_u8(Reg::C, v as u8),
                "d" => self.cpu.write_reg_u8(Reg::D, v as u8),
                "e" => self.cpu.write_reg_u8(Reg::E, v as u8),
                "h" => self.cpu.write_reg_u8(Reg::H, v as u8),
                "l" => self.cpu.write_reg_u8(Reg::L, v as u8),
                "af" => self.cpu.write_reg_u16(Reg::AF, v as u16),
                "bc" => self.cpu.write_reg_u16(Reg::BC, v as u16),
                "de" => self.cpu.write_reg_u16(Reg::DE, v as u16),
                "hl" => self.cpu.write_reg_u16(Reg::HL, v as u16),
                _ => {}
              }
            }
            Command::Step(s) => {
              match s {
                Some(s) => {
                  for _ in 0..s {
                    if self.step(true) {
                      break;
                    }
                  }
                }
                None => {
                  self.step(true);
                }
              };
            }
            Command::Print(addr) => {
              let d = self.cpu.read_u8(addr as u16);
              println!("{:#04x}", d);
            }
            Command::Breakpoint(l) => {
              self.breakpoints.push(l as usize);
              println!("Added breakpoint @ {:#04x}", l);
            }
            Command::Breakpoints => {
              for loc in &self.breakpoints {
                println!("Breakpoint @ {:#04x}", loc);
              }
            }
            Command::Exit => exit(0),
          };
        }
        Err(ReadlineError::Interrupted) => {
          println!("CTRL-C");
          break;
        }
        Err(ReadlineError::Eof) => {
          println!("CTRL-D");
          break;
        }
        Err(err) => {
          println!("Error: {:?}", err);
          break;
        }
      }
    }

    rl.save_history("history.txt").unwrap();
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {}
}
