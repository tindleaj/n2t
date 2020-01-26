pub struct Parser {
  input: String,
}

impl Parser {
  pub fn new() -> Parser {
    unimplemented!()
  }

  fn has_more_commands() -> bool {
    unimplemented!()
  }

  fn advance(&self) {
    unimplemented!()
  }

  fn command_type() -> CommandType {
    unimplemented!()
  }

  fn arg(n: usize) {
    unimplemented!()
  }
}

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

pub enum CommandType {
  Arithemtic(ArithmeticCommand),
  Memory(MemoryCommand),
  Branching(BranchingCommand),
  Function(FunctionCommand),
}

pub enum FunctionCommand {
  Function,
  Return,
  Call,
}

pub enum BranchingCommand {
  Label,
  Goto,
  If,
}

pub enum ArithmeticCommand {
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

pub enum MemoryCommand {
  Push,
  Pop,
}
