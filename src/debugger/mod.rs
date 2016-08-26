extern crate rustyline;

use libc;

use self::rustyline::error::ReadlineError;
use self::rustyline::Editor;
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use ctrlc;
use clap::{App, SubCommand, Arg, AppSettings, ArgMatches};
use term_grid::{Grid, GridOptions, Direction, Filling};
use terminal_size::{Width, terminal_size};

use super::cpu::{Cpu, Reg};

macro_rules! parse_num {
  ($n:expr, $default:expr) => {
    match $n {
      Some(v) => {
        if v.starts_with("0x") {
          match usize::from_str_radix(&v[2..], 16) {
            Ok(v) => v,
            Err(e) => {
              println!("hex parse: {}", e);
              return;
            }
          }
        } else {
          match v.parse::<usize>() {
            Ok(v) => v,
            Err(e) => {
              println!("usize parse: {}", e);
              return;
            }
          }
        }
      }
      None => {
        $default
      }
    }
  };
  ($n:expr) => {
    parse_num!($n, 0)
  }
}

fn debugger_app<'a, 'b>() -> App<'a, 'b> {
  App::new("Gameboy-rs debugger")
    .usage("<SUBCOMMAND>")
    .setting(AppSettings::VersionlessSubcommands)
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .setting(AppSettings::NoBinaryName)
    .subcommand(SubCommand::with_name("continue")
      .visible_alias("c")
      .about("Controls testing features")
      .arg(Arg::with_name("print")
        .short("p")
        .help("Print assembly at each step")))
    .subcommand(SubCommand::with_name("debug")
      .visible_alias("d")
      .about("Prints out debug information"))
    .subcommand(SubCommand::with_name("set")
      .about("Sets a variable to a value")
      .arg(Arg::with_name("var")
        .help("The variable to receive the value")
        .required(true)
        .index(1))
      .arg(Arg::with_name("value")
        .help("The value to set the variable to")
        .required(true)
        .index(2)))
    .subcommand(SubCommand::with_name("step")
      .visible_alias("s")
      .about("Steps the program")
      .arg(Arg::with_name("n")
        .help("How many steps to perform")
        .index(1)))
    .subcommand(SubCommand::with_name("x")
      .about("Prints the value at the memory address")
      .arg(Arg::with_name("size")
        .short("s")
        .possible_values(&["b", "h", "w", "g"])
        .help("Size to print: b(byte), h(halfword), w(word), g(giant, 8 bytes)")
        .takes_value(true))
      .arg(Arg::with_name("count")
        .short("c")
        .help("Number of consecutive memory values to print")
        .takes_value(true))
      .arg(Arg::with_name("format")
        .short("f")
        .possible_values(&["x", "d", "u", "b", "f", "c"])
        .help("Format to print: x(hex), d(decimal), u(unsigned decimal), b(binary), \
               f(float), c(char)")
        .takes_value(true))
      .arg(Arg::with_name("addr")
        .help("The expression (only supports numbers for now)")
        .index(1)))
    .subcommand(SubCommand::with_name("break")
      .visible_alias("b")
      .about("Breaks at the expression")
      .arg(Arg::with_name("expr")
        .help("The expression (only supports numbers for now)")
        .index(1)
        .required(true)))
    .subcommand(SubCommand::with_name("breakpoints")
      .visible_alias("bp")
      .about("Prints out all the breakpoints"))
    .subcommand(SubCommand::with_name("exit")
      .visible_alias("quit")
      .about("Exits the debugger"))
}

extern "C" {
  pub static stdout: *mut libc::FILE;
  pub static stderr: *mut libc::FILE;
  pub static stdin: *mut libc::FILE;
}

pub struct Debugger {
  cpu: Cpu,
  breakpoints: Vec<usize>,
  signal: Arc<AtomicBool>,
}

impl Default for Debugger {
  fn default() -> Debugger {
    Debugger {
      cpu: Cpu::default(),
      breakpoints: vec![],
      signal: Arc::new(AtomicBool::new(false)),
    }
  }
}

impl Debugger {
  pub fn new(cpu: Cpu) -> Debugger {
    Debugger { cpu: cpu, ..Debugger::default() }
  }

