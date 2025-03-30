use super::error::TokenizerError;
use super::Tokenizer;
use crate::data::SyntaxElements;
use crate::type_traits::hashmap::HashMapExtra;
use super::structures::VariableHistory;
use crate::type_traits::vector::VecExtra;
use super::declaration::DataType;


#[derive(Debug, Clone, PartialEq)]
pub enum MathOperator {
    ADD,
    SUB,
    MUL,
    DIV,
} impl MathOperator {
    /// Converts a string of the accociated operator with a variant of `Operator`
    ///
    /// Returns Err() if an incorrect symbol was given
    ///
    /// This is used in conjunction with `SyntaxElements` to correctly match all math operators
    pub fn from_string(from: &str) -> Result<Self, TokenizerError> {
        let syntax_elements = SyntaxElements::init();
        match from {
            val if val == syntax_elements.math_symbols["addition"] => {
                return Ok(Self::ADD)
            }
            val if val == syntax_elements.math_symbols["subtraction"] => {
                return Ok(Self::SUB)
            }
            val if val == syntax_elements.math_symbols["multiplication"] => {
                return Ok(Self::MUL)
            }
            val if val == syntax_elements.math_symbols["division"] => {
                return Ok(Self::DIV)
            }
            _ => {
                return Err(TokenizerError::CouldNotParseMathOperator)
            }
        }
    }
}


#[derive(Debug, Clone)]
pub enum ComparisonOperator {
    /// Equal To
    EQ,
    /// Not Equal To
    NEQ,
    /// Greater Than
    GT,
    /// Less Than
    LT,
    /// Greater Than or Equal to
    GEQ,
    /// Less Than or Equal to
    LEQ,
} impl ComparisonOperator {
    pub fn from_string(from: &str) -> Result<Self, TokenizerError> {
        let syntax_elements = SyntaxElements::init();
        match from {
            val if val == syntax_elements.comparision_symbols["equal to"] => {
                return Ok(Self::EQ)
            }
            val if val == syntax_elements.comparision_symbols["not equal to"] => {
                return Ok(Self::NEQ)
            }
            val if val == syntax_elements.comparision_symbols["greater than"] => {
                return Ok(Self::GT)
            }
            val if val == syntax_elements.comparision_symbols["greater than or equal to"] => {
                return Ok(Self::GEQ)
            }
            val if val == syntax_elements.comparision_symbols["less than"] => {
                return Ok(Self::LT)
            }
            val if val == syntax_elements.comparision_symbols["less than or equal to"] => {
                return Ok(Self::LEQ)
            }
            _ => {
                return Err(TokenizerError::CouldNotParseComparisonOperator)
            }
        }
    }
}


/*
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
*/


