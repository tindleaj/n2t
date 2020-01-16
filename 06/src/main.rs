mod code;
mod parser;

use parser::Parser;

pub fn main() {
  let path = std::env::args()
    .nth(1)
    .expect("Path to .asm file is required");

  Parser::parse(&path)
}
