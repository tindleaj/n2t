// push constant 1
          // set D to 1
          @1
          D=A

          // set *SP to 1
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// if-goto life
@SP
A=M-1
D=M
@SP
M=M-1
@life
D;JNE
// goto not_life
@not_life
// label life
(life)
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
// label not_life
(not_life)
// push constant 666
          // set D to 666
          @666
          D=A

          // set *SP to 666
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
