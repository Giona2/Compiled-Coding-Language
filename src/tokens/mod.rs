pub mod constructors;
pub mod data;
pub mod types;
    use types::*;

pub enum Token {
    KEYWORDS(Keywords),
    SYNTAX(Syntax),
    DATATYPES(DataType),
}