  fn step(&mut self, display_instructions: bool) -> bool {
    self.cpu.step();

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

    let signal_clone = self.signal.clone();
    ctrlc::set_handler(move || {
      signal_clone.store(true, Ordering::SeqCst);
    });

    let mut rl = Editor::<()>::new();
    if let Err(_) = rl.load_history("history.txt") {
      println!("No previous history.");
    }

    let mut app = debugger_app();

    loop {
      let readline = rl.readline("(gameboy) ");
      let line = match readline {
        Ok(line) => {
          if line.is_empty() {
            continue;
          }
          line
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
      };

      rl.add_history_entry(&line);

      let argv: Vec<_> = line.trim().split(' ').collect();
      let m = match app.get_matches_from_safe_borrow(argv) {
        Ok(matches) => matches,
        Err(e) => {
          println!("{}", e);
          continue;
        }
      };

      match m.subcommand() {
        ("continue", Some(sub_m)) => {
          self.cmd_continue(sub_m);
        }
        ("debug", Some(_)) => {
          println!("{:?}", self.cpu);
        }
        ("set", Some(sub_m)) => {
          self.cmd_set(sub_m);
        }
        ("step", Some(sub_m)) => {
          let n = parse_num!(sub_m.value_of("n"), 1);

          for _ in 0..n {
            if self.step(true) {
              break;
            }
          }
        }
        ("x", Some(sub_m)) => {
          self.cmd_x(sub_m);
        }
        ("break", Some(sub_m)) => {
          let b = parse_num!(sub_m.value_of("expr"));
          self.breakpoints.push(b);
          println!("Added breakpoint @ {:#04x}", b);
        }
        ("breakpoints", Some(_)) => {
          for (i, loc) in self.breakpoints.iter().enumerate() {
            println!("{:02}: {:#06x}", i, loc);
          }
        }
        ("exit", Some(_)) => {
          exit(0);
        }
        (t, Some(_)) => {
          println!("Unknown command: {}", t);
        }
        _ => {
          continue;
        }
      };
    }

    rl.save_history("history.txt").unwrap();
  }

  fn cmd_continue<'a>(&mut self, sub_m: &ArgMatches<'a>) {
    loop {
      if self.signal.load(Ordering::SeqCst) {
        println!("Got SIGINT. Breaking.");
        self.signal.store(false, Ordering::SeqCst);
        break;
      }

      if self.step(sub_m.is_present("print")) {
        break;
      }
    }
  }

  fn cmd_set<'a>(&mut self, sub_m: &ArgMatches<'a>) {
    let var = sub_m.value_of("var").unwrap();
    let val = parse_num!(sub_m.value_of("value"));

    match var {
      "a" => self.cpu.write_reg_u8(Reg::A, val as u8),
      "f" => self.cpu.write_reg_u8(Reg::F, val as u8),
      "b" => self.cpu.write_reg_u8(Reg::B, val as u8),
      "c" => self.cpu.write_reg_u8(Reg::C, val as u8),
      "d" => self.cpu.write_reg_u8(Reg::D, val as u8),
      "e" => self.cpu.write_reg_u8(Reg::E, val as u8),
      "h" => self.cpu.write_reg_u8(Reg::H, val as u8),
      "l" => self.cpu.write_reg_u8(Reg::L, val as u8),
      "af" => self.cpu.write_reg_u16(Reg::AF, val as u16),
      "bc" => self.cpu.write_reg_u16(Reg::BC, val as u16),
      "de" => self.cpu.write_reg_u16(Reg::DE, val as u16),
      "hl" => self.cpu.write_reg_u16(Reg::HL, val as u16),
      _ => {
        println!("Unknown variable: {}", var);
      }
    };
  }

  fn cmd_x<'a>(&mut self, sub_m: &ArgMatches<'a>) {
    let mut grid = Grid::new(GridOptions {
      filling: Filling::Spaces(1),
      direction: Direction::LeftToRight,
    });

    let size = match sub_m.value_of("size").unwrap_or("b") {
      "b" => 1, // byte
      "h" => 2, // half word (2 bytes)
      "w" => 4, // word (4 bytes)
      "g" => 8, // giant (8 bytes)
      _ => unreachable!(),
    };

    let format: Box<Fn(usize) -> String> = match sub_m.value_of("format").unwrap_or("x") {
      "x" => Box::new(|v| format!("0x{1:00$x}", size * 2, v)),
      "d" => Box::new(|v| format!("{}", v as isize)),
      "u" => Box::new(|v| format!("{}", v as usize)),
      "b" | "t" => Box::new(|v| format!("{1:00$b}", size * 8, v)),
      "f" => Box::new(|v| format!("{}", v as f64)),
      "c" => Box::new(|v| format!("{}", v as u8 as char)),
      _ => unreachable!(),
    };

    let addr = parse_num!(sub_m.value_of("addr"));
    let count = parse_num!(sub_m.value_of("count"), 1);
    for i in 0..count {
      let mut val = 0usize;

      for s in 0..size {
        match self.cpu.read_u8_safe(addr as u16 + s as u16 + (i as u16 * size as u16)) {
          Ok(d) => {
            val |= (d as usize) << ((size - 1 - s) * 8);
          }
          Err(_) => break,
        }
      }

      grid.add(format(val).into());
    }

    // FIXME: fixes a bug in term_grid. Remove when fixed.
    if count == 1 {
      grid.add("".into());
    }

    let width = match terminal_size() {
      Some((Width(w), _)) => w as usize,
      None => 50,
    };

    // Fit the grid into the terminal width minus 8 for the address
    // at the beginning of the line.
    if let Some(grid_display) = grid.fit_into_width(width - 8) {
      let g = format!("{}", grid_display);
      let g: Vec<_> = g.split('\n').collect();
      for (i, row) in g.iter().enumerate() {
        if row.is_empty() {
          continue;
        }

        // Find out the number of cols and calculate what the address
        // should be for the given line.
        let cols = row.matches(' ').count() + 1;
        let a = addr + i * cols * size;
        println!("{:#06x}:  {}", a, row);
      }
    } else {
      println!("Couldn't fit grid!");
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {}
}
