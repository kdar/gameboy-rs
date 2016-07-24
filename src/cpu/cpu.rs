
#[derive(Default)]
pub struct CPU {
  reg_af: u16,
  reg_bc: u16,
  reg_de: u16,
  reg_hl: u16,
  reg_sp: u16,
  reg_pc: u16,
}

impl CPU {
  pub fn new() -> CPU {
    CPU::default()
  }

  fn run() {}
}
