
#[derive(Default)]
pub struct CPU {
  reg_af: u16, // Accumulator and flags
  reg_bc: u16, // General purpose
  reg_de: u16, // General purpose
  reg_hl: u16, // General purpose

  reg_sp: u16, // Stack pointer
  reg_pc: u16, // Program counter
}

impl CPU {
  pub fn new() -> CPU {
    CPU::default()
  }

  pub fn step(&self) {
    println!("step");
  }
}
