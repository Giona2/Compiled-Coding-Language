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
    #[error("Operator could not parse this comparison operator")]
    CouldNotParseComparisonOperator,
    #[error("Tokenizer could not find the end of this block.
             Ensure you properly ended this block or didn't forget any miscellanious end block characters")]
    CouldNotFindEndOfBlock,
    #[error("A non-condition assignment was used to construct this conditional statement.
             Ensure you used a conditional statement or a tof value")]
    IncorrectAssignmentForConditionalCondition,
    #[error("A variable was not passed through a cmp and was used in one of the branches.
             Ensure you have passed the variable between the :&[]")]
    VarNotUsedInComparison,
}
