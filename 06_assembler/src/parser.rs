use crate::code;
use crate::symbol_table::SymbolTable;
use std::io::Write;

pub struct Parser {
  symbol_table: SymbolTable,
}

impl Parser {
  pub fn new() -> Parser {
    Parser {
      symbol_table: SymbolTable::new(),
    }
  }
  pub fn parse(&mut self, input: &str) -> Vec<u8> {
    let mut buf = Vec::new();
    let mut var_counter = 0;

    // first pass
    self.resolve_labels(input);

    for line in input.lines() {
      let line = line.trim();

      if line.starts_with("//") || line.is_empty() {
        continue;
      }

      match Parser::command_type(line).unwrap() {
        CommandType::ACommand => {
          let line = line.trim().trim_matches('@');
          let symbol: u16;

          // if the line is a decimal number, use it as is
          if Parser::is_literal(line) {
            symbol = line.parse().unwrap();
          } else {
            // add the var to the symbol table if not already there
            if !self.symbol_table.contains(line) {
              self
                .symbol_table
                .add_entry(line, &(16 + var_counter).to_string());
              var_counter += 1;
            }

            // lookup the symbol in the table, and get the value
            symbol = self.symbol_table.get_addr(line).parse().unwrap();
          }

          // binary format
          let bin_symbol = format!("{:b}", symbol);

          // pad to 16 chars
          writeln!(&mut buf, "{:0>16}", bin_symbol).expect("problem writing to buffer");
        }
        CommandType::CCommand => {
          let mut tokens: Vec<&str> = line.split(|c| c == '=' || c == ';').collect();

          // Add null ops to short commands
          if !line.contains('=') {
            tokens.insert(0, "null");
          }

          if !line.contains(';') {
            tokens.insert(2, "null")
          }

          // lookup tokens in code tables
          let dest = match tokens.get(0) {
            Some(v) => code::dest(v).expect("invalid DEST token"),
            None => code::dest("null").unwrap(),
          };

          let comp = match tokens.get(1) {
            Some(v) => code::comp(v).expect("invalid COMP token"),
            None => code::comp("null").unwrap(),
          };

          let jump = match tokens.get(2) {
            Some(v) => code::jump(v).expect("invalid JUMP token"),
            None => code::jump("null").unwrap(),
          };

          // convert tokens to binary (machine instruction)
          let bin_line = format!("111{}{}{}", comp, dest, jump);

          // write binary instruction to buffer,
          writeln!(&mut buf, "{}", bin_line).expect("problem writing to buffer");
        }
        _ => {}
      }
    }

    buf
  }

  fn is_literal(line: &str) -> bool {
    if line.chars().any(|c| c.is_alphabetic()) {
      return false;
    }

    true
  }

  fn resolve_labels(&mut self, input: &str) {
    let mut counter = 0;

    for line in input.lines() {
      let line = line.trim();

      if line.starts_with("//") || line.is_empty() {
        continue;
      }

      match Parser::command_type(line).unwrap() {
        CommandType::LCommand => {
          let symbol = line.trim_matches(|c| c == '(' || c == ')');
          let next_line = &counter.to_string();
          self.symbol_table.add_entry(symbol, next_line)
        }
        _ => counter += 1,
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
mod tests {
  use super::*;

  #[test]
  fn get_a_command() {
    match Parser::command_type("@255").unwrap() {
      CommandType::ACommand => assert!(true),
      _ => panic!(),
    }
  }

  #[test]
  fn get_c_command() {
    match Parser::command_type("D=D+M;JGT").unwrap() {
      CommandType::CCommand => assert!(true),
      _ => panic!(),
    }
  }

  #[test]
  fn get_l_command() {
    match Parser::command_type("(SYMBOL)").unwrap() {
      CommandType::LCommand => assert!(true),
      _ => panic!(),
    }
  }

  #[test]
  fn resolve_labels() {
    let mut p = Parser::new();
    p.resolve_labels("(LOOP)");

    assert!(p.symbol_table.contains("LOOP"));
    assert_eq!(p.symbol_table.get_addr("LOOP"), "0");
  }

  #[test]
  fn resolve_multiple_labels() {
    let mut p = Parser::new();
    p.resolve_labels("(FIRST)\n@1234\nD=D+M;JGT\n(SECOND)\n@5678");

    assert!(p.symbol_table.contains("FIRST"));
    assert!(p.symbol_table.contains("SECOND"));
    assert_eq!(p.symbol_table.get_addr("FIRST"), "0");
    assert_eq!(p.symbol_table.get_addr("SECOND"), "2");
  }
}
