  mov #2 acc

loop:
; full-line-comment
  nop
  mov #2 b ; Move 2 into the B register
  add b
  jlt #10 loop