#[derive(Debug, Clone)]
pub enum Assignment {
    EVAL(Box<Assignment>, MathOperator, Box<Assignment>),
    CMP(Box<Assignment>, ComparisonOperator, Box<Assignment>),
    FUNC(String, DataType, Vec<Assignment>),
    BOOL(i64),
    VAR(usize),
    INTEGER(i64),
    FLOAT(f64),
} impl Assignment {
    /// Parses a string vector (an equation) into an Assignment
    pub fn from_string_vec(tokenizer: &Tokenizer, variable_history: &VariableHistory, string_equation: Vec<String>) -> Self {
        let syntax_elements = SyntaxElements::init();

        let operator_symbols = syntax_elements.get_all_operator_symbols();

        // If the equation given is just one element
        if string_equation.len() == 1 || string_equation.find_from_vec(&operator_symbols).is_none() {
            return Self::from_equation_term(tokenizer, variable_history, string_equation).unwrap()
        }

        // Get the location of the math symbol
        let operator_symbol_index = string_equation.find_from_vec(&operator_symbols).unwrap();
        let operator_symbol = string_equation[operator_symbol_index].clone();

        // Get the first and second terms
        let first_term_slice  = string_equation[..=operator_symbol_index-1].to_owned();
        let second_term_slice = string_equation[operator_symbol_index+1..].to_owned();

        // Parse the first and second terms
        let first_term  = Self::from_equation_term(tokenizer, variable_history, first_term_slice ).unwrap();
        let second_term = Self::from_equation_term(tokenizer, variable_history, second_term_slice).unwrap();

        // Return the evaluation of the first term and the second term operating with the given
        // math symbol
        if syntax_elements.math_symbols.contains_value(&operator_symbol) {
            return Self::EVAL(Box::new(first_term), MathOperator::from_string(&operator_symbol).unwrap(), Box::new(second_term));
        }
        else if syntax_elements.comparision_symbols.contains_value(&operator_symbol) {
            return Self::CMP(Box::new(first_term), ComparisonOperator::from_string(&operator_symbol).unwrap(), Box::new(second_term))
        }
        else {
            panic!("Math operator not found");
        }

        /*match string_equation[1].as_str() {
            "+" => Ok(*feq!(first_term, ADD, second_term)),
            "-" => Ok(*feq!(first_term, SUB, second_term)),
            "*" => Ok(*feq!(first_term, MUL, second_term)),
            "/" => Ok(*feq!(first_term, DIV, second_term)),
              _ => Err(TokenizerError::IncorrectEquationFormatting), 
        }*/
    }

    /// Returns the data type this Assignment will become after evaluation
    pub fn evaluate_type(&self, variable_history: &VariableHistory) -> DataType { match self {
        Self::INTEGER(_)   => { return DataType::INTEGER }
        Self::FLOAT(_)     => { return DataType::FLOAT   }
        Self::BOOL(_)      => { return DataType::BOOL    }
        Self::CMP(_, _, _) => { return DataType::BOOL    }

        Self::VAR(variable_location) => {
            let variable = variable_history.data[*variable_location].clone().unwrap();

            return variable.data_type
        }

        Self::FUNC(_, data_type, _) => {
            return data_type.clone()
        }

        Self::EVAL(first_term, _, second_term) => {
            let first_term_type  = first_term.evaluate_type(variable_history);
            let second_term_type = second_term.evaluate_type(variable_history);

            if first_term_type.is_float() || second_term_type.is_float() {
                return DataType::FLOAT
            } else {
                return DataType::INTEGER
            }
        }
    }}

    /// Parses a singular term in an equation (functions, numbers) into Self
    fn from_equation_term(tokenizer: &Tokenizer, variable_history: &VariableHistory, term: Vec<String>) -> Result<Self, TokenizerError> {
        println!("coding_language::tokenizer::enumerators::Assignment::from_equation_term()");
        println!("  recieved: {term:?}");
        println!("  tokenizer functions = {:?}", tokenizer.function_history);

        let syntax_elements = SyntaxElements::init();

        let begin_args_char = syntax_elements.assignment_symbols.get("begin conditions").unwrap();
        let end_args_char   = syntax_elements.assignment_symbols.get("end conditions").unwrap();

        // Check if the declaration is an integer
        if let Ok(returned_number) = term[0].clone().parse::<i64>() {
            return Ok(Assignment::INTEGER(returned_number))
        }
        // Check if the declaration is a float
        else if let Ok(returned_number) = term[0].clone().parse::<f64>() {
            return Ok(Assignment::FLOAT(returned_number))
        }
        // Chech if the declaration is a boolean value
        else if syntax_elements.comparision_names.contains_value(&term[0]) {
            if term[0] == syntax_elements.comparision_names["true"] {
                return Ok(Assignment::BOOL(1))
            } else if term[0] == syntax_elements.comparision_names["false"] {
                return Ok(Assignment::BOOL(0))
            } else {
                return Err(TokenizerError::CouldNotParseTerm)
            }
        }
        // Check if the declaration is a variable
        else if let Some(variable_location_index) = variable_history.find_variable(&term[0]) {
            return Ok(Assignment::VAR(variable_location_index))
        }
        // Check if the declaration is a function
        else if let Some(function) = tokenizer.function_history.find_by_name(&term[0]) {
            // Find the begin and end args characters
            let begin_args_index = term.find(begin_args_char).unwrap();
            let end_args_index   = term.find(end_args_char).unwrap();

            // Get the argument slice
            let passed_args_slice: Vec<String> = term[begin_args_index+1..=end_args_index-1].to_vec();
            
            // Get the name of the function and make a list of all the arguments
            let mut passed_args: Vec<Assignment> = Vec::new();
            if passed_args_slice.len() > 0 { for passed_argument_string in passed_args_slice.split(|x| x==",").into_iter() {
                passed_args.push(Self::from_equation_term(tokenizer, variable_history, passed_argument_string.to_vec()).unwrap())
            }};

            return Ok(Assignment::FUNC(function.name, function.return_type, passed_args));
        }
        else {
            Err(TokenizerError::CouldNotParseTerm)
        }
    }
}


/*
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
    FUNCTION(String, Vec<Assignment>),

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
    /*pub fn from_string_vec(stack_memory: &VariableHistory, string_equation: Vec<String>) -> Result<Self, TokenizerError> {
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
    }*/

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
    FUNCTION(String, Vec<Assignment>),

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
}*/
