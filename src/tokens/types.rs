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
// equation!("3", ADD, "4")
// equation!(equation!("3", ADD, "4"), MULTIPLY, "4")
macro_rules! equation  {
    ($first_constant:ident, $operation:ident, $second_constant:ident) => {
        Box::new(Assignment::$operation((
            Box::new(Assignment::$first_constant),
            Box::new(Assignment::$second_constant),
        )))
    };
}

#[derive(Debug)]
pub enum Assignment {
    ADD((Box<Assignment>, Box<Assignment>)),
    SUBTRACT((Box<Assignment>, Box<Assignment>)),
    MULTIPLY((Box<Assignment>, Box<Assignment>)),
    DIVIDE((Box<Assignment>, Box<Assignment>)),
    TERM(String),
}
