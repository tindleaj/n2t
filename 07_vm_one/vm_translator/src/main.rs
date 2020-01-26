mod lexer;
mod parser;
mod writer;

use std::env;

/// Requires two arguments, a `.vm` file as input, such as `input.vm`
/// and a `.asm` file to output to, such as `output/bin.asm`
fn main() {
    let input = env::args();
}
