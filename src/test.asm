  mov #2 acc

loop:
  mov #2 b # Move 2 into the B register
  add b
  jlt #10 loop
