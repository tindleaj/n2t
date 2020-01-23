mod code;
mod parser;
mod symbol_table;

use parser::Parser;
use std::fs;
use std::io::Write;

pub fn main() {
  let path = std::env::args()
    .nth(1)
    .expect("Path to .asm file is required");

  let out_name = std::env::args()
    .nth(2)
    .expect("Output file path is required");

  let input = fs::read_to_string(path).expect("problem reading path to string");
  let mut out_file = fs::File::create(out_name).expect("problem creating output file");

  let mut parser = Parser::new();
  let parsed = parser.parse(&input);
  out_file
    .write(&parsed)
    .expect("problem writing to output file");
}
