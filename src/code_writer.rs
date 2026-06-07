use crate::parser::CommandType;

pub struct CodeWriter {
    output: Vec<String>,
    file_name: String,
    label_counter: u32,
}
