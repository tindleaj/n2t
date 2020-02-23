// sub
@SP
A=M-1
D=M
@R13
M=D

@SP
D=M-1
D=D-1
A=D
D=M

@R13
D=D-M
M=D

@SP
M=M-1
M=M-1
@R13
D=M
@SP
A=M
M=D

@SP
M=M+1

// negate
@SP
A=M-1
D=!M
@SP
A=M-1
M=D

// eq
// Set first arg to *R13
@SP
A=M-1
D=M
@R13
M=D

// Set D to second arg
@SP
D=M-1
D=D-1
A=D
D=M

// Subtract to check equality
@R13
D=D-M

// jump if eq
@EQ
D;JEQ
@NEQ
0;JMP

(EQ)
// if equal set to -1 (true)
@-1
D=A
@SP
M=M-1
M=M-1
@SP
A=M
M=D
@END
0;JMP

(NEQ)
// if not equal set to 0 (false)
@0
D=A
@SP
M=M-1
M=M-1
@SP
A=M
M=D

(END)
// Move SP up by one
@SP
M=M+1
  