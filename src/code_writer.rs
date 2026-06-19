use crate::parser::CommandType;

pub struct CodeWriter {
    output: Vec<String>,
    file_name: String,
    label_counter: u32,
    current_function: String,
    return_counter: u32,
}

impl CodeWriter {
    pub fn new(file_name: &str) -> Self {
        CodeWriter {
            output: Vec::new(),
            file_name: file_name.to_string(),
            label_counter: 0,
            current_function: String::new(),
            return_counter: 0,
        }
    }

    pub fn set_file_name(&mut self, name: &str) {
        self.file_name = name.to_string();
    }

    pub fn write_label(&mut self, label: &str) {
        self.output.push(format!("// label {}", label));
        let scoped = self.scoped_label(label);
        self.emit(&[&format!("({})", scoped)]);
    }

    pub fn write_goto(&mut self, label: &str) {
        self.output.push(format!("// goto {}", label));
        let scoped = self.scoped_label(label);
        self.emit(&[
            &format!("@{}", scoped),
            "0;JMP",
        ]);
    }

    pub fn write_if(&mut self, label: &str) {
        self.output.push(format!("// if-goto {}", label));
        let scoped = self.scoped_label(label);
        self.emit(&[
            "@SP",
            "AM=M-1",
            "D=M",
            &format!("@{}", scoped),
            "D;JNE",
        ]);
    }

    pub fn write_function(&mut self, name: &str, n_locals: u16) {
        self.output.push(format!("// function {} {}", name, n_locals));
        self.current_function = name.to_string();
        self.emit(&[&format!("({})", name)]);
        for _ in 0..n_locals {
            self.emit(&[
                "@SP",
                "A=M",
                "M=0",
                "@SP",
                "M=M+1",
            ]);
        }
    }

    pub fn write_call(&mut self, name: &str, n_args: u16) {
        self.output.push(format!("// call {} {}", name, n_args));
        let ret_label = format!("{}$ret.{}", name, self.return_counter);
        self.return_counter += 1;

        // push return address
        self.emit(&[
            &format!("@{}", ret_label),
            "D=A",
            "@SP",
            "A=M",
            "M=D",
            "@SP",
            "M=M+1",
        ]);

        // push LCL, ARG, THIS, THAT (save caller frame)
        for reg in &["LCL", "ARG", "THIS", "THAT"] {
            self.emit(&[
                &format!("@{}", reg),
                "D=M",
                "@SP",
                "A=M",
                "M=D",
                "@SP",
                "M=M+1",
            ]);
        }

        // ARG = SP - 5 - nArgs
        self.emit(&[
            "@SP",
            "D=M",
            "@5",
            "D=D-A",
            &format!("@{}", n_args),
            "D=D-A",
            "@ARG",
            "M=D",
        ]);

        // LCL = SP
        self.emit(&[
            "@SP",
            "D=M",
            "@LCL",
            "M=D",
        ]);

        // goto callee
        self.emit(&[
            &format!("@{}", name),
            "0;JMP",
        ]);

        // plant return address label
        self.emit(&[&format!("({})", ret_label)]);
    }

    pub fn write_return(&mut self) {
        todo!("write_return not yet implemented")
    }

    pub fn write_arithmetic(&mut self, command: &str) {
        self.output.push(format!("// {}", command));
        match command {
            "add" => self.write_binary_op("M=D+M"),
            "sub" => self.write_binary_op("M=M-D"),
            "and" => self.write_binary_op("M=D&M"),
            "or"  => self.write_binary_op("M=D|M"),
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
                    "pointer"              => self.write_push_pointer(index),
                    "static"               => self.write_push_static(index),
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
                    "pointer"              => self.write_pop_pointer(index),
                    "static"               => self.write_pop_static(index),
                    _ => panic!("unsupported pop segment: {}", segment),
                }
            }
            CommandType::CArithmetic => unreachable!(),
            _ => unreachable!(),
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

    fn write_push_static(&mut self, index: u16) {
        let label = format!("{}.{}", self.file_name, index);
        self.emit(&[
            &format!("@{}", label),
            "D=M",
            "@SP",
            "A=M",
            "M=D",
            "@SP",
            "M=M+1",
        ]);
    }

    fn write_pop_static(&mut self, index: u16) {
        let label = format!("{}.{}", self.file_name, index);
        self.emit(&[
            "@SP",
            "M=M-1",
            "A=M",
            "D=M",
            &format!("@{}", label),
            "M=D",
        ]);
    }

