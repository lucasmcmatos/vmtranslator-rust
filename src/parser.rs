pub enum CommandType {
    CArithmetic,
    CPush,
    CPop,
}

pub struct Command {
    pub command_type: CommandType,
    pub arg1: String,
    pub arg2: Option<u16>,
}

pub struct Parser {
    commands: Vec<Command>,
    current: usize,
}

impl Parser {
    pub fn new(source: &str) -> Self {
        let commands = source
            .lines()
            .map(|line| match line.find("//") {
                Some(idx) => &line[..idx],
                None => line,
            })
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(Self::parse_line)
            .collect();

        Parser { commands, current: 0 }
    }

    pub fn has_more_commands(&self) -> bool {
        self.current < self.commands.len()
    }

    pub fn advance(&mut self) {
        self.current += 1;
    }

    pub fn current_command(&self) -> &Command {
        &self.commands[self.current]
    }

    fn parse_line(line: &str) -> Command {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "push" => Command {
                command_type: CommandType::CPush,
                arg1: parts[1].to_string(),
                arg2: Some(parts[2].parse().expect("push index must be a number")),
            },
            "pop" => Command {
                command_type: CommandType::CPop,
                arg1: parts[1].to_string(),
                arg2: Some(parts[2].parse().expect("pop index must be a number")),
            },
            cmd => Command {
                command_type: CommandType::CArithmetic,
                arg1: cmd.to_string(),
                arg2: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ignores_comments_and_blank_lines() {
        let source = "// this is a comment\n\npush constant 7\n";
        let parser = Parser::new(source);
        assert_eq!(parser.commands.len(), 1);
    }

    #[test]
    fn test_inline_comment_stripped() {
        let source = "push constant 7 // inline comment";
        let parser = Parser::new(source);
        assert_eq!(parser.commands.len(), 1);
        assert_eq!(parser.commands[0].arg2, Some(7));
    }

    #[test]
    fn test_push_classified() {
        let source = "push local 2";
        let parser = Parser::new(source);
        let cmd = &parser.commands[0];
        assert!(matches!(cmd.command_type, CommandType::CPush));
        assert_eq!(cmd.arg1, "local");
        assert_eq!(cmd.arg2, Some(2));
    }

    #[test]
    fn test_pop_classified() {
        let source = "pop argument 0";
        let parser = Parser::new(source);
        let cmd = &parser.commands[0];
        assert!(matches!(cmd.command_type, CommandType::CPop));
        assert_eq!(cmd.arg1, "argument");
        assert_eq!(cmd.arg2, Some(0));
    }

    #[test]
    fn test_arithmetic_classified() {
        let source = "add";
        let parser = Parser::new(source);
        let cmd = &parser.commands[0];
        assert!(matches!(cmd.command_type, CommandType::CArithmetic));
        assert_eq!(cmd.arg1, "add");
        assert!(cmd.arg2.is_none());
    }

    #[test]
    fn test_advance_and_has_more() {
        let source = "push constant 1\nadd";
        let mut parser = Parser::new(source);
        assert!(parser.has_more_commands());
        assert_eq!(parser.current_command().arg1, "constant");
        parser.advance();
        assert!(parser.has_more_commands());
        assert_eq!(parser.current_command().arg1, "add");
        parser.advance();
        assert!(!parser.has_more_commands());
    }
}
