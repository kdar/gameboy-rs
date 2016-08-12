#[macro_export]
macro_rules! imm {
  ($inst:path[imm8], $m:ident, $addr:ident, $pc:ident) => ({
    let n = try!($m.read_u8($addr + $pc));
    $pc += 1;
    Ok(($inst(Operand::Imm(Imm::Imm8(n))), $pc))
  });

  ($inst:path[$operand1:expr, imm8], $m:ident, $addr:ident, $pc:ident) => ({
    let n = try!($m.read_u8($addr + $pc));
    $pc += 1;
    Ok(($inst($operand1, Operand::Imm(Imm::Imm8(n))), $pc))
  });

  ($inst:path[imm8, $operand2:path], $m:ident, $addr:ident, $pc:ident) => ({
    let n = try!($m.read_u8($addr + $pc));
    $pc += 1;
    Ok(($inst(Operand::Imm(Imm::Imm8(n)), $operand2), $pc))
  });

  ($inst:path[imm16], $m:ident, $addr:ident, $pc:ident) => ({
    let nn = try!($m.read_u16($addr + $pc));
    $pc += 2;
    Ok(($inst(Operand::Imm(Imm::Imm16(nn))), $pc))
  });

  ($inst:path[$operand1:expr, imm16], $m:ident, $addr:ident, $pc:ident) => ({
    let nn = try!($m.read_u16($addr + $pc));
    $pc += 2;
    Ok(($inst($operand1, Operand::Imm(Imm::Imm16(nn))), $pc))
  });

  ($inst:path[imm16, $operand2:path], $m:ident, $addr:ident, $pc:ident) => ({
    let nn = try!($m.read_u16($addr + $pc));
    $pc += 2;
    Ok(($inst(Operand::Imm(Imm::Imm16(nn)), $operand2), $pc))
  });

  ($inst:path[$operand1:expr, imm_addr], $m:ident, $addr:ident, $pc:ident) => ({
    let nn = try!($m.read_u16($addr + $pc));
    $pc += 2;
    Ok(($inst($operand1, Operand::Addr(Addr::Imm16(nn))), $pc))
  });

  ($inst:path[imm_addr, $operand2:expr], $m:ident, $addr:ident, $pc:ident) => ({
    let nn = try!($m.read_u16($addr + $pc));
    $pc += 2;
    Ok(($inst(Operand::Addr(Addr::Imm16(nn)), $operand2), $pc))
  });
}
