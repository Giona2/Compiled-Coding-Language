use thiserror::Error;


#[derive(Error, Debug)]
pub enum AssemblerError {
    #[error("This token is not yet supported")]
    TokenNotSupported,
    #[error("The value attempting to the retrieved through Assignment.term_to_assembly_value().
             Ensure this Assignment is either a CONST() or a TERM()")]
    ValueRetrievedIsNotATerm,
}
