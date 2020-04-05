use crate::parser::{MathCommand, MemoryCommand, MemorySegment};
use std::io::Write;

pub struct Writer {
  pub output: Vec<u8>,
  pub namespace: String,
  pub current_function: String,
  jump_index: usize,
  return_index: usize
}

impl Writer {
  pub fn new(namespace: &str) -> Writer {
    Writer {
      output: Vec::new(),
      namespace: String::from(namespace),
      jump_index: 0,
      current_function: String::new(),
      return_index: 0
    }
  }

  /// Writes the bootstrap ASM to the output
  /// Initializes SP to 256 and calls Sys.init
  pub fn write_init(&mut self) {
    self.writeln("@256");
    self.writeln("D=A");
    self.writeln("@SP");
    self.writeln("M=D");

    self.writeln("@1");
    self.writeln("D=-A");
    self.writeln("@LCL");
    self.writeln("M=D");

    self.writeln("@2");
    self.writeln("D=-A");
    self.writeln("@ARG");
    self.writeln("M=D");

    self.writeln("@3");
    self.writeln("D=-A");
    self.writeln("@THIS");
    self.writeln("M=D");

    self.writeln("@4");
    self.writeln("D=-A");
    self.writeln("@THAT");
    self.writeln("M=D");

    self.write_call("Sys.init", 0);
    // // Push LCL base address to stack
    // self.writeln("@LCL");
    // self.writeln("D=M");
    // self.write_dreg_to_stack();
    // self.write_inc_sp();

    // // Push ARG base address to stack
    // self.writeln("@ARG");
    // self.writeln("D=M");
    // self.write_dreg_to_stack();
    // self.write_inc_sp();

    // // Push THIS base address to stack
    // self.writeln("@THIS");
    // self.writeln("D=M");
    // self.write_dreg_to_stack();
    // self.write_inc_sp();

    // // Push THAT base address to stack
    // self.writeln("@THAT");
    // self.writeln("D=M");
    // self.write_dreg_to_stack();
    // self.write_inc_sp();
    
    // // Reposition ARG 
    // self.writeln("@256");
    // self.writeln("D=A");
    // self.writeln("@ARG");
    // self.writeln("M=D");

    // // Reposition LCL
    // self.writeln("@SP");
    // self.writeln("D=M");
    // self.writeln("@LCL");
    // self.writeln("M=D");

    // // Goto called function
    // self.write_goto("Sys.init");
  }

