use super::enumerators::DataType;
use super::structures::{VariableHistory, Variable};
use super::Token;


#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub return_type: DataType,
    pub arguments: Vec<Variable>,
    pub variable_history: VariableHistory,
    pub functionaliy: Vec<Token>,
}
