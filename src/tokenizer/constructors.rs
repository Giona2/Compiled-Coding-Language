use super::representations::StackMemory;
use super::Token;
use super::types::Assignment;
use super::types::DataType;


#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub return_type: DataType,
    pub args: Vec<Argument>,
    pub memory: StackMemory,
    pub functionaliy: Vec<Token>,
}
#[derive(Debug, Clone)]
pub struct Argument {
    pub name: String,
    pub data_type: DataType,
} impl Argument {
    pub fn from_string_vec(from: Vec<String>) -> Self { println!("coding_language::tokenizer::constructors::Argument::from_string_vec()\n  recieved: {:?}", from); Self {
        name: from[1].clone(),
        data_type: DataType::check_token_type(&from[0]).unwrap(),
    }}
}


#[derive(Debug)]
pub struct TerminatingLoop {}

#[derive(Debug, Clone)]
pub struct Declaration {
    pub name: String,
    pub location: usize,
    pub data_type: DataType,
    pub value: Option<Assignment>,
}
