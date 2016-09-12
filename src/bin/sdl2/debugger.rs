use libc;
use linefeed::Reader;
use linefeed::complete::{Completer, Completion};
use linefeed::terminal::Terminal;
use std::rc::Rc;

use gameboy::debugger::Debugger;
use gameboy::cpu::Cpu;

extern "C" {
  pub static stdout: *mut libc::FILE;
  pub static stderr: *mut libc::FILE;
  pub static stdin: *mut libc::FILE;
}

pub fn run_debugger(cpu: Cpu) {
  let mut debugger = Debugger::new(cpu);

  unsafe {
    libc::setbuf(stdout as *mut libc::FILE, 0 as *mut i8);
    libc::setbuf(stderr as *mut libc::FILE, 0 as *mut i8);
    libc::setbuf(stdin as *mut libc::FILE, 0 as *mut i8);
  }

  let mut reader = Reader::new("debugger").unwrap();
  reader.set_completer(Rc::new(CmdCompleter));
  reader.set_prompt("(gameboy) ");

  while let Ok(Some(line)) = reader.read_line() {
    let line = line.trim();
    if line.is_empty() {
      continue;
    }

    reader.add_history(line.to_owned());
    debugger.run_cmd(line.to_owned());
  }
}

struct CmdCompleter;

impl<Term: Terminal> Completer<Term> for CmdCompleter {
  fn complete(&self,
              word: &str,
              reader: &Reader<Term>,
              start: usize,
              _end: usize)
              -> Option<Vec<Completion>> {
    let line = reader.buffer();

    let mut words = line[..start].split_whitespace();

    let cmds = vec!["break",
    "breakpoints",
    "continue",
    "debug",
    "exit",
    "help",
    "set",
    "step",
    "x",];

    match words.next() {
      None => {
        let mut compls = Vec::new();

        for cmd in cmds {
          if cmd.starts_with(word) {
            compls.push(Completion::simple(cmd.to_owned()));
          }
        }

        Some(compls)
      }
      // Some("set") => {
      //  if words.count() == 0 {
      //    let mut res = Vec::new();
      //
      //    for (name, _) in reader.variables() {
      //      if name.starts_with(word) {
      //        res.push(Completion::simple(name.to_owned()));
      //      }
      //    }
      //
      //    Some(res)
      //  } else {
      //    None
      //  }
      // }
      _ => None,
    }
  }
}
