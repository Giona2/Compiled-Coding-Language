use thiserror::Error;


#[derive(Error, Debug)]
pub enum TokenizerError {
    #[error("The stack_data variable in a Memory structure is incorrectly formatted.
             Ensure stack_data always ends with a None construct")]
    IncorrectStackDataFormatting,
    #[error("The equation inserted into Assignment::from_string_vec() was incorrectly formatted.
             Ensure you are using proper built-in arithmetic characters in the equation")]
    IncorrectEquationFormatting,
    #[error("Assignment could not parse this term in a variable declaration")]
    CouldNotParseTerm,
    #[error("Operator could not parse this math operator")]
    CouldNotParseMathOperator,
}
