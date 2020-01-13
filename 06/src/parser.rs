use std::collections::HashMap;
use std::fs;

const SYMBOL_TABLE: [(&str, &str); 22] = [
  ("SP", "0"),
  ("LCL", "1"),
  ("ARG", "2"),
  ("THIS", "3"),
  ("THAT", "4"),
  ("R0", "0"),
  ("R1", "1"),
  ("R2", "2"),
  ("R3", "3"),
  ("R4", "4"),
  ("R5", "5"),
  ("R6", "6"),
  ("R7", "7"),
  ("R8", "8"),
  ("R9", "9"),
  ("R10", "10"),
  ("R11", "11"),
  ("R13", "13"),
  ("R14", "14"),
  ("R15", "15"),
  ("SCREEN", "16384"),
  ("KBD", "24567"),
];

enum CommandType {
  A_COMMAND,
  C_COMMAND,
  L_COMMAND,
}

pub struct Parser {
  symbol: String,
  dest: String,
  comp: String,
  jump: String,
}

impl Parser {
  pub fn parse(path: &str) {
    let input = match fs::read_to_string(path) {
      Ok(value) => value,
      Err(e) => panic!(e),
    };

    for line in input.split('\n') {
      if line.len() < 1 {
        continue;
      }

      if &line[0..2] == "//" {
        continue;
      }

      match Parser::command_type(line) {
        Some(command) => match command {
          CommandType::A_COMMAND => println!("A Command"),
          CommandType::C_COMMAND => println!("C Command"),
          CommandType::L_COMMAND => println!("L Command"),
        },
        None => println!("No command"),
      }
    }
  }

  fn advance() {
    unimplemented!()
  }

  fn command_type(line: &str) -> Option<CommandType> {
    if &line[0..1] == "@" {
      return Some(CommandType::A_COMMAND);
    } else if &line[0..1] == "(" {
      return Some(CommandType::L_COMMAND);
    } else if line.contains("=") {
      return Some(CommandType::C_COMMAND);
    }

    None
  }

  fn has_more_commands() -> bool {
    unimplemented!()
  }
}

#[cfg(test)]
mod tests {}
