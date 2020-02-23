// push constant 10
          // set D to 10
          @10
          D=A

          // set *SP to 10
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// pop local 0
@0
D=A
@LCL
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
// push constant 21
          // set D to 21
          @21
          D=A

          // set *SP to 21
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// push constant 22
          // set D to 22
          @22
          D=A

          // set *SP to 22
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// pop argument 2
@2
D=A
@ARG
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
// pop argument 1
@1
D=A
@ARG
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
// push constant 36
          // set D to 36
          @36
          D=A

          // set *SP to 36
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// pop this 6
@6
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
// push constant 42
          // set D to 42
          @42
          D=A

          // set *SP to 42
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// push constant 45
          // set D to 45
          @45
          D=A

          // set *SP to 45
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// pop that 5
@5
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
// pop that 2
@2
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
// push constant 510
          // set D to 510
          @510
          D=A

          // set *SP to 510
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// pop temp 6
@6
D=A
@5
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
// push local 0
@0
D=A
@LCL
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
// push that 5
@5
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
// push arg 1
@1
D=A
@ARG
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
// push this 6
@6
D=A
@THIS
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
// push this 6
@6
D=A
@THIS
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
// push temp 6
@6
D=A
@5
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
