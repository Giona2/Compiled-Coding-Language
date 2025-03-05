pub mod constructors;
pub mod keyword_names;
pub mod types;
    use types::*;

pub enum Token {
    KEYWORDS(Keywords),
    SYNTAX(Syntax),
    DATATYPES(DataType),
}

