// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)

// Put your code here.

@i
M = 1

@sum
M = 0

(LOOP)
  // If i == y, end the loop
  @i
  D = M

  @R1
  D = D - M

  @END
  D;JGT

  // Add x to the sum
  @R0
  D = M

  @sum
  M = M + D

  // Increment i by 1
  @i
  M = M + 1

  @LOOP
  0;JMP

(END)

// Set product to R2
@sum
D = M

@R2
M = D

// Terminate program
(EXIT)
@EXIT
0;JMP
