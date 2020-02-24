use std::str::Lines;

pub struct Parser<'a> {
  lines: Lines<'a>,
}

impl<'a> Parser<'a> {
  pub fn new(file_contents: &'a str) -> Parser<'a> {
    Parser {
      lines: file_contents.lines(),
    }
  }

  pub fn has_more_commands(&self) -> bool {
    // This seems unoptimal?
    let mut lookahead = self.lines.clone().peekable();
    match lookahead.peek() {
      Some(_v) => return true,
      None => return false,
    }
  }

  pub fn advance(&mut self) -> &str {
    self
      .lines
      .next()
      .expect("cannot advance, no more lines")
      .trim()
  }

  pub fn command_type(line: &str) -> CommandType {
    use BranchingCommand::*;
    use CommandType::*;
    use FunctionCommand::*;
    use MathCommand::*;
    use MemoryCommand::*;

    let command = line
      .split_whitespace()
      .nth(0)
      .expect(&format!("invalid line: '{}'", line));

    match command {
      "add" => Math(Add),
      "sub" => Math(Subtract),
      "neg" => Math(Negate),
      "eq" => Math(EqualTo),
      "gt" => Math(GreaterThan),
      "lt" => Math(LessThan),
      "and" => Math(And),
      "or" => Math(Or),
      "not" => Math(Not),

      "push" => Memory(Push),
      "pop" => Memory(Pop),

      "label" => Branching(Label),
      "goto" => Branching(Goto),
      "if-goto" => Branching(If),

      "function" => Function(Declare),
      "call" => Function(Call),
      "return" => Function(Return),

      _ => panic!("invalid command: '{}'", line),
    }
  }

  pub fn segment_type(segment: &str) -> MemorySegment {
    use MemorySegment::*;

    match segment {
      "argument" => Argument,
      "local" => Local,
      "static" => Static,
      "constant" => Constant,
      "this" => This,
      "that" => That,
      "pointer" => Pointer,
      "temp" => Temp,
      _ => panic!("invalid memory segment type: '{}'", segment),
    }
  }

  pub fn first_arg(line: &str) -> &str {
    line
      .split_whitespace()
      .nth(1)
      .expect(&format!("first argument required: '{}'", line))
  }

  pub fn second_arg(line: &str) -> usize {
    let argument = line
      .split_whitespace()
      .nth(2)
      .expect(&format!("second argument required: '{}'", line));

    argument
      .parse()
      .expect(&format!("argument is not an integer: '{}'", line))
  }
}

#[derive(PartialEq, Debug)]
pub enum MemorySegment {
  Argument,
  Local,
  Static,
  Constant,
  This,
  That,
  Pointer,
  Temp,
}

#[derive(PartialEq, Debug)]
pub enum CommandType {
  Math(MathCommand),
  Memory(MemoryCommand),
  Branching(BranchingCommand),
  Function(FunctionCommand),
}

#[derive(PartialEq, Debug)]
pub enum FunctionCommand {
  Declare,
  Return,
  Call,
}

#[derive(PartialEq, Debug)]
pub enum BranchingCommand {
  Label,
  Goto,
  If,
}

#[derive(PartialEq, Debug)]
pub enum MathCommand {
  Add,
  Subtract,
  Negate,
  EqualTo,
  GreaterThan,
  LessThan,
  And,
  Or,
  Not,
}

#[derive(PartialEq, Debug)]
pub enum MemoryCommand {
  Push,
  Pop,
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn creates_a_new_parser() {
    let mut p = Parser::new("First line\nthis is the second line\nfin");
    assert_eq!(p.lines.next().unwrap(), "First line");
    assert_eq!(p.lines.next().unwrap(), "this is the second line");
    assert_eq!(p.lines.next().unwrap(), "fin");
  }

  #[test]
  fn checks_for_more_lines() {
    let mut p = Parser::new("First line\nthis is the second line\nfin");
    assert_eq!(p.has_more_commands(), true);
    p.lines.next();
    p.lines.next();
    p.lines.next();
    assert_eq!(p.has_more_commands(), false);
  }

  #[test]
  fn advances_to_next_command() {
    let mut p = Parser::new("push constant 1\npush constant 2\nadd");
    assert_eq!(p.advance(), "push constant 1");
    assert_eq!(p.advance(), "push constant 2");
    assert_eq!(p.advance(), "add");
  }

  #[test]
  fn parses_correct_command_type() {
    assert_eq!(
      Parser::command_type("add"),
      CommandType::Math(MathCommand::Add)
    );

    assert_eq!(
      Parser::command_type("sub"),
      CommandType::Math(MathCommand::Subtract)
    );

    assert_eq!(
      Parser::command_type("push"),
      CommandType::Memory(MemoryCommand::Push)
    );
  }

  #[test]
  fn gets_first_command_arg() {
    let first = Parser::first_arg("push local 1");
    assert_eq!(first, "local");
  }

  #[test]
  fn gets_second_command_arg() {
    let second = Parser::second_arg("push local 1");
    assert_eq!(second, 1);
  }
}
