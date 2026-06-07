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
            "and" => self.write_binary_op("M=M&D"),
            "or"  => self.write_binary_op("M=M|D"),
            "neg" => self.write_unary_op("M=-M"),
            "not" => self.write_unary_op("M=!M"),
            "eq"  => self.write_comparison("JEQ"),
            "gt"  => self.write_comparison("JGT"),
            "lt"  => self.write_comparison("JLT"),
            _ => panic!("unknown arithmetic command: {}", command),
        }
    }

    pub fn write_push_pop(&mut self, cmd_type: &CommandType, segment: &str, index: u16) {
        match cmd_type {
            CommandType::CPush => {
                self.output.push(format!("// push {} {}", segment, index));
                match segment {
                    "constant"             => self.write_push_constant(index),
                    "local"                => self.write_push_segment("LCL", index),
                    "argument"             => self.write_push_segment("ARG", index),
                    "this"                 => self.write_push_segment("THIS", index),
                    "that"                 => self.write_push_segment("THAT", index),
                    "temp"                 => self.write_push_temp(index),
                    _ => panic!("unsupported push segment: {}", segment),
                }
            }
            CommandType::CPop => {
                self.output.push(format!("// pop {} {}", segment, index));
                match segment {
                    "local"                => self.write_pop_segment("LCL", index),
                    "argument"             => self.write_pop_segment("ARG", index),
                    "this"                 => self.write_pop_segment("THIS", index),
                    "that"                 => self.write_pop_segment("THAT", index),
                    "temp"                 => self.write_pop_temp(index),
                    _ => panic!("unsupported pop segment: {}", segment),
                }
            }
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

    fn write_comparison(&mut self, jump: &str) {
        let n = self.label_counter;
        self.label_counter += 1;
        let true_label = format!("TRUE_{}", n);
        let end_label  = format!("END_{}", n);
        self.emit(&[
            "@SP",
            "M=M-1",
            "A=M",
            "D=M",
            "A=A-1",
            "D=M-D",
            &format!("@{}", true_label),
            &format!("D;{}", jump),
            "@SP",
            "A=M-1",
            "M=0",
            &format!("@{}", end_label),
            "0;JMP",
            &format!("({})", true_label),
            "@SP",
            "A=M-1",
            "M=-1",
            &format!("({})", end_label),
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

    fn write_push_segment(&mut self, base: &str, index: u16) {
        self.emit(&[
            &format!("@{}", index),
            "D=A",
            &format!("@{}", base),
            "A=D+M",
            "D=M",
            "@SP",
            "A=M",
            "M=D",
            "@SP",
            "M=M+1",
        ]);
    }

    fn write_pop_segment(&mut self, base: &str, index: u16) {
        self.emit(&[
            &format!("@{}", index),
            "D=A",
            &format!("@{}", base),
            "D=D+M",
            "@R13",
            "M=D",
            "@SP",
            "M=M-1",
            "A=M",
            "D=M",
            "@R13",
            "A=M",
            "M=D",
        ]);
    }

    fn write_push_temp(&mut self, index: u16) {
        self.emit(&[
            &format!("@R{}", 5 + index),
            "D=M",
            "@SP",
            "A=M",
            "M=D",
            "@SP",
            "M=M+1",
        ]);
    }

    fn write_pop_temp(&mut self, index: u16) {
        self.emit(&[
            "@SP",
            "M=M-1",
            "A=M",
            "D=M",
            &format!("@R{}", 5 + index),
            "M=D",
        ]);
    }

    fn emit(&mut self, lines: &[&str]) {
        for line in lines {
            self.output.push(line.to_string());
        }
    }
}
