#!python3.5

# Tool to indent CALLs to make it easier to trace the execution
# in the source.

spacer = ''
with open("logr.txt", "r") as f:
  for line in f:
    if line.upper().find("CALL") != -1:
      spacer += ' '
    print(spacer, line, end="")
    if line.upper().find("RET") != -1:
      spacer = spacer[:-1]
