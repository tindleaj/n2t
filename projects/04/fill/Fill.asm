// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.

(MAIN)
  @KBD
  D = M

   // If KBD > 0 go to FILL
  @FILL
  D;JNE

  // If KBD <= 0 go to CLEAR
  @CLEAR
  D;JLE

  // End iteration
  @MAIN
  0;JMP
(END)

(EXIT)
@EXIT
0;JMP

// Fill screen with black pixels
(FILL)
  @lscr
  M = 0

  // Set index
  @index
  M = 0

  // a SCREEN row is 32 16bit words, so 32 * 256
  @8192
  D = A

  @lscr
  M = D

  (FLOOP)
    // Check if we've iterated over all SCREEN addresses, jump back to MAIN if so
    @index
    D = M

    @lscr
    D = D - M

    @MAIN
    D;JGT

    // Set current word bits to 1
    @SCREEN
    D = A

    @index
    D = D + M

    A = D
    M = -1

    // Increment index
    @index
    M = M + 1

    // Check if keyboard input changed
    @KBD
    D = M

    @MAIN
    D;JEQ

    // Go to next iteration of loop
    @FLOOP
    0;JMP
  (EFLOOP)
(ENDFILL)

// Clear screen
(CLEAR)
  @lscr
  M = 0

  @index
  M = 0

  @8192
  D = A

  @lscr
  M = D

  (CLOOP)
    // Check if we've iterated over all SCREEN addresses, jump back to MAIN if so
    @index
    D = M

    @lscr
    D = D - M

    @MAIN
    D;JGT

    // Set current word bits to 0
    @SCREEN
    D = A

    @index
    D = D + M

    A = D
    M = 0

    // Increment index
    @index
    M = M + 1

    // Check if keyboard input changed
    @KBD
    D = M

    @MAIN
    D;JNE

    // Go to next iteration of loop
    @CLOOP
    0;JMP
  (ECLOOP)
  @MAIN
  0;JMP
(ENDCLEAR)