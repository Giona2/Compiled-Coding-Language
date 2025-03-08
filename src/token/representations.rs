use super::constructors::Declaration;
use super::types::{DataType, Assignment};
use super::error::TokensError;


pub struct StackVariable {
    data_type: DataType,
    value: Assignment,
}


/// Structure representing the stack memory of the program (with some constraints)
///   - All variables are of equal size
#[derive(Debug)]
#[allow(dead_code)]
pub struct StackMemory {
    data: Vec<Option<Declaration>>,
    step: usize,

} impl StackMemory {
    /// Initiates stack memory representation (`Memory`) for the program
    /// `step` indicates how many bytes each variable can occupy
    ///
    /// # Example
    ///
    /// ```rust
    /// // Represents stack memory where each variable is 8 bytes
    /// let memory = Memory::init(step: 8)
    /// ```
    #[allow(dead_code)]
    pub fn init(step: usize) -> Self { return Self {
        data: vec![None],
        step,
    }}

    #[allow(dead_code)]
    pub fn add_variable(&mut self, variable_name: &str, variable_data_type: DataType, variable_value: Option<Assignment>) -> Result<(), TokensError> {
        let mut new_variable_location: Option<usize> = None;

        for (variable_index, variable) in self.data.iter().enumerate() {
            if let None = variable { new_variable_location = Some(variable_index) }
        }

        if let Some(unwrapped_new_variable_location) = new_variable_location {
            self.data[unwrapped_new_variable_location] = Some(Declaration::new(
                variable_name,
                unwrapped_new_variable_location,
                variable_data_type,
                variable_value
            ));
            return Ok(())
        } else {
            return Err(TokensError::IncorrectStackDataFormatting)
        }
    }
}
