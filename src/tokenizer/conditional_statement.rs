use super::enumerators::Assignment;
use super::function::Function;
use super::Token;


#[derive(Clone, Debug)]
pub struct ConditionalStatement {
    pub parent: Function,
    pub index: usize,
    pub comparison_value: Assignment,
    pub condition_fields: Vec<(Option<Assignment>, Vec<Token>)>,
}
