// push constant 17
          // set D to 17
          @17
          D=A

          // set *SP to 17
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// push constant 17
          // set D to 17
          @17
          D=A

          // set *SP to 17
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
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
        D=-1
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        @END
        0;JMP
        
        (NEQ)
        D=0
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        
        (END)
        @SP
        M=M+1
// push constant 17
          // set D to 17
          @17
          D=A

          // set *SP to 17
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// push constant 16
          // set D to 16
          @16
          D=A

          // set *SP to 16
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
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
        D=-1
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        @END
        0;JMP
        
        (NEQ)
        D=0
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        
        (END)
        @SP
        M=M+1
// push constant 16
          // set D to 16
          @16
          D=A

          // set *SP to 16
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// push constant 17
          // set D to 17
          @17
          D=A

          // set *SP to 17
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
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
        D=-1
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        @END
        0;JMP
        
        (NEQ)
        D=0
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        
        (END)
        @SP
        M=M+1
// push constant 892
          // set D to 892
          @892
          D=A

          // set *SP to 892
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// push constant 891
          // set D to 891
          @891
          D=A

          // set *SP to 891
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// lt
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
        
        // jump if lt
        @LT
        D;JLT
        @NLT
        0;JMP
        
        (LT)
        D=-1
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        @END
        0;JMP
        
        (NLT)
        D=0
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        
        (END)
        @SP
        M=M+1
// push constant 891
          // set D to 891
          @891
          D=A

          // set *SP to 891
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// push constant 892
          // set D to 892
          @892
          D=A

          // set *SP to 892
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// lt
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
        
        // jump if lt
        @LT
        D;JLT
        @NLT
        0;JMP
        
        (LT)
        D=-1
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        @END
        0;JMP
        
        (NLT)
        D=0
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        
        (END)
        @SP
        M=M+1
// push constant 891
          // set D to 891
          @891
          D=A

          // set *SP to 891
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// push constant 891
          // set D to 891
          @891
          D=A

          // set *SP to 891
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// lt
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
        
        // jump if lt
        @LT
        D;JLT
        @NLT
        0;JMP
        
        (LT)
        D=-1
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        @END
        0;JMP
        
        (NLT)
        D=0
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        
        (END)
        @SP
        M=M+1
// push constant 32767
          // set D to 32767
          @32767
          D=A

          // set *SP to 32767
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// push constant 32766
          // set D to 32766
          @32766
          D=A

          // set *SP to 32766
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// gt
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
        
        // jump if gt
        @GT
        D;JGT
        @NGT
        0;JMP
        
        (GT)
        D=-1
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        @END
        0;JMP
        
        (NGT)
        D=0
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        
        (END)
        @SP
        M=M+1
// push constant 32766
          // set D to 32766
          @32766
          D=A

          // set *SP to 32766
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// push constant 32767
          // set D to 32767
          @32767
          D=A

          // set *SP to 32767
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// gt
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
        
        // jump if gt
        @GT
        D;JGT
        @NGT
        0;JMP
        
        (GT)
        D=-1
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        @END
        0;JMP
        
        (NGT)
        D=0
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        
        (END)
        @SP
        M=M+1
// push constant 32766
          // set D to 32766
          @32766
          D=A

          // set *SP to 32766
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// push constant 32766
          // set D to 32766
          @32766
          D=A

          // set *SP to 32766
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// gt
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
        
        // jump if gt
        @GT
        D;JGT
        @NGT
        0;JMP
        
        (GT)
        D=-1
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        @END
        0;JMP
        
        (NGT)
        D=0
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        
        (END)
        @SP
        M=M+1
// push constant 57
          // set D to 57
          @57
          D=A

          // set *SP to 57
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// push constant 31
          // set D to 31
          @31
          D=A

          // set *SP to 31
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// push constant 53
          // set D to 53
          @53
          D=A

          // set *SP to 53
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
// push constant 112
          // set D to 112
          @112
          D=A

          // set *SP to 112
          @SP
          A=M
          M=D

          // increment *SP
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
// negate
        @SP
        A=M-1
        D=-M
        @SP
        A=M-1
        M=D
// and
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
        D=D&M
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
// push constant 82
          // set D to 82
          @82
          D=A

          // set *SP to 82
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1
// or
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
        D=D|M
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
// logical not
        @SP
        A=M-1
        D=!M
        @SP
        A=M-1
        M=D
