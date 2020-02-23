// push constant 111
          // set D to 111
          @111
          D=A

          // set *SP to 111
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// push constant 333
          // set D to 333
          @333
          D=A

          // set *SP to 333
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// push constant 888
          // set D to 888
          @888
          D=A

          // set *SP to 888
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// pop static 8
@SP
A=M-1
D=M
@StaticTest.8
M=D
@SP
M=M-1
// pop static 3
@SP
A=M-1
D=M
@StaticTest.3
M=D
@SP
M=M-1
// pop static 1
@SP
A=M-1
D=M
@StaticTest.1
M=D
@SP
M=M-1
// push static 3
@StaticTest.3
D=M
@SP
A=M
M=D
@SP
M=M+1
// push static 1
@StaticTest.1
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
// push static 8
@StaticTest.8
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
