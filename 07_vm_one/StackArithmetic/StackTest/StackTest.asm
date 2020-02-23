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
        // Set *R13 'y' arg to
        @SP
        A=M-1
        D=M
        @R13
        M=D
        
        // Set D to 'x' arg
        @SP
        D=M-1
        D=D-1
        A=D
        D=M
        
        // Subtract x - y
        @R13
        D=D-M
        
        // jump if result = 0
        @EQ_0
        D;JEQ
        @NEQ_0
        0;JMP
        
        (EQ_0)
        D=-1
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        @END_0
        0;JMP
        
        (NEQ_0)
        D=0
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        
        (END_0)
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
        // Set *R13 'y' arg to
        @SP
        A=M-1
        D=M
        @R13
        M=D
        
        // Set D to 'x' arg
        @SP
        D=M-1
        D=D-1
        A=D
        D=M
        
        // Subtract x - y
        @R13
        D=D-M
        
        // jump if result = 0
        @EQ_1
        D;JEQ
        @NEQ_1
        0;JMP
        
        (EQ_1)
        D=-1
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        @END_1
        0;JMP
        
        (NEQ_1)
        D=0
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        
        (END_1)
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
        // Set *R13 'y' arg to
        @SP
        A=M-1
        D=M
        @R13
        M=D
        
        // Set D to 'x' arg
        @SP
        D=M-1
        D=D-1
        A=D
        D=M
        
        // Subtract x - y
        @R13
        D=D-M
        
        // jump if result = 0
        @EQ_2
        D;JEQ
        @NEQ_2
        0;JMP
        
        (EQ_2)
        D=-1
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        @END_2
        0;JMP
        
        (NEQ_2)
        D=0
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        
        (END_2)
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
          // Set 'y' arg to *R13
          @SP
          A=M-1
          D=M
          @R13
          M=D
          
          // Set D to 'x' arg
          @SP
          D=M-1
          D=D-1
          A=D
          D=M
          
          // Subtract x - y
          @R13
          D=D-M
          
          // jump if result < 0
          @LT_3
          D;JLT
          @NLT_3
          0;JMP
          
          (LT_3)
          D=-1
          @SP
          M=M-1
          M=M-1
          @SP
          A=M
          M=D
          @END_3
          0;JMP
          
          (NLT_3)
          D=0
          @SP
          M=M-1
          M=M-1
          @SP
          A=M
          M=D
          
          (END_3)
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
          // Set 'y' arg to *R13
          @SP
          A=M-1
          D=M
          @R13
          M=D
          
          // Set D to 'x' arg
          @SP
          D=M-1
          D=D-1
          A=D
          D=M
          
          // Subtract x - y
          @R13
          D=D-M
          
          // jump if result < 0
          @LT_4
          D;JLT
          @NLT_4
          0;JMP
          
          (LT_4)
          D=-1
          @SP
          M=M-1
          M=M-1
          @SP
          A=M
          M=D
          @END_4
          0;JMP
          
          (NLT_4)
          D=0
          @SP
          M=M-1
          M=M-1
          @SP
          A=M
          M=D
          
          (END_4)
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
          // Set 'y' arg to *R13
          @SP
          A=M-1
          D=M
          @R13
          M=D
          
          // Set D to 'x' arg
          @SP
          D=M-1
          D=D-1
          A=D
          D=M
          
          // Subtract x - y
          @R13
          D=D-M
          
          // jump if result < 0
          @LT_5
          D;JLT
          @NLT_5
          0;JMP
          
          (LT_5)
          D=-1
          @SP
          M=M-1
          M=M-1
          @SP
          A=M
          M=D
          @END_5
          0;JMP
          
          (NLT_5)
          D=0
          @SP
          M=M-1
          M=M-1
          @SP
          A=M
          M=D
          
          (END_5)
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
          // Set 'y' arg to *R13
          @SP
          A=M-1
          D=M
          @R13
          M=D
          
          // Set D to 'x' arg
          @SP
          D=M-1
          D=D-1
          A=D
          D=M
          
          // Subtract x - y
          @R13
          D=D-M
          
          // jump if result > 0
          @GT_6
          D;JGT
          @NGT_6
          0;JMP
          
          (GT_6)
          D=-1
          @SP
          M=M-1
          M=M-1
          @SP
          A=M
          M=D
          @END_6
          0;JMP
          
          (NGT_6)
          D=0
          @SP
          M=M-1
          M=M-1
          @SP
          A=M
          M=D
          
          (END_6)
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
          // Set 'y' arg to *R13
          @SP
          A=M-1
          D=M
          @R13
          M=D
          
          // Set D to 'x' arg
          @SP
          D=M-1
          D=D-1
          A=D
          D=M
          
          // Subtract x - y
          @R13
          D=D-M
          
          // jump if result > 0
          @GT_7
          D;JGT
          @NGT_7
          0;JMP
          
          (GT_7)
          D=-1
          @SP
          M=M-1
          M=M-1
          @SP
          A=M
          M=D
          @END_7
          0;JMP
          
          (NGT_7)
          D=0
          @SP
          M=M-1
          M=M-1
          @SP
          A=M
          M=D
          
          (END_7)
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
          // Set 'y' arg to *R13
          @SP
          A=M-1
          D=M
          @R13
          M=D
          
          // Set D to 'x' arg
          @SP
          D=M-1
          D=D-1
          A=D
          D=M
          
          // Subtract x - y
          @R13
          D=D-M
          
          // jump if result > 0
          @GT_8
          D;JGT
          @NGT_8
          0;JMP
          
          (GT_8)
          D=-1
          @SP
          M=M-1
          M=M-1
          @SP
          A=M
          M=D
          @END_8
          0;JMP
          
          (NGT_8)
          D=0
          @SP
          M=M-1
          M=M-1
          @SP
          A=M
          M=D
          
          (END_8)
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
