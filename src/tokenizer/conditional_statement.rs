use super::enumerators::Assignment;
use super::Token;


#[derive(Clone, Debug)]
pub struct ConditionalStatement {
    pub condition: Assignment,
    pub functionality: Vec<Token>
}
