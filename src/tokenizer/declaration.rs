use super::enumerators::Assignment;
use crate::data::SyntaxElements;


#[derive(Debug, Clone)]
pub enum DataType {
    INTEGER,
    FLOAT,
    BOOL,
} impl DataType {
    pub fn check_token_type(word_to_check: &str) -> Option<Self> { let syntax_elements = SyntaxElements::init(); match word_to_check {
        val if val == syntax_elements.type_names["integer"] => Some(Self::INTEGER),
        val if val == syntax_elements.type_names["float"]   => Some(Self::FLOAT),
                                                          _ => None,
    }}

    pub fn is_integer(&self) -> bool {
        if let Self::INTEGER = self {
            return true
        } else {
            return false
        }
    }
    pub fn is_float(&self) -> bool {
        if let Self::FLOAT = self {
            return true
        } else {
            return false
        }
    }
}


#[derive(Debug, Clone)]
pub struct Declaration {
    pub name: String,
    pub location: usize,
    pub data_type: DataType,
    pub value: Assignment,
}
