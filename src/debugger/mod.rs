extern crate rustyline;
extern crate shlex;

use self::rustyline::error::ReadlineError;
use self::rustyline::Editor;

use gameboy::cpu;

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
          let mut sh = shlex::Shlex::new(&line);
          match sh.nth(0).unwrap().as_ref() {
            "step" | "s" => {
              let steps: usize = match sh.nth(0) {
                Some(s) => s.parse().unwrap(),
                None => 1,
              };

              for _ in 0..steps {
                self.cpu.step();
              }
            }
            _ => {
              println!("unknown command: {}", line);
            }
          }
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
