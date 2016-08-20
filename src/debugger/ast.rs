#[derive(Debug, Clone)]
pub enum Command {
  Breakpoint(usize),
  Breakpoints,
  Continue,
  Debug,
  Print(usize),
  Exit,
  Set(String, usize),
  Step(Option<usize>),
}
