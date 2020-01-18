use crate::code;

use std::char;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

const PREDEFINED_SYMBOLS: [(&str, &str); 22] = [
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

pub struct Parser {}

impl Parser {
  pub fn parse(path: &str) {
    let input = fs::read_to_string(path).expect("problem reading path to string");
    let mut output = fs::File::create("output.hack").expect("problem creating file 'output.hack'");

    for line in input.lines() {
      let line = line.trim();

      if line.is_empty() {
        continue;
      }

      if line.starts_with("//") {
        continue;
      }

      match Parser::command_type(line) {
        Some(command) => match command {
          CommandType::ACommand => {
            let symbol: u16 = line.trim().trim_matches('@').parse().unwrap();
            let bin_symbol = format!("{:b}", symbol);
            let mut buf = Vec::new();

            writeln!(&mut buf, "{:0>16}", bin_symbol).expect("problem writing to buffer");

            output.write(&buf).expect("problem writing to file");
          }
          CommandType::CCommand => {
            let mut tokens: Vec<&str> = line.split(|c| c == '=' || c == ';').collect();
            let mut buf = Vec::new();

            // Add null ops to short commands
            if !line.contains('=') {
              tokens.insert(0, "null");
            }

            if !line.contains(';') {
              tokens.insert(2, "null")
            }

            let dest = match tokens.get(0) {
              Some(v) => code::dest(v).expect("invalid DEST token"),
              None => code::dest("null").unwrap(),
            };

            let comp = match tokens.get(1) {
              Some(v) => {
                dbg!(&v);
                code::comp(v).expect("invalid COMP token")
              }
              None => code::comp("null").unwrap(),
            };

            let jump = match tokens.get(2) {
              Some(v) => code::jump(v).expect("invalid JUMP token"),
              None => code::jump("null").unwrap(),
            };

            let bin_line = format!("111{}{}{}", comp, dest, jump);
            writeln!(&mut buf, "{:0>16}", bin_line).expect("problem writing to buffer");

            output.write(&buf).expect("problem writing to file");
          }
          CommandType::LCommand => println!("L Command"),
        },
        None => println!("No command"),
      }
    }
  }

  /// Returns the type of a command
  fn command_type(line: &str) -> Option<CommandType> {
    if line.trim().starts_with("@") {
      return Some(CommandType::ACommand);
    } else if line.trim().starts_with("(") {
      return Some(CommandType::LCommand);
    } else if code::contains_comp(line) {
      return Some(CommandType::CCommand);
    }

    None
  }
}

enum CommandType {
  /// For @xxx where xxx is a symbol or decimal number
  ACommand,
  /// For lines that follow the form `dest=comp;jump`
  CCommand,
  /// Psuedo-command for (xxx) where xxx is a symbol
  LCommand,
}

#[cfg(test)]
mod tests {}
