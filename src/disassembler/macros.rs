#[macro_export]
macro_rules! imm {
  ($inst:path[imm8], $m:ident, $addr:ident, $pc:ident) => ({
    let n = try!($m.read_u8($addr + $pc));
    $pc += 1;
    Ok(($inst(Operand::Imm8(n)), $pc))
  });

  ($inst:path[$operand1:path, imm8], $m:ident, $addr:ident, $pc:ident) => ({
    let n = try!($m.read_u8($addr + $pc));
    $pc += 1;
    Ok(($inst($operand1, Operand::Imm8(n)), $pc))
  });

  ($inst:path[imm8, $operand2:path], $m:ident, $addr:ident, $pc:ident) => ({
    let n = try!($m.read_u8($addr + $pc));
    $pc += 1;
    Ok(($inst(Operand::Imm8(n), $operand2), $pc))
  });

  ($inst:path[imm16], $m:ident, $addr:ident, $pc:ident) => ({
    let nn = try!($m.read_u16($addr + $pc));
    $pc += 2;
    Ok(($inst(Operand::Imm16(nn)), $pc))
  });

  ($inst:path[$operand1:path, imm16], $m:ident, $addr:ident, $pc:ident) => ({
    let nn = try!($m.read_u16($addr + $pc));
    $pc += 2;
    Ok(($inst($operand1, Operand::Imm16(nn)), $pc))
  });

  ($inst:path[imm16, $operand2:path], $m:ident, $addr:ident, $pc:ident) => ({
    let nn = try!($m.read_u16($addr + $pc));
    $pc += 2;
    Ok(($inst(Operand::Imm16(nn), $operand2), $pc))
  });
}
