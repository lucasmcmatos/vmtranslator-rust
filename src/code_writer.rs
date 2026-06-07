use crate::parser::CommandType;

pub struct CodeWriter {
    output: Vec<String>,
    file_name: String,
    label_counter: u32,
}

impl CodeWriter {
    pub fn new(file_name: &str) -> Self {
        CodeWriter {
            output: Vec::new(),
            file_name: file_name.to_string(),
            label_counter: 0,
        }
    }

    pub fn write_arithmetic(&mut self, command: &str) {
        self.output.push(format!("// {}", command));
        match command {
            "add" => self.write_binary_op("M=M+D"),
            "sub" => self.write_binary_op("M=M-D"),
            "neg" => self.write_unary_op("M=-M"),
            _ => panic!("unknown arithmetic command: {}", command),
        }
    }

    pub fn write_push_pop(&mut self, cmd_type: &CommandType, segment: &str, index: u16) {
        match cmd_type {
            CommandType::CPush => {
                self.output.push(format!("// push {} {}", segment, index));
                match segment {
                    "constant" => self.write_push_constant(index),
                    _ => panic!("unsupported push segment: {}", segment),
                }
            }
            CommandType::CPop => panic!("unsupported pop segment: {}", segment),
            CommandType::CArithmetic => unreachable!(),
        }
    }

    pub fn output(&self) -> &[String] {
        &self.output
    }

    fn write_push_constant(&mut self, value: u16) {
        self.emit(&[
            &format!("@{}", value),
            "D=A",
            "@SP",
            "A=M",
            "M=D",
            "@SP",
            "M=M+1",
        ]);
    }

    fn write_binary_op(&mut self, op: &str) {
        self.emit(&[
            "@SP",
            "M=M-1",
            "A=M",
            "D=M",
            "A=A-1",
            op,
        ]);
    }

    fn write_unary_op(&mut self, op: &str) {
        self.emit(&[
            "@SP",
            "A=M-1",
            op,
        ]);
    }

    fn emit(&mut self, lines: &[&str]) {
        for line in lines {
            self.output.push(line.to_string());
        }
    }
}
