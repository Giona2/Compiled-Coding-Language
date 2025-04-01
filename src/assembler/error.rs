use thiserror::Error;


#[derive(Error, Debug)]
pub enum AssemblerError {
    #[error("This token is not yet supported")]
    TokenNotSupported,
    #[error("The value attempting to the retrieved through Assignment.term_to_assembly_value().
             Ensure this Assignment is either a CONST() or a TERM()")]
    ValueRetrievedIsNotATerm,
    #[error("The assignment you are trying to convert to an assembly instruction is not a valid branch.
             Ensure you are using either an Assignment::INTEGER or Assignment::FLOAT branch")]
    IncorrectAssignmentAttemptedToConvert,
    #[error("This evaluation could not be parsed.
             Ensure you are using a numerical value in the evaluation")]
    CouldNotParseEvaluation,
    #[error("The Assembler's types_translator was misused\n
             Ensure you are only using this with complex Assignments")]
    ImproperUseOfTypesTranslator,
    #[error("An assignment was incorrectly formatted\n
             This is most commonly caused by mixing value types in equations")]
    IncorrectAssignmentFormatting,
}