  pub fn write_math(&mut self, command: MathCommand) {
    use MathCommand::*;

    match command {
      Add => {
        self.writeln("// add");

        // Set *R13 to the first argument
        self.writeln("@SP");
        self.writeln("A=M-1");
        self.writeln("D=M");
        self.writeln("@R13");
        self.writeln("M=D");

        // Set D register to the second argument
        self.writeln("@SP");
        self.writeln("D=M-1");
        self.writeln("D=D-1");
        self.writeln("A=D");
        self.writeln("D=M");

        // Add *R13 + D
        self.writeln("@R13");
        self.writeln("D=D+M");
        self.writeln("M=D");

        // Put sum in *(SP - 2)
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
        @EQ.{index}
        D;JEQ
        @NEQ.{index}
        0;JMP
        
        (EQ.{index})
        D=-1
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        @END.{index}
        0;JMP
        
        (NEQ.{index})
        D=0
        @SP
        M=M-1
        M=M-1
        @SP
        A=M
        M=D
        
        (END.{index})
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
          @GT.{index}
          D;JGT
          @NGT.{index}
          0;JMP
          
          (GT.{index})
          D=-1
          @SP
          M=M-1
          M=M-1
          @SP
          A=M
          M=D
          @END.{index}
          0;JMP
          
          (NGT.{index})
          D=0
          @SP
          M=M-1
          M=M-1
          @SP
          A=M
          M=D
          
          (END.{index})
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
          @LT.{index}
          D;JLT
          @NLT.{index}
          0;JMP
          
          (LT.{index})
          D=-1
          @SP
          M=M-1
          M=M-1
          @SP
          A=M
          M=D
          @END.{index}
          0;JMP
          
          (NLT.{index})
          D=0
          @SP
          M=M-1
          M=M-1
          @SP
          A=M
          M=D
          
          (END.{index})
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
        Static => {
          let label = &format!("{}.{}", &self.namespace, index);

          self.writeln(&format!("// push static {}", index));
          self.writeln(&format!("@{}", label));
          self.writeln("D=M");

          self.write_dreg_to_stack();
          self.write_inc_sp();
        }
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
        Static => {
          let label = &format!("{}.{}", &self.namespace, index);

          self.writeln(&format!("// pop static {}", index));
          self.writeln("@SP");
          self.writeln("A=M-1");
          self.writeln("D=M");
          self.writeln(&format!("@{}", label));
          self.writeln("M=D");

          self.write_dec_sp();
        }
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

  pub fn write_if(&mut self, label: &str) {

    self.writeln(&format!("// if-goto {}", label));

    // Put value at top of stack in D register
    self.writeln("@SP");
    self.writeln("A=M-1");
    self.writeln("D=M");

    self.write_dec_sp();

    // Conditional jump (*D != 0)
    self.writeln(&format!("@{}", label));
    self.writeln("D;JNE");
  }

  pub fn write_goto(&mut self, label: &str) {

    self.writeln(&format!("// goto {}", label));
    self.writeln(&format!("@{}", label));
    self.writeln("0;JMP");
  }

  pub fn write_label(&mut self, label: &str) {

    self.writeln(&format!("// label {}", label));
    self.writeln(&format!("({})", label));
  }

  pub fn write_function(&mut self, name: &str, num_locals: usize) {
    self.current_function = String::from(name);

    self.writeln(&format!("// function {} {}", name, num_locals));
    self.writeln(&format!("({})", name));

    for _arg in 0..num_locals {
      self.writeln("// init local");
      self.writeln("@0");
      self.writeln("D=A");
      self.writeln("@SP");
      self.writeln("A=M");
      self.writeln("M=D");

      self.write_inc_sp();
    }
  }

  pub fn write_call(&mut self, name: &str, num_args: usize) {
    self.return_index += 1;
    println!("write_call, @AFTER_{}_{}.{}", name,  &self.namespace,&self.return_index);
    
    // Push the return address to the stack
    self.writeln(&format!("@AFTER_{}_{}.{}", name,  &self.namespace,&self.return_index));
    self.writeln("D=A");
    self.write_dreg_to_stack();
    self.write_inc_sp();

    // Push LCL base address to stack
    self.writeln("@LCL");
    self.writeln("D=M");
    self.write_dreg_to_stack();
    self.write_inc_sp();

    // Push ARG base address to stack
    self.writeln("@ARG");
    self.writeln("D=M");
    self.write_dreg_to_stack();
    self.write_inc_sp();

    // Push THIS base address to stack
    self.writeln("@THIS");
    self.writeln("D=M");
    self.write_dreg_to_stack();
    self.write_inc_sp();

    // Push THAT base address to stack
    self.writeln("@THAT");
    self.writeln("D=M");
    self.write_dreg_to_stack();
    self.write_inc_sp();
    
    // Reposition ARG (ARG = SP-n-5)
    self.writeln("@SP");
    self.writeln("D=M");
    self.writeln(&format!("@{}", num_args));
    self.writeln("D=D-A");
    self.writeln("@5");
    self.writeln("D=D-A");
    self.writeln("@ARG");
    self.writeln("M=D");

    // Reposition LCL
    self.writeln("@SP");
    self.writeln("D=M");
    self.writeln("@LCL");
    self.writeln("M=D");

    // Goto called function
    self.write_goto(name);

    // Declare return address just after the called function lexically
    self.writeln(&format!("(AFTER_{}_{}.{})", name, &self.namespace, &self.return_index));

    

  }

  pub fn write_return(&mut self) {

    self.writeln("// return");

    // Store the base of the current frame in R13
    self.writeln("// Store the base of the current frame in R13");
    self.writeln("@LCL");
    self.writeln("D=M");
    self.writeln("@R13");
    self.writeln("M=D");
    
    // Store the return-address in R14
    self.writeln("// Store the return-address in R14");
    self.writeln("@R13");
    self.writeln("D=M");
    self.writeln("@5");
    self.writeln("A=D-A");
    self.writeln("D=M");
    self.writeln("@R14");
    self.writeln("M=D");

    // Put the return value on top of the caller's stack
    self.writeln("// Put the return value on top of the caller's stack");
    self.writeln("@SP");
    self.writeln("A=M-1");
    self.writeln("D=M");
    self.writeln("@ARG");
    self.writeln("A=M");
    self.writeln("M=D");

    // Restore SP of caller
    self.writeln("// Restore SP of caller");
    self.writeln("@ARG");
    self.writeln("D=M");
    self.writeln("@SP");
    self.writeln("M=D+1");

    // Restore THAT of caller
    self.writeln("// Restore THAT of caller");
    self.writeln("@R13");
    self.writeln("D=M");
    self.writeln("@1");
    self.writeln("D=D-A");
    self.writeln("A=D");
    self.writeln("D=M");
    self.writeln("@THAT");
    self.writeln("M=D");

    // Restore THIS of caller
    self.writeln("// Restore THIS of caller");
    self.writeln("@R13");
    self.writeln("D=M");
    self.writeln("@2");
    self.writeln("D=D-A");
    self.writeln("A=D");
    self.writeln("D=M");
    self.writeln("@THIS");
    self.writeln("M=D");

    // Restore ARG of caller
    self.writeln("// Restore ARG of caller");
    self.writeln("@R13");
    self.writeln("D=M");
    self.writeln("@3");
    self.writeln("D=D-A");
    self.writeln("A=D");
    self.writeln("D=M");
    self.writeln("@ARG");
    self.writeln("M=D");

    // Restore LCL of caller
    self.writeln("// Restore LCL of caller");
    self.writeln("@R13");
    self.writeln("D=M");
    self.writeln("@4");
    self.writeln("D=D-A");
    self.writeln("A=D");
    self.writeln("D=M");
    self.writeln("@LCL");
    self.writeln("M=D");

    // Goto return-address
    self.writeln("// Goto return address");
    self.writeln("@R14");
    self.writeln("A=M");
    self.writeln("0;JMP")

  }

  /// Increments the stack pointer by 1
  fn write_inc_sp(&mut self) {
    self.writeln("@SP");
    self.writeln("M=M+1");
  }
  /// Decrements the stack pointer by 1
  fn write_dec_sp(&mut self) {
    self.writeln("@SP");
    self.writeln("M=M-1");
  }

  /// Sets the value of *SP to the contents of the D register
  fn write_dreg_to_stack(&mut self) {
    self.writeln("@SP");
    self.writeln("A=M");
    self.writeln("M=D");
  }

  /// Copies top item on stack to address value at register provided
  fn write_copy_stack_head_indirect(&mut self, register: &str) {
    // Copy SP-1 into *register
    self.writeln("@SP");
    self.writeln("A=M-1");
    self.writeln("D=M");
    self.writeln(&format!("{}", register));
    self.writeln("A=M");
    self.writeln("M=D");
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
    let mut writer = Writer::new("test_namespace");
    writer.writeln("@SP");
    writer.writeln("M=D");

    assert_eq!(std::str::from_utf8(&writer.output).unwrap(), "@SP\nM=D\n");
  }
}
