use crate::parser::{MathCommand, MemoryCommand, MemorySegment};
use std::fs::File;
use std::io::Write;

pub struct Writer {
  pub output: Vec<u8>,
  jump_index: usize,
}

impl Writer {
  pub fn new() -> Writer {
    Writer {
      output: Vec::new(),
      jump_index: 0,
    }
  }

  pub fn write_math(&mut self, command: MathCommand) {
    use MathCommand::*;

    match command {
      Add => {
        self.writeln("// add the top two items on the stack");

        self.writeln("// Set first arg to *R13");
        self.writeln("@SP");
        self.writeln("A=M-1");
        self.writeln("D=M");
        self.writeln("@R13");
        self.writeln("M=D");

        self.writeln("// Set D to second arg");
        self.writeln("@SP");
        self.writeln("D=M-1");
        self.writeln("D=D-1");
        self.writeln("A=D");
        self.writeln("D=M");

        self.writeln("// Add");
        self.writeln("@R13");
        self.writeln("D=D+M");
        self.writeln("M=D");

        self.writeln("// Put sum in *(SP - 2)");
        self.writeln("@SP");
        self.writeln("M=M-1");
        self.writeln("M=M-1");
        self.writeln("@R13");
        self.writeln("D=M");
        self.writeln("@SP");
        self.writeln("A=M");
        self.writeln("M=D");

        self.writeln("// Move SP up by one");
        self.writeln("@SP");
        self.writeln("M=M+1");
      }
      Subtract => self.writeln(
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
        M=M+1",
      ),
      Negate => self.writeln(
        "// negate
        @SP
        A=M-1
        D=-M
        @SP
        A=M-1
        M=D",
      ),
      EqualTo => {
        let formatted = format!(
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
        @EQ_{index}
        D;JEQ
        @NEQ_{index}
        0;JMP
        
        (EQ_{index})
        D=0
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        @END_{index}
        0;JMP
        
        (NEQ_{index})
        D=-1
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        
        (END_{index})
        @SP
        M=M+1",
          index = self.jump_index
        );

        self.jump_index += 1;
        self.writeln(&formatted)
      }
      GreaterThan => {
        let formatted = format!(
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
      @GT_{index}
      D;JGT
      @NGT_{index}
      0;JMP
      
      (GT_{index})
      D=-1
      @SP
      M=M-1
      M=M-1
      @SP
      A=M
      M=D
      @END_{index}
      0;JMP
      
      (NGT_{index})
      D=0
      @SP
      M=M-1
      M=M-1
      @SP
      A=M
      M=D
      
      (END_{index})
      @SP
      M=M+1",
          index = self.jump_index
        );

        self.jump_index += 1;
        self.writeln(&formatted);
      }
      LessThan => {
        let formatted = format!(
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
        @LT_{index}
        D;JLT
        @NLT_{index}
        0;JMP
        
        (LT_{index})
        D=-1
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        @END_{index}
        0;JMP
        
        (NLT_{index})
        D=0
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        
        (END_{index})
        @SP
        M=M+1",
          index = self.jump_index
        );

        self.jump_index += 1;
        self.writeln(&formatted);
      }
      And => self.writeln(
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
        M=M+1",
      ),
      Or => self.writeln(
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
        M=M+1",
      ),
      Not => self.writeln(
        "// logical not
        @SP
        A=M-1
        D=!M
        @SP
        A=M-1
        M=D",
      ),
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
        Constant => self.writeln(&format!(
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

  fn writeln(&mut self, content: &str) {
    writeln!(self.output, "{}", content).expect("problem writing to buffer");
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn writes_output_with_newlines() {
    let mut writer = Writer::new();
    writer.writeln("@SP");
    writer.writeln("M=D");

    assert_eq!(std::str::from_utf8(&writer.output).unwrap(), "@SP\nM=D\n");
  }
}
