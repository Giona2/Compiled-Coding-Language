use super::enumerators::Assignment;
use super::Token;


#[derive(Clone, Debug)]
pub struct ConditionalLoop {
    pub condition: Assignment,
    pub functionality: Vec<Token>,
}
