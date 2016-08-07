#!python3.5

import pexpect
import re
import sys

def flags(value):
  flags = []
  if value&0x80 != 0:
  	flags.append("Z")
  if value&0x40 != 0:
  	flags.append("N")
  if value&0x20 != 0:
  	flags.append("H")
  if value&0x10 != 0:
  	flags.append("C")
  return "".join(flags)

child1 = pexpect.spawn('target/debug/gameboy-emu -b "res/DMG_ROM.bin" "res/cpu_instrs/individual/11-op a,(hl).gb" --debug')
child2 = pexpect.spawn('/home/outroot/build/realboy-0.2.2/src/realboy -d "res/cpu_instrs/individual/11-op a,(hl).gb"')

child1.expect('(gameboy)')
child1.sendline('b c24c')
child1.expect('(gameboy)')
child1.sendline('c')
child1.expect('(gameboy)')

child2.expect('gddb>')
child2.sendline('break 0xc24c')
child2.expect('gddb>')
child2.sendline('step 0xFFFFFFFF')
child2.expect('gddb>')

while True:
  child1.sendline('debug')
  child1.expect('(gameboy)')
  regex1 = re.compile('([AFBCDEHLSPC]{2}):\s+0x(....).*?', re.MULTILINE)
  match1 = regex1.findall(str(child1.before))

  child2.sendline('show regs')
  child2.expect('gddb>')
  regex2 = re.compile('([AFBCDEHLSPC]{2}) = 0x(....).*?', re.MULTILINE)
  match2 = regex2.findall(str(child2.before))

  fail = False
  for i in range(len(match1)):
    if match1[i][1] != match2[i][1]:
      fail = True
      break

  if fail:
    print("AF: Got: {}, Expect: {}".format(match1[0][1], match2[0][1]))
    print("BC: Got: {}, Expect: {}".format(match1[1][1], match2[1][1]))
    print("DE: Got: {}, Expect: {}".format(match1[2][1], match2[2][1]))
    print("HL: Got: {}, Expect: {}".format(match1[3][1], match2[3][1]))
    print("SP: Got: {}, Expect: {}".format(match1[4][1], match2[4][1]))
    print("PC: Got: {}, Expect: {}".format(match1[5][1], match2[5][1]))
    f1 = int(match1[0][1], 16)
    f2 = int(match2[0][1], 16)
    print("Flags: Got: {}, Expect: {}".format(flags(f1), flags(f2)))
    sys.exit(1)

  child1.sendline('s')
  child1.expect('(gameboy)')
  child2.sendline('step')
  child2.expect('gddb>')
