use crate::parser::{MathCommand, MemoryCommand, MemorySegment};
use std::fs::File;
use std::io::Write;

pub struct Writer {
  pub output: Vec<u8>,
}

impl Writer {
  pub fn new() -> Writer {
    Writer { output: Vec::new() }
  }

  pub fn write_math(&mut self, command: MathCommand) {
    use MathCommand::*;

    match command {
      Add => self.write(&format!(
        "// add the top two items on the stack
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
        M=M+1"
      )),
      Subtract => self.write(&format!(
        "// sub
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
        M=M+1"
      )),
      Negate => self.write(&format!(
        "// negate
        @SP
        A=M-1
        D=-M
        @SP
        A=M-1
        M=D"
      )),
      EqualTo => self.write(&format!(
        "// eq
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
        M=M+1"
      )),
      GreaterThan => self.write(&format!(
        "// gt
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
        M=M+1"
      )),
      LessThan => self.write(&format!(
        "// lt
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
        M=M+1"
      )),
      And => self.write(&format!(
        "// and
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
        M=M+1"
      )),
      Or => self.write(&format!(
        "// or
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
        M=M+1"
      )),
      Not => self.write(&format!(
        "// logical not
        @SP
        A=M-1
        D=!M
        @SP
        A=M-1
        M=D"
      )),
    }
  }

  pub fn write_push_pop(&mut self, command: MemoryCommand, segment: MemorySegment, index: usize) {
    use MemoryCommand::*;
    use MemorySegment::*;

    match command {
      Push => match segment {
        Argument => unimplemented!(),
        Local => unimplemented!(),
        Static => unimplemented!(),
        Constant => self.write(&format!(
          "// push constant {index}
          // set D to {index}
          @{index}
          D=A

          // set *SP to {index}
          @SP
          A=M
          M=D

          // increment *SP
          @SP
          M=M+1",
          index = index,
        )),
        This => unimplemented!(),
        That => unimplemented!(),
        Pointer => unimplemented!(),
        Temp => unimplemented!(),
      },

      Pop => match segment {
        Constant => panic!("cannot pop to the constant segment"),
        _ => unimplemented!(),
      },
    }
  }

  fn write(&mut self, content: &str) {
    writeln!(self.output, "{}", content).expect("problem writing to buffer");
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn name() {
    unimplemented!();
  }
}
