#!python3.5

import itertools
import copy

while True:
  print("Enter a bit pattern (e.g. 00xxx110): ")
  pattern = list(input().strip())

  products = list(itertools.product(range(2), repeat=pattern.count('x')))
  opcodes = []
  for p in products:
    index = 0
    tmp = copy.copy(pattern)
    for (i, _) in enumerate(tmp):
      if tmp[i] == 'x':
        tmp[i] = str(p[index])
        index += 1
    opcodes.append(int(''.join(tmp), 2))

  print(" | ".join([hex(i) for i in opcodes]))
