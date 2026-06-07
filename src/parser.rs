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
