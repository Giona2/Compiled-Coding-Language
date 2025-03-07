use super::keyword_names;


#[derive(Debug)]
pub enum DataType {
    INTEGER,

} impl DataType {
    pub fn check_token_type(word_to_check: &str) -> Option<Self> { match word_to_check {
        keyword_names::types::INTEGER => Some(Self::INTEGER),
                                    _ => None
    }}
}


#[macro_export]
macro_rules! equation  {
    ($first_term:literal, $operation:ident, $second_term:literal) => {
        Box::new(Assignment::$operation((
            Box::new(Assignment::TERM($first_term.to_string())),
            Box::new(Assignment::TERM($second_term.to_string())),
        )))
    };

    ($first_term:expr, $operation:ident, $second_term:literal) => {
        Box::new(Assignment::$operation((
            $first_term,
            Box::new(Assignment::TERM($second_term.to_string())),
        )))
    };

    ($first_term:literal, $operation:ident, $second_term:expr) => {
        Box::new(Assignment::$operation((
            Box::new(Assignment::TERM($first_term.to_string())),
            $second_term,
        )))
    };

    ($first_term:expr, $operation:ident, $second_term:expr) => {
        Box::new(Assignment::$operation((
            $first_term,
            $second_term,
        )))
    };
}

#[derive(Debug)]
pub enum Assignment {
    ADD((Box<Assignment>, Box<Assignment>)),
    SUB((Box<Assignment>, Box<Assignment>)),
    MUL((Box<Assignment>, Box<Assignment>)),
    DIV((Box<Assignment>, Box<Assignment>)),
    TERM(String),
}