    fn pointer_reg(index: u16) -> &'static str {
        if index == 0 { "THIS" } else { "THAT" }
    }

    fn write_push_pointer(&mut self, index: u16) {
        let reg = Self::pointer_reg(index);
        self.emit(&[
            &format!("@{}", reg),
            "D=M",
            "@SP",
            "A=M",
            "M=D",
            "@SP",
            "M=M+1",
        ]);
    }

    fn write_pop_pointer(&mut self, index: u16) {
        let reg = Self::pointer_reg(index);
        self.emit(&[
            "@SP",
            "M=M-1",
            "A=M",
            "D=M",
            &format!("@{}", reg),
            "M=D",
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

    fn scoped_label(&self, label: &str) -> String {
        if self.current_function.is_empty() {
            label.to_string()
        } else {
            format!("{}${}", self.current_function, label)
        }
    }

    fn emit(&mut self, lines: &[&str]) {
        for line in lines {
            self.output.push(line.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn asm(writer: &CodeWriter) -> Vec<&str> {
        writer.output().iter().map(|s| s.as_str()).collect()
    }

    #[test]
    fn test_write_label_scoped() {
        let mut cw = CodeWriter::new("Test");
        cw.current_function = "Main.main".to_string();
        cw.write_label("LOOP");
        let out = asm(&cw);
        assert!(out.contains(&"(Main.main$LOOP)"));
    }

    #[test]
    fn test_write_label_no_function() {
        let mut cw = CodeWriter::new("Test");
        cw.write_label("TOP");
        let out = asm(&cw);
        assert!(out.contains(&"(TOP)"));
    }

    #[test]
    fn test_write_goto_scoped() {
        let mut cw = CodeWriter::new("Test");
        cw.current_function = "Main.main".to_string();
        cw.write_goto("LOOP");
        let out = asm(&cw);
        assert!(out.contains(&"@Main.main$LOOP"));
        assert!(out.contains(&"0;JMP"));
    }

    #[test]
    fn test_write_call_pushes_return_address() {
        let mut cw = CodeWriter::new("Test");
        cw.write_call("Foo.bar", 2);
        let out = asm(&cw);
        assert!(out.contains(&"@Foo.bar$ret.0"));
        assert!(out.contains(&"D=A"));
    }

    #[test]
    fn test_write_call_saves_caller_frame() {
        let mut cw = CodeWriter::new("Test");
        cw.write_call("Foo.bar", 0);
        let asm_str = cw.output().join("\n");
        assert!(asm_str.contains("@LCL"));
        assert!(asm_str.contains("@ARG"));
        assert!(asm_str.contains("@THIS"));
        assert!(asm_str.contains("@THAT"));
    }

    #[test]
    fn test_write_call_repositions_arg() {
        let mut cw = CodeWriter::new("Test");
        cw.write_call("Foo.bar", 3);
        let asm_str = cw.output().join("\n");
        // ARG = SP - 5 - nArgs: expect @5 and @3
        assert!(asm_str.contains("@5\nD=D-A\n@3\nD=D-A\n@ARG\nM=D"));
    }

    #[test]
    fn test_write_call_sets_lcl_and_jumps() {
        let mut cw = CodeWriter::new("Test");
        cw.write_call("Foo.bar", 0);
        let asm_str = cw.output().join("\n");
        assert!(asm_str.contains("@LCL\nM=D"));
        assert!(asm_str.contains("@Foo.bar\n0;JMP"));
    }

    #[test]
    fn test_write_call_plants_return_label() {
        let mut cw = CodeWriter::new("Test");
        cw.write_call("Foo.bar", 0);
        let out = asm(&cw);
        assert!(out.contains(&"(Foo.bar$ret.0)"));
    }

    #[test]
    fn test_write_call_return_counter_increments() {
        let mut cw = CodeWriter::new("Test");
        cw.write_call("Foo.bar", 0);
        cw.write_call("Foo.bar", 0);
        let out = asm(&cw);
        assert!(out.contains(&"(Foo.bar$ret.0)"));
        assert!(out.contains(&"(Foo.bar$ret.1)"));
    }

    #[test]
    fn test_write_function_sets_current_function() {
        let mut cw = CodeWriter::new("Test");
        cw.write_function("Main.main", 0);
        assert_eq!(cw.current_function, "Main.main");
    }

    #[test]
    fn test_write_function_emits_label() {
        let mut cw = CodeWriter::new("Test");
        cw.write_function("Main.main", 0);
        let out = asm(&cw);
        assert!(out.contains(&"(Main.main)"));
    }

    #[test]
    fn test_write_function_initializes_locals() {
        let mut cw = CodeWriter::new("Test");
        cw.write_function("Main.main", 2);
        let asm_str = cw.output().join("\n");
        // two pushes of 0: count occurrences of M=0
        assert_eq!(asm_str.matches("M=0").count(), 2);
    }

    #[test]
    fn test_write_function_zero_locals_no_init() {
        let mut cw = CodeWriter::new("Test");
        cw.write_function("Main.main", 0);
        let asm_str = cw.output().join("\n");
        assert!(!asm_str.contains("M=0"));
    }

    #[test]
    fn test_write_function_scopes_subsequent_labels() {
        let mut cw = CodeWriter::new("Test");
        cw.write_function("Foo.bar", 0);
        cw.write_label("LOOP");
        let out = asm(&cw);
        assert!(out.contains(&"(Foo.bar$LOOP)"));
    }

    #[test]
    fn test_write_if_scoped() {
        let mut cw = CodeWriter::new("Test");
        cw.current_function = "Main.main".to_string();
        cw.write_if("LOOP");
        let out = asm(&cw);
        assert!(out.contains(&"@SP"));
        assert!(out.contains(&"AM=M-1"));
        assert!(out.contains(&"D=M"));
        assert!(out.contains(&"@Main.main$LOOP"));
        assert!(out.contains(&"D;JNE"));
    }
}
