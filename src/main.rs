mod parser;
mod code_writer;

use std::fs;
use std::path::Path;
use parser::Parser;
use code_writer::CodeWriter;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: vm_translator <file.vm>");
        std::process::exit(1);
    }

    let input_path = Path::new(&args[1]);
    let source = fs::read_to_string(input_path)
        .unwrap_or_else(|e| { eprintln!("Error reading file: {}", e); std::process::exit(1); });

    let file_name = input_path
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();

    let mut parser = Parser::new(&source);
    let mut writer = CodeWriter::new(file_name);

    while parser.has_more_commands() {
        let cmd = parser.current_command();
        match cmd.command_type {
            parser::CommandType::CArithmetic => writer.write_arithmetic(&cmd.arg1),
            parser::CommandType::CPush | parser::CommandType::CPop => {
                writer.write_push_pop(&cmd.command_type, &cmd.arg1, cmd.arg2.unwrap());
            }
        }
        parser.advance();
    }

    let output_path = input_path.with_extension("asm");
    let output = writer.output().join("\n") + "\n";
    fs::write(&output_path, output)
        .unwrap_or_else(|e| { eprintln!("Error writing file: {}", e); std::process::exit(1); });

    println!("Written: {}", output_path.display());
}
