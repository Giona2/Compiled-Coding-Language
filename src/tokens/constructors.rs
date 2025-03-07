use super::types::DataType;
use super::Token;


#[derive(Debug)]
pub struct Function {
    name: String,
    return_type: DataType,
    args: Vec<String>,
    functionaliy: Vec<Token>,

} impl Function {
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
    name: String,
    data_type: DataType,
    value: Option<String>,
} impl Declaration {
    pub fn new(name: &str, data_type: DataType, value: Option<&str>) -> Self { return Self {
        name: name.to_string(),
        data_type,
        value: if let Some(unwrapped_value) = value { Some(unwrapped_value.to_string()) } else { None }
    }}
}
