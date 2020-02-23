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

    pub fn parse(contents: &str, namespace: &str) -> Vec<u8> {
        use parser::CommandType;

        let mut parser = Parser::new(contents);
        let mut writer = Writer::new(namespace);

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
        let mut output_buffer = Vec::new();

        if Path::is_dir(input_path) {
            fs::read_dir(input_path)
                .unwrap()
                .for_each(|result: IOResult<fs::DirEntry>| {
                    let entry = result.unwrap();
                    let entry_path = entry.path();
                    let os_file_name = entry.file_name();
                    let path = entry.path();
                    let file_name = os_file_name.to_str().expect("problem getting filename");

                    let file_stem = entry_path
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .expect("problem getting file_stem");

                    if !file_name.ends_with(".vm") {
                        panic!("all source files must have the '.vm' extension")
                    }

                    let contents =
                        fs::read_to_string(path).expect("problem reading file contents to string");

                    output_buffer.append(&mut parse(&contents, file_stem));
                });
        } else {
            let file_stem = input_path
                .file_stem()
                .unwrap()
                .to_str()
                .expect("problem getting filename");
            let source =
                std::fs::read_to_string(input_path).expect("problem reading contents of file");

            output_buffer.append(&mut parse(&source, file_stem));
        }

        fs::File::create(output_path)
            .expect("problem creating output file")
            .write(&output_buffer)
            .expect("problem writing buffer to output file");
    }

    #[cfg(test)]
    mod tests {
        use super::*;
    }
}
