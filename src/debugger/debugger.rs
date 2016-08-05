use super::rustyline::error::ReadlineError;
use super::rustyline::Editor;
use std::process::exit;

use super::super::cpu;
use super::command::Command;

pub struct Debugger {
  cpu: cpu::Cpu,
  breakpoints: Vec<usize>,
}

impl Default for Debugger {
  fn default() -> Debugger {
    Debugger {
      cpu: cpu::Cpu::default(),
      breakpoints: vec![],
    }
  }
}

impl Debugger {
  pub fn new() -> Debugger {
    let cpu = cpu::Cpu::new();
    Debugger { cpu: cpu, ..Debugger::default() }
  }

  pub fn set_cart_rom(&mut self, rom: &[u8]) {
    self.cpu.set_cart_rom(rom);
  }

  pub fn set_boot_rom(&mut self, rom: Box<[u8]>) {
    self.cpu.set_boot_rom(rom);
  }

  fn step(&mut self) -> bool {
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

    let pc = self.cpu.pc();

    let inst = self.cpu.step();
    println!("{:#04x}: {:?}", pc, inst);

    for &b in &self.breakpoints {
      if self.cpu.pc() as usize == b {
        println!("Breakpoint hit @ {:#04x}", pc);
        return true;
      }
    }

    false
  }

  pub fn run(&mut self) {
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

          let c = match Command::parse(&line) {
            Ok(c) => c,
            Err(e) => {
              println!("Unable to parse \"{}\": {}", line, e);
              continue;
            }
          };

          match c {
            Command::Continue => {
              loop {
                if self.step() {
                  break;
                }
              }
            }
            Command::Debug => {
              println!("{:?}", self.cpu);
            }
            Command::Step(s) => {
              for _ in 0..s {
                if self.step() {
                  break;
                }
              }
            }
            Command::Breakpoint(Some(l)) => {
              self.breakpoints.push(l as usize);
              println!("Added breakpoint @ {:#04x}", l);
            }
            Command::Breakpoints => {
              for loc in &self.breakpoints {
                println!("Breakpoint @ {:#04x}", loc);
              }
            }
            Command::Exit => exit(0),
            _ => {}
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
