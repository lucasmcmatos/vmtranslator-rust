mod parser;
mod code_writer;

use std::fs;
use std::path::{Path, PathBuf};
use parser::Parser;
use code_writer::CodeWriter;

fn translate_file(source: &str, writer: &mut CodeWriter) {
    let mut parser = Parser::new(source);
    while parser.has_more_commands() {
        let cmd = parser.current_command();
        let arg1 = cmd.arg1.clone();
        let arg2 = cmd.arg2;
        match cmd.command_type {
            parser::CommandType::CArithmetic => writer.write_arithmetic(&arg1),
            parser::CommandType::CPush | parser::CommandType::CPop => {
                writer.write_push_pop(&cmd.command_type, &arg1, arg2.unwrap());
            }
            parser::CommandType::CLabel    => writer.write_label(&arg1),
            parser::CommandType::CGoto     => writer.write_goto(&arg1),
            parser::CommandType::CIf       => writer.write_if(&arg1),
            parser::CommandType::CFunction => writer.write_function(&arg1, arg2.unwrap()),
            parser::CommandType::CCall     => writer.write_call(&arg1, arg2.unwrap()),
            parser::CommandType::CReturn   => writer.write_return(),
        }
        parser.advance();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: vmtranslator <file.vm | directory>");
        std::process::exit(1);
    }

    let input_path = Path::new(&args[1]);

    if input_path.is_dir() {
        translate_dir(input_path);
    } else {
        translate_single_file(input_path);
    }
}

fn translate_dir(dir: &Path) {
    let dir_name = dir.file_name().unwrap().to_str().unwrap();

    let mut vm_files: Vec<PathBuf> = fs::read_dir(dir)
        .unwrap_or_else(|e| { eprintln!("Error reading directory: {}", e); std::process::exit(1); })
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("vm"))
        .collect();

    if vm_files.is_empty() {
        eprintln!("No .vm files found in {}", dir.display());
        std::process::exit(1);
    }

    vm_files.sort();

    let mut writer = CodeWriter::new(dir_name);
    writer.write_bootstrap();

    for vm_file in &vm_files {
        let file_stem = vm_file.file_stem().unwrap().to_str().unwrap();
        writer.set_file_name(file_stem);
        let source = fs::read_to_string(vm_file)
            .unwrap_or_else(|e| { eprintln!("Error reading {}: {}", vm_file.display(), e); std::process::exit(1); });
        translate_file(&source, &mut writer);
    }

    let output_path = dir.join(dir_name).with_extension("asm");
    write_output(&writer, &output_path);
}

fn translate_single_file(input_path: &Path) {
    let file_name = input_path.file_stem().unwrap().to_str().unwrap();
    let source = fs::read_to_string(input_path)
        .unwrap_or_else(|e| { eprintln!("Error reading file: {}", e); std::process::exit(1); });

    let mut writer = CodeWriter::new(file_name);
    translate_file(&source, &mut writer);

    let input_dir = input_path.parent().unwrap_or(Path::new("."));
    let output_dir = input_dir.join("output");
    fs::create_dir_all(&output_dir)
        .unwrap_or_else(|e| { eprintln!("Error creating output dir: {}", e); std::process::exit(1); });

    let output_path = output_dir.join(file_name).with_extension("asm");
    write_output(&writer, &output_path);
}

fn write_output(writer: &CodeWriter, path: &Path) {
    let output = writer.output().join("\n") + "\n";
    fs::write(path, output)
        .unwrap_or_else(|e| { eprintln!("Error writing file: {}", e); std::process::exit(1); });
    println!("Written: {}", path.display());
}
