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


/// Short-hand way to build an Assignment
/// The suffix indicates the types you need to use for this specific variant
///     c: constant (a string litteral)
///     e: equation (an Assignment build from equation_!())
/// 
/// # Examples
///
/// ```rust
/// let equation: Assignment = equation_cc!("1", ADD, "2");
///
/// // output: ADD((TERM("1"), TERM("2")))
/// println!("{equation:?}");
/// ```
macro_rules! equation_cc {
    ($first_term:expr, $operation:ident, $second_term:expr) => {
        Box::new(Assignment::$operation(
            Box::new(Assignment::TERM($first_term.to_string())),
            Box::new(Assignment::TERM($second_term.to_string())),
        ))
    };
}
/// Short-hand way to build an Assignment
/// The suffix indicates the types you need to use for this specific variant
///     c: constant (a string litteral)
///     e: equation (an Assignment build from equation_!())
/// 
/// # Examples
///
/// ```rust
/// let equation: Assignment = equation_ec!(equation_cc!("1", ADD, "2"), ADD, "3");
///
/// // output: ADD((ADD((TERM("1"), TERM("2"))), TERM("3")))
/// println!("{equation:?}");
/// ```
macro_rules! equation_ec {
    ($first_term:expr, $operation:ident, $second_term:expr) => {
        Box::new(Assignment::$operation(
            $first_term,
            Box::new(Assignment::TERM($second_term.to_string())),
        ))
    };
}
/// Short-hand way to build an Assignment
/// The suffix indicates the types you need to use for this specific variant
///     c: constant (a string litteral)
///     e: equation (an Assignment build from equation_!())
/// 
/// # Examples
///
/// ```rust
/// let equation: Assignment = equation_ce!("1", ADD, equation_cc("2", ADD, "3"));
///
/// // output: ADD((TERM("1"), ADD((TERM("2"), TERM("3")))))
/// println!("{equation:?}");
/// ```
macro_rules! equation_ce {
    ($first_term:expr, $operation:ident, $second_term:expr) => {
        Box::new(Assignment::$operation(
            Box::new(Assignment::TERM($first_term.to_string())),
            $second_term,
        ))
    };
}
/// Short-hand way to build an Assignment
/// The suffix indicates the types you need to use for this specific variant
///     c: constant (a string litteral)
///     e: equation (an Assignment build from equation_!())
/// 
/// # Examples
///
/// ```rust
/// let equation: Assignment = equation_ee!(equation_cc!("1", ADD "2"), ADD, equation_cc("3", ADD, "4"));
///
/// // output: ADD((ADD((TERM("1"), TERM("2"))), ADD((TERM("3"), TERM("4")))))
/// println!("{equation:?}");
/// ```
macro_rules! equation_ee {
    ($first_term:expr, $operation:ident, $second_term:expr) => {
        Box::new(Assignment::$operation(
            $first_term,
            $second_term,
        ))
    };
}

#[derive(Debug)]
pub enum Assignment {
    ADD(Box<Assignment>, Box<Assignment>),
    SUB(Box<Assignment>, Box<Assignment>),
    MUL(Box<Assignment>, Box<Assignment>),
    DIV(Box<Assignment>, Box<Assignment>),
    TERM(String),

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
    pub fn from_string_vec(string_equation: Vec<String>) -> Option<Self> {
        if string_equation.len() == 1 {
            return Some(Self::TERM(string_equation[0].clone()))
        }

        println!("Assignment::from_string_vec");
        println!("  recieved {:?}", string_equation);
        match string_equation[1].as_str() {
            "+" => Some(*equation_cc!(string_equation[0], ADD, string_equation[2])),
            "-" => Some(*equation_cc!(string_equation[0], SUB, string_equation[2])),
            "*" => Some(*equation_cc!(string_equation[0], MUL, string_equation[2])),
            "/" => Some(*equation_cc!(string_equation[0], DIV, string_equation[2])),
              _ => None
        }
    }
}
