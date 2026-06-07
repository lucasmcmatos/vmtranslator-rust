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
