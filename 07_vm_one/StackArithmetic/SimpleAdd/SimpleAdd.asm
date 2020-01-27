// push constant 7
          // set D to 7
          @7
          D=A

          // set *SP to 7
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// push constant 8
          // set D to 8
          @8
          D=A

          // set *SP to 8
          @SP
          A=M
          M=D

          // increment *SP
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

        // Move SP up by one
        @SP
        M=M+1
