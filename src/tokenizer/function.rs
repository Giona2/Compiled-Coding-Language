use super::enumerators::{Assignment, DataType};
use super::structures::{VariableHistory, Variable};
use super::Token;


#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub return_type: DataType,
    pub arguments: Vec<Variable>,
    pub variable_history: VariableHistory,
    pub functionaliy: Vec<Token>,
}

#[derive(Debug, Clone)]
pub struct Return {
    pub assignment: Assignment,
}
