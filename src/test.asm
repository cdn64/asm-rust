  mov #2 acc

loop:
  mov #2 b # Move 2 into the B register
  add b
  jeq #4 loop
