use super::error::TokenizerError;
use crate::data::SyntaxElements;
use super::structures::VariableHistory;


#[derive(Debug, Clone)]
pub enum DataType {
    INTEGER,
    FLOAT,
} impl DataType {
    pub fn check_token_type(word_to_check: &str) -> Option<Self> { let syntax_elements = SyntaxElements::init(); match word_to_check {
        val if val == syntax_elements.type_names["integer"] => Some(Self::INTEGER),
        val if val == syntax_elements.type_names["float"]   => Some(Self::FLOAT),
                                                          _ => None,
    }}
}


#[derive(Debug, Clone)]
pub enum Assignment {
    INTEGER(IntegerAssignment),
    FLOAT(FloatAssignment),
}


/// A shorthand method to build an Assignment enumerator
///
/// ```rust
/// ieq!(t"1", ADD, ieq(t"2", SUB, t'3'))
/// ```
#[macro_export]
macro_rules! ieq {
    ($first_term_branch:ident($first_term:expr), $operation:ident, $second_term_branch:ident($second_term:expr)) => {
        Box::new(IntegerAssignment::$operation(
            Box::new(IntegerAssignment::$first_term_branch($first_term.to_string())),
            Box::new(IntegerAssignment::$second_term_branch($second_term.to_string())),
        ))
    };
    ($first_equation:expr, $operation:ident, $second_term_branch:ident($second_term:expr)) => {
        Box::new(IntegerAssignment::$operation(
            $first_equation,
            Box::new(IntegerAssignment::$second_term_branch($second_term.to_string())),
        ))
    };
    ($first_term_branch:ident($first_term:expr), $operation:ident, $second_term:expr) => {
        Box::new(IntegerAssignment::$operation(
            Box::new(IntegerAssignment::$first_term_branch($first_term.to_string())),
            $second_term,
        ))
    };
    ($first_term:expr, $operation:ident, $second_term:expr) => {
        Box::new(IntegerAssignment::$operation(
            $first_term,
            $second_term,
        ))
    };
}

macro_rules! feq {
    ($first_term_branch:ident($first_term:expr), $operation:ident, $second_term_branch:ident($second_term:expr)) => {
        Box::new(FloatAssignment::$operation(
            Box::new(FloatAssignment::$first_term_branch($first_term.to_string())),
            Box::new(Assignment::$second_term_branch($second_term.to_string())),
        ))
    };
    ($first_equation:expr, $operation:ident, $second_term_branch:ident($second_term:expr)) => {
        Box::new(FloatAssignment::$operation(
            $first_equation,
            Box::new(FloatAssignment::$second_term_branch($second_term.to_string())),
        ))
    };
    ($first_term_branch:ident($first_term:expr), $operation:ident, $second_term:expr) => {
        Box::new(FloatAssignment::$operation(
            Box::new(FloatAssignment::$first_term_branch($first_term.to_string())),
            $second_term,
        ))
    };
    ($first_term:expr, $operation:ident, $second_term:expr) => {
        Box::new(FloatAssignment::$operation(
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
#[derive(Debug, Clone)]
pub enum IntegerAssignment {
    ADD(Box<IntegerAssignment>, Box<IntegerAssignment>),
    SUB(Box<IntegerAssignment>, Box<IntegerAssignment>),
    MUL(Box<IntegerAssignment>, Box<IntegerAssignment>),
    CONST(i64),
    VAR(usize),

} impl IntegerAssignment {
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
    pub fn from_string_vec(stack_memory: &VariableHistory, string_equation: Vec<String>) -> Result<Self, TokenizerError> {
        println!("IntegerAssignment::from_string_vec()");
        println!("  |- recieved: {string_equation:?}");

        if string_equation.len() == 1 {
            return Ok(Self::CONST(string_equation[0].clone().parse().unwrap()))
        }

        let first_term: Box<IntegerAssignment>;
        let second_term: Box<IntegerAssignment>;

        if let Some(variable_location) = stack_memory.find_variable(&string_equation[0]) {
            first_term = Box::new(IntegerAssignment::VAR(variable_location));
        } else {
            first_term = Box::new(IntegerAssignment::CONST(string_equation[0].clone().parse().unwrap()));
        }

        if let Some(variable_location) = stack_memory.find_variable(&string_equation[2]) {
            second_term = Box::new(IntegerAssignment::VAR(variable_location));
        } else {
            second_term = Box::new(IntegerAssignment::CONST(string_equation[2].clone().parse().unwrap()));
        }
        
        match string_equation[1].as_str() {
            "+" => Ok(*ieq!(first_term, ADD, second_term)),
            "-" => Ok(*ieq!(first_term, SUB, second_term)),
            "*" => Ok(*ieq!(first_term, MUL, second_term)),
              _ => Err(TokenizerError::IncorrectEquationFormatting), 
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

#[derive(Debug, Clone)]
pub enum FloatAssignment {
    ADD(Box<FloatAssignment>, Box<FloatAssignment>),
    SUB(Box<FloatAssignment>, Box<FloatAssignment>),
    MUL(Box<FloatAssignment>, Box<FloatAssignment>),
    DIV(Box<FloatAssignment>, Box<FloatAssignment>),
    CONST(f64),
    VAR(usize),

} impl FloatAssignment {
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
    pub fn from_string_vec(stack_memory: &VariableHistory, string_equation: Vec<String>) -> Result<Self, TokenizerError> {
        println!("IntegerAssignment::from_string_vec()");
        println!("  |- recieved: {string_equation:?}");

        if string_equation.len() == 1 {
            return Ok(Self::CONST(string_equation[0].clone().parse().unwrap()))
        }

        let first_term: Box<FloatAssignment>;
        let second_term: Box<FloatAssignment>;

        if let Some(variable_location) = stack_memory.find_variable(&string_equation[0]) {
            first_term = Box::new(FloatAssignment::VAR(variable_location));
        } else {
            first_term = Box::new(FloatAssignment::CONST(string_equation[0].clone().parse().unwrap()));
        }

        if let Some(variable_location) = stack_memory.find_variable(&string_equation[2]) {
            second_term = Box::new(FloatAssignment::VAR(variable_location));
        } else {
            second_term = Box::new(FloatAssignment::CONST(string_equation[2].clone().parse().unwrap()));
        }
        
        match string_equation[1].as_str() {
            "+" => Ok(*feq!(first_term, ADD, second_term)),
            "-" => Ok(*feq!(first_term, SUB, second_term)),
            "*" => Ok(*feq!(first_term, MUL, second_term)),
            "/" => Ok(*feq!(first_term, DIV, second_term)),
              _ => Err(TokenizerError::IncorrectEquationFormatting), 
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
