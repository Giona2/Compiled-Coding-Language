use super::keyword_names;


pub enum Keywords {
    FUNCTION,
}

pub enum Syntax {
    RCURLY,
    LCURLY,
}

pub enum DataType {
    INTEGER,

} impl DataType {
    pub fn check_token_type(word_to_check: &str) -> Option<Self> { match word_to_check {
        keyword_names::types::INTEGER => Some(Self::INTEGER),
                                    _ => None
    }}
}

