use crate::parser::{MathCommand, MemoryCommand, MemorySegment};
use std::collections::HashMap;
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

  /// Writes ASM to self.output that increments the stack pointer by 1
  fn write_inc_sp(&mut self) {
    self.writeln("@SP");
    self.writeln("M=M+1");
  }
  /// Writes ASM to self.output that decrements the stack pointer by 1
  fn write_dec_sp(&mut self) {
    self.writeln("@SP");
    self.writeln("M=M-1");
  }

  /// Writes ASM to self.output that sets the value of *SP to the contents of the D register
  fn write_dreg_to_stack(&mut self) {
    self.writeln("@SP");
    self.writeln("A=M");
    self.writeln("M=D");
  }

  /// Copies top item on stack to address value at register provided
  fn write_copy_stack_head_indirect(&mut self, register: &str) {
    // Copy SP-1 into *R13
    self.writeln("@SP");
    self.writeln("A=M-1");
    self.writeln("D=M");
    self.writeln(&format!("{}", register));
    self.writeln("A=M");
    self.writeln("M=D");
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

        self.write_inc_sp();
      }
      Subtract => {
        self.writeln(
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
        M=D",
        );
        self.write_inc_sp();
      }
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
          r"// eq
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
        @EQ_{index}
        D;JEQ
        @NEQ_{index}
        0;JMP
        
        (EQ_{index})
        D=-1
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        @END_{index}
        0;JMP
        
        (NEQ_{index})
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
        self.writeln(&formatted)
      }
      GreaterThan => {
        let formatted = format!(
          "// gt
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
      And => {
        self.writeln(
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
        M=D",
        );
        self.write_inc_sp();
      }
      Or => {
        self.writeln(
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
        M=D",
        );

        self.write_inc_sp();
      }
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
        Argument => {
          self.writeln(&format!("// push arg {}", index));
          // set temp register to index
          self.writeln(&format!("@{}", index));
          self.writeln("D=A");
          self.writeln("@ARG");
          self.writeln("A=D+M");
          self.writeln("D=M");

          self.write_dreg_to_stack();
          self.write_inc_sp();
        }
        Local => {
          self.writeln(&format!("// push local {}", index));
          // set temp register to index
          self.writeln(&format!("@{}", index));
          self.writeln("D=A");
          self.writeln("@LCL");
          self.writeln("A=D+M");
          self.writeln("D=M");

          self.write_dreg_to_stack();
          self.write_inc_sp();
        }
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
        This => {
          self.writeln(&format!("// push this {}", index));
          // set temp register to index
          self.writeln(&format!("@{}", index));
          self.writeln("D=A");
          self.writeln("@THIS");
          self.writeln("A=D+M");
          self.writeln("D=M");

          self.write_dreg_to_stack();
          self.write_inc_sp();
        }
        That => {
          self.writeln(&format!("// push that {}", index));
          // set temp register to index
          self.writeln(&format!("@{}", index));
          self.writeln("D=A");
          self.writeln("@THAT");
          self.writeln("A=D+M");
          self.writeln("D=M");

          self.write_dreg_to_stack();
          self.write_inc_sp();
        }
        Pointer => {
          self.writeln(&format!("// push pointer {}", index));
          self.writeln(&format!("@{}", index));
          self.writeln("D=A");
          self.writeln("@3"); // pointer base
          self.writeln("A=D+A");
          self.writeln("D=M");

          self.write_dreg_to_stack();
          self.write_inc_sp();
        }
        Temp => {
          self.writeln(&format!("// push temp {}", index));
          self.writeln(&format!("@{}", index));
          self.writeln("D=A");
          self.writeln("@5"); // temp base
          self.writeln("A=D+A");
          self.writeln("D=M");

          self.write_dreg_to_stack();
          self.write_inc_sp();
        }
      },

      Pop => match segment {
        Argument => {
          self.writeln(&format!("// pop argument {}", index));
          // set R13 to *ARG+index
          self.writeln(&format!("@{}", index));
          self.writeln("D=A");
          self.writeln("@ARG");
          self.writeln("D=D+M");
          self.writeln("@R13");
          self.writeln("M=D");

          self.write_copy_stack_head_indirect("@R13");

          self.write_dec_sp();
        }
        Local => {
          self.writeln(&format!("// pop local {}", index));
          // set R13 to *LCL+index
          self.writeln(&format!("@{}", index));
          self.writeln("D=A");
          self.writeln("@LCL");
          self.writeln("D=D+M");
          self.writeln("@R13");
          self.writeln("M=D");

          self.write_copy_stack_head_indirect("@R13");

          self.write_dec_sp();
        }
        Static => unimplemented!(),
        Constant => panic!("cannot pop to the constant segment"),
        This => {
          self.writeln(&format!("// pop this {}", index));
          // set R13 to *THIS+index
          self.writeln(&format!("@{}", index));
          self.writeln("D=A");
          self.writeln("@THIS");
          self.writeln("D=D+M");
          self.writeln("@R13");
          self.writeln("M=D");

          self.write_copy_stack_head_indirect("@R13");

          self.write_dec_sp();
        }
        That => {
          self.writeln(&format!("// pop that {}", index));
          // set R13 to *THAT+index
          self.writeln(&format!("@{}", index));
          self.writeln("D=A");
          self.writeln("@THAT");
          self.writeln("D=D+M");
          self.writeln("@R13");
          self.writeln("M=D");

          self.write_copy_stack_head_indirect("@R13");

          self.write_dec_sp();
        }
        Pointer => {
          self.writeln(&format!("// pop pointer {}", index));
          // set R13 to 3+index
          self.writeln(&format!("@{}", index));
          self.writeln("D=A");
          self.writeln("@3");
          self.writeln("D=D+A");
          self.writeln("@R13");
          self.writeln("M=D");

          self.write_copy_stack_head_indirect("@R13");

          self.write_dec_sp();
        }
        Temp => {
          self.writeln(&format!("// pop temp {}", index));
          // set R13 to 5+index
          self.writeln(&format!("@{}", index));
          self.writeln("D=A");
          self.writeln("@5");
          self.writeln("D=D+A");
          self.writeln("@R13");
          self.writeln("M=D");

          self.write_copy_stack_head_indirect("@R13");

          self.write_dec_sp();
        }
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
