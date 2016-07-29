use nom::{self, multispace, eof, space, digit};
use std::str::{self, FromStr};
use std::fmt;
use std::error;

#[derive(Debug, Clone, Copy)]
pub enum Command {
  Breakpoint(Option<usize>),
  Breakpoints,
  Continue,
  Debug,
  Exit,
  Step(usize),
}

impl Command {
  pub fn parse(line: &str) -> Result<Command, ParserError> {
    let parsed = command(line.as_bytes());
    match parsed {
      nom::IResult::Done(_, o) => Ok(o),
      nom::IResult::Error(e) => Err(From::from(e)),
      nom::IResult::Incomplete(i) => Err(ParserError(format!("Incomplete: {:?}", i))),
    }
  }
}

#[derive(Debug)]
pub struct ParserError(String);

impl fmt::Display for ParserError {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    write!(fmt, "{}", self.0)
  }
}

impl error::Error for ParserError {
  fn description(&self) -> &str {
    &self.0
  }
}

impl<'a> From<nom::Err<&'a [u8]>> for ParserError {
  fn from(e: nom::Err<&'a [u8]>) -> ParserError {
    ParserError(format!("{:?}", e))
  }
}

named!(command<Command>,
  chain!(
    c: alt_complete!(
        step |
        continue_ |
        breakpoints |
        breakpoint |
        debug |
        exit
    ) ~
    multispace? ~
    eof,
    || c
  )
);

named!(breakpoint<Command>,
  chain!(
    alt_complete!(
      tag!("breakpoint") |
      tag!("break") |
      tag!("b")
    ) ~
    loc: opt!(preceded!(space, usize_parser)),
    || Command::Breakpoint(loc)
  )
);

named!(breakpoints<Command>,
  map!(
    complete!(
      tag!("breakpoints")
    ),
    |_| Command::Breakpoints
  )
);

named!(continue_<Command>,
  map!(
    alt_complete!(
      tag!("continue") |
      tag!("c")
    ),
    |_| Command::Continue
  )
);

named!(debug<Command>,
  map!(
    alt_complete!(
      tag!("debug") |
      tag!("d")
    ),
    |_| Command::Debug
  )
);

named!(exit<Command>,
  map!(
    alt_complete!(
      tag!("exit") |
      tag!("quit") |
      tag!("e") |
      tag!("q")
    ),
    |_| Command::Exit
  )
);

named!(step<Command>,
  chain!(
    alt_complete!(
      tag!("step") |
      tag!("s")
    ) ~
    count: opt!(preceded!(space, usize_parser)),
    || Command::Step(count.unwrap_or(1))
  )
);

named!(usize_parser<usize>,
  map_res!(
    map_res!(
      digit,
      str::from_utf8
    ),
    FromStr::from_str
  )
);
