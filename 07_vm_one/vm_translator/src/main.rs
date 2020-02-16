mod parser;
mod writer;

use std::env::args;

/// Requires two arguments, a `.vm` file as input, such as `input.vm`
/// and a `.asm` file to output to, such as `output/bin.asm`
fn main() {
    let input_path = args().nth(1).expect("must supply an input path");
    let output_path = args().nth(2).expect("must supply an output file path");

    lib::compile_to_target(&input_path, &output_path);
}

mod lib {
    use crate::parser;
    use crate::parser::Parser;
    use crate::writer::Writer;
    use std::fs;
    use std::io::{Result as IOResult, Write};
    use std::path::Path;

    pub fn parse(contents: &str) -> Vec<u8> {
        use parser::CommandType;

        let mut parser = Parser::new(contents);
        let mut writer = Writer::new();

        while parser.has_more_commands() {
            let line = parser.advance();

            if line.starts_with("//") || line.is_empty() {
                continue;
            }

            let command_type = Parser::command_type(line);

            match command_type {
                CommandType::Math(command) => writer.write_math(command),
                CommandType::Memory(command) => {
                    let first_arg = Parser::first_arg(line);
                    let segment = Parser::segment_type(first_arg);
                    let index = Parser::second_arg(line);

                    writer.write_push_pop(command, segment, index)
                }
                _ => unimplemented!(),
            }
        }

        writer.output
    }

    pub fn compile_to_target(input_path: &str, output_path: &str) {
        let input_path = Path::new(input_path);
        let mut source_string = String::new();

        if Path::is_dir(input_path) {
            let mut source = Vec::new();

            fs::read_dir(input_path)
                .unwrap()
                .for_each(|result: IOResult<fs::DirEntry>| {
                    let entry = result.unwrap();
                    let file_name = entry.file_name();
                    let path = entry.path();

                    if !file_name.to_str().unwrap().ends_with(".vm") {
                        panic!("all source files must have the '.vm' extension")
                    }

                    let contents =
                        fs::read_to_string(path).expect("problem reading file contents to string");

                    write!(source, "{}", &contents);
                });

            source_string = String::from_utf8(source).unwrap();
        } else {
            source_string =
                std::fs::read_to_string(input_path).expect("problem reading contents of file");
        }

        let output = parse(&source_string);

        fs::File::create(output_path)
            .expect("problem creating output file")
            .write(&output)
            .expect("problem writing buffer to output file");
    }

    #[cfg(test)]
    mod tests {
        use super::*;
    }
}
