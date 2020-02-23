// push constant 3030
          // set D to 3030
          @3030
          D=A

          // set *SP to 3030
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// pop pointer 0
@0
D=A
@3
D=D+A
@R13
M=D
@SP
A=M-1
D=M
@R13
A=M
M=D
@SP
M=M-1
// push constant 3040
          // set D to 3040
          @3040
          D=A

          // set *SP to 3040
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// pop pointer 1
@1
D=A
@3
D=D+A
@R13
M=D
@SP
A=M-1
D=M
@R13
A=M
M=D
@SP
M=M-1
// push constant 32
          // set D to 32
          @32
          D=A

          // set *SP to 32
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// pop this 2
@2
D=A
@THIS
D=D+M
@R13
M=D
@SP
A=M-1
D=M
@R13
A=M
M=D
@SP
M=M-1
// push constant 46
          // set D to 46
          @46
          D=A

          // set *SP to 46
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// pop that 6
@6
D=A
@THAT
D=D+M
@R13
M=D
@SP
A=M-1
D=M
@R13
A=M
M=D
@SP
M=M-1
// push pointer 0
@0
D=A
@3
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1
// push pointer 1
@1
D=A
@3
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1
// add the top two items on the stack
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
// Add
@R13
D=D+M
M=D
// Put sum in *(SP - 2)
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
// push this 2
@2
D=A
@THIS
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
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
// push that 6
@6
D=A
@THAT
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
// add the top two items on the stack
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
// Add
@R13
D=D+M
M=D
// Put sum in *(SP - 2)
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
