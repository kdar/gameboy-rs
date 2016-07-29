use super::rustyline::error::ReadlineError;
use super::rustyline::Editor;
use std::process::exit;

use super::super::cpu;
use super::command::Command;

pub struct Debugger {
  cpu: cpu::Cpu,
}

impl Debugger {
  pub fn new(cart_rom: Box<[u8]>) -> Debugger {
    let mut cpu = cpu::Cpu::new();
    cpu.set_cart_rom(cart_rom);
    Debugger { cpu: cpu }
  }

  pub fn set_boot_rom(&mut self, rom: Box<[u8]>) {
    self.cpu.set_boot_rom(rom);
  }

  pub fn run(&mut self) {
    let mut rl = Editor::<()>::new();
    if let Err(_) = rl.load_history("history.txt") {
      println!("No previous history.");
    }

    loop {
      let readline = rl.readline("gameboy> ");
      match readline {
        Ok(line) => {
          if line.len() == 0 {
            continue;
          }

          rl.add_history_entry(&line);

          let c = match Command::parse(&line) {
            Ok(c) => c,
            Err(_) => {
              println!("Unable to parse: {}", line);
              continue;
            }
          };

          match c {
            Command::Step(s) => {
              for _ in 0..s {
                self.cpu.step();
              }
            }
            Command::Breakpoint(l) => {}
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
