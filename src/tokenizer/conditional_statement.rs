use super::enumerators::Assignment;
use super::Token;


#[derive(Clone, Debug)]
pub struct ConditionalStatement {
    pub active_variables: Vec<usize>,
    pub condition_fields: Vec<(Option<Assignment>, Vec<Token>)>,
}
