use super::error::TokensError;
use super::keyword_names;
use super::representations::StackMemory;


#[derive(Debug, Clone)]
pub enum DataType {
    INTEGER,

} impl DataType {
    pub fn check_token_type(word_to_check: &str) -> Option<Self> { match word_to_check {
        keyword_names::types::INTEGER => Some(Self::INTEGER),
                                    _ => None
    }}
}


/// Used in conjunction with Assignment to represent an arithmatic term
///
/// LITERAL(String) -> literal number value (`3` in `x + 3`)
/// VARIABLE(usize) -> variable value (`x` in `x + 3`) where usize is the stack memory slot where
///                    the variable is contained
#[derive(Debug)]
pub enum Term {
    LITERAL(String),
    VARIABLE(usize),
}

/// A shorthand method to build an Assignment enumerator
#[macro_export]
macro_rules! eq {
    ($first_term_branch:ident($first_term:expr), $operation:ident, $second_term_branch:ident($second_term:expr)) => {
        Box::new(Assignment::$operation(
            Box::new(Assignment::$first_term_branch($first_term.to_string())),
            Box::new(Assignment::$second_term_branch($second_term.to_string())),
        ))
    };
    ($first_equation:expr, $operation:ident, $second_term_branch:ident($second_term:expr)) => {
        Box::new(Assignment::$operation(
            $first_equation,
            Box::new(Assignment::$second_term_branch($second_term.to_string())),
        ))
    };
    ($first_term_branch:ident($first_term:expr), $operation:ident, $second_term:expr) => {
        Box::new(Assignment::$operation(
            Box::new(Assignment::$first_term_branch($first_term.to_string())),
            $second_term,
        ))
    };
    ($first_term:expr, $operation:ident, $second_term:expr) => {
        Box::new(Assignment::$operation(
            $first_term,
            $second_term,
        ))
    };
}

/// A representation of the series of steps the program needs to run to equate a `Declaration`
///
/// Each branch represents an arithmatic expression that can be calculated by the CPU registers
///
/// Note that the recommended method to build an Assignment is by using the
/// equation_cc/_ec/_ce/_ee! macros
///
/// ADD(TERM("1"), TERM("2")) -> add 1 to 2
/// SUB(TERM("1"), TERM("2")) -> subtract 1 from 2
/// MUL(TERM("1"), TERM("2")) -> multiply 1 to 2
/// DIV(TERM("1"), TERM("2")) -> divide 1 by 2
/// TERM("1")                 -> represents a constant (a number litteral)
#[derive(Debug)]
pub enum Assignment {
    ADD(Box<Assignment>, Box<Assignment>),
    SUB(Box<Assignment>, Box<Assignment>),
    MUL(Box<Assignment>, Box<Assignment>),
    DIV(Box<Assignment>, Box<Assignment>),
    CONST(String),
    VAR(usize),

} impl Assignment {
    /// Converts a formatted Vec<String> to a nested Declaration orderded using PEMDAS. Each element in the list should
    /// be seperated each symbol/element in an equation
    /// `3 + 4 / 6` -> `["3", "+", "4", "/", "6"]`
    /// Note that this is primarily meant for parsing variable declarations
    ///
    /// # Examples
    ///
    /// ```rust
    /// let equation: Vec<String> = vec!["3", "+", "4", "/", "6"]
    ///     .into_iter().map(|x| x.to_string()).collect()
    /// let declaration = Assignment::from_string_vec(equation)
    ///
    /// // output: ADD((TERM(3), DIV(TERM(4), TERM(6))))
    /// println!("{declaration:?}");
    /// ```
    pub fn from_string_vec(stack_memory: &StackMemory, string_equation: Vec<String>) -> Result<Self, TokensError> {
        if string_equation.len() == 1 {
            return Ok(Self::CONST(string_equation[0].clone()))
        }

        let mut first_term  = Box::new(Assignment::CONST(string_equation[0].clone()));
        let mut second_term = Box::new(Assignment::CONST(string_equation[2].clone()));

        if let Some(variable_location) = stack_memory.find_variable(&string_equation[0]) {
            first_term = Box::new(Assignment::VAR(variable_location))
        }

        if let Some(variable_location) = stack_memory.find_variable(&string_equation[2]) {
            second_term = Box::new(Assignment::VAR(variable_location))
        }
        
        match string_equation[1].as_str() {
            "+" => Ok(*eq!(first_term, ADD, second_term)),
            "-" => Ok(*eq!(first_term, SUB, second_term)),
            "*" => Ok(*eq!(first_term, MUL, second_term)),
            "/" => Ok(*eq!(first_term, DIV, second_term)),
              _ => Err(TokensError::IncorrectEquationFormatting), 
        }
    }

    pub fn is_const(&self) -> bool {
        if let Self::CONST(_) = *self {
            return true
        } else {
            return false
        }
    }

    pub fn is_var(&self) -> bool {
        if let Self::VAR(_) = *self {
            return true
        } else {
            return false
        }
    }
}
