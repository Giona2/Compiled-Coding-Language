use thiserror::Error;


#[derive(Error, Debug)]
pub enum TokensError {
    #[error("The stack_data variable in a Memory structure is incorrectly formatted.
             Ensure stack_data always ends with a None construct")]
    IncorrectStackDataFormatting
}
