fn test_cpu(name: String, v: CpuDataTest) {
  let mut c = Cpu::default();
  c.write_reg_u8(Reg::A, v.pre.A);
  c.write_reg_u8(Reg::F, v.pre.F);
  c.write_reg_u8(Reg::B, v.pre.B);
  c.write_reg_u8(Reg::C, v.pre.C);
  c.write_reg_u8(Reg::D, v.pre.D);
  c.write_reg_u8(Reg::E, v.pre.E);
  c.write_reg_u8(Reg::H, v.pre.H);
  c.write_reg_u8(Reg::L, v.pre.L);
  c.reg_sp = v.pre.SP;
  c.reg_pc = v.pre.PC;
  for (map_k, map_v) in v.pre.mem {
    c.system
      .write_u8(map_k.parse::<u16>().unwrap(), map_v)
      .unwrap();
  }

  c.step();

  let (v1, v2) = (c.read_reg_u8(Reg::A), v.post.A);
  assert!(v1 == v2,
          "\n{0}:\n A:\n  Got:      {1:#04x} [{1:08b}],\n  Expected: {2:#04x} [{2:08b}]",
          test_name,
          v1,
          v2);

  let (v1, v2) = (c.read_reg_u8(Reg::F), v.post.F);
  let mut flags1 = vec![];
  if v1 & 0b10000000 != 0 {
    flags1.push("Z");
  }
  if v1 & 0b01000000 != 0 {
    flags1.push("N");
  }
  if v1 & 0b00100000 != 0 {
    flags1.push("H");
  }
  if v1 & 0b00010000 != 0 {
    flags1.push("C");
  }
  let mut flags2 = vec![];
  if v2 & 0b10000000 != 0 {
    flags2.push("Z");
  }
  if v2 & 0b01000000 != 0 {
    flags2.push("N");
  }
  if v2 & 0b00100000 != 0 {
    flags2.push("H");
  }
  if v2 & 0b00010000 != 0 {
    flags2.push("C");
  }
  assert!(v1 == v2,
          "\n{0}:\n F:\n  Got:      {1:#04x} [{2:}],\n  Expected: {3:#04x} [{4:}]",
          test_name,
          v1,
          flags1.join(""),
          v2,
          flags2.join(""));

  let (v1, v2) = (c.read_reg_u8(Reg::B), v.post.B);
  assert!(v1 == v2,
          "\n{0}:\n B:\n  Got:      {1:#04x} [{1:08b}],\n  Expected: {2:#04x} [{2:08b}]",
          test_name,
          v1,
          v2);

  let (v1, v2) = (c.read_reg_u8(Reg::C), v.post.C);
  assert!(v1 == v2,
          "\n{0}:\n C:\n  Got:      {1:#04x} [{1:08b}],\n  Expected: {2:#04x} [{2:08b}]",
          test_name,
          v1,
          v2);

  let (v1, v2) = (c.read_reg_u8(Reg::D), v.post.D);
  assert!(v1 == v2,
          "\n{0}:\n D:\n  Got:      {1:#04x} [{1:08b}],\n  Expected: {2:#04x} [{2:08b}]",
          test_name,
          v1,
          v2);

  let (v1, v2) = (c.read_reg_u8(Reg::E), v.post.E);
  assert!(v1 == v2,
          "\n{0}:\n E:\n  Got:      {1:#04x} [{1:08b}],\n  Expected: {2:#04x} [{2:08b}]",
          test_name,
          v1,
          v2);

  let (v1, v2) = (c.read_reg_u8(Reg::H), v.post.H);
  assert!(v1 == v2,
          "\n{0}:\n H:\n  Got:      {1:#04x} [{1:08b}],\n  Expected: {2:#04x} [{2:08b}]",
          test_name,
          v1,
          v2);
  let (v1, v2) = (c.read_reg_u8(Reg::L), v.post.L);
  assert!(v1 == v2,
          "\n{0}:\n L:\n  Got:      {1:#04x} [{1:08b}],\n  Expected: {2:#04x} [{2:08b}]",
          test_name,
          v1,
          v2);

  let (v1, v2) = (c.read_reg_u16(Reg::SP), v.post.SP);
  assert!(v1 == v2,
          "\n{0}:\n SP:\n  Got:      {1:#04x} [{1:08b}],\n  Expected: {2:#04x} [{2:08b}]",
          test_name,
          v1,
          v2);
  let (v1, v2) = (c.read_reg_u16(Reg::PC), v.post.PC);
  assert!(v1 == v2,
          "\n{0}:\n PC:\n  Got:      {1:#04x} [{1:08b}],\n  Expected: {2:#04x} [{2:08b}]",
          test_name,
          v1,
          v2);

  for (map_k, map_v) in v.post.mem {
    let k = map_k.parse::<u16>().unwrap();
    let (v1, v2) = (c.system.read_u8(k).unwrap(), map_v);
    assert!(v1 == v2,
            "\n{0}\nmem {1}:\n  Got:      {2:#04x},\n  Expected: {3:#04x}",
            test_name,
            k,
            v1,
            v2);
  }
}

% for (k, v) in data.items():
<%
  test_func_name = k.replace(" #", "_")
%>
#[test]
fn test_${test_func_name}() {
  test_cpu("${k}", CpuDataTest{
    pre: CpuData{
      A: ${hex(v['pre']['A'])},
      F: ${hex(v['pre']['F'])},
      B: ${hex(v['pre']['B'])},
      C: ${hex(v['pre']['C'])},
      D: ${hex(v['pre']['D'])},
      E: ${hex(v['pre']['E'])},
      H: ${hex(v['pre']['H'])},
      L: ${hex(v['pre']['L'])},
      SP: ${hex(v['pre']['SP'])},
      PC: ${hex(v['pre']['PC'])},
      mem: hashmap!{
      % for (mem_k, mem_v) in v["pre"]["mem"].items():
        ${hex(int(mem_k))} => ${hex(mem_v)},
      % endfor
      },
    },
    post: CpuData{
      A: ${hex(v['post']['A'])},
      F: ${hex(v['post']['F'])},
      B: ${hex(v['post']['B'])},
      C: ${hex(v['post']['C'])},
      D: ${hex(v['post']['D'])},
      E: ${hex(v['post']['E'])},
      H: ${hex(v['post']['H'])},
      L: ${hex(v['post']['L'])},
      SP: ${hex(v['post']['SP'])},
      PC: ${hex(v['post']['PC'])},
      mem: hashmap!{
      % for (mem_k, mem_v) in v["post"]["mem"].items():
        ${hex(int(mem_k))} => ${hex(mem_v)},
      % endfor
      },
    },
  });
}
% endfor
