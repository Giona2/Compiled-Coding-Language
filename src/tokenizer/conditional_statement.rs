use super::enumerators::Assignment;
use super::function::Function;
use super::Token;


#[derive(Clone, Debug)]
pub struct ConditionalStatement {
    pub parent: Function,
    pub index: usize,
    pub active_variables: Vec<usize>,
    pub condition_fields: Vec<(Option<Assignment>, Vec<Token>)>,
}
