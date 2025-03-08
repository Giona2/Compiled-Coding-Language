use super::Token;
use super::types::Assignment;
use super::types::DataType;


#[derive(Debug)]
pub struct Function {
    name: String,
    return_type: DataType,
    args: Vec<String>,
    functionaliy: Vec<Token>,

} impl Function {
    #[allow(dead_code)]
    pub fn new(name: &str, return_type: DataType, args: Vec<&str>) -> Self { return Self {
        name: name.to_string(),
        return_type,
        args: args.into_iter().map(|x| x.to_string()).collect(),
        functionaliy: Vec::new(),
    }}
}

#[derive(Debug)]
pub struct TerminatingLoop {}

#[derive(Debug)]
pub struct Declaration {
    pub name: String,
    pub data_type: DataType,
    pub value: Option<Assignment>,

}
