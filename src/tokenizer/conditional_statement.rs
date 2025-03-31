use super::enumerators::Assignment;
use super::function::Function;
use super::Token;


#[derive(Clone, Debug)]
pub struct ConditionalStatement {
    pub parent: Function,
    pub index: usize,
    pub condition: Assignment,
    pub functionality: Vec<Token>
}
