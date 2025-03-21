use super::types::DataType;
use super::error::TokenizerError;


#[derive(Debug)]
pub struct StackVariable {
    name: String,
    data_type: DataType,
}


/// Structure representing the stack memory of the program (with some constraints)
///   - All variables are of equal size
#[derive(Debug)]
pub struct StackMemory {
    pub data: Vec<Option<StackVariable>>,
    pub step: usize,

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
    pub fn init(step: usize) -> Self { return Self {
        data: vec![None],
        step,
    }}

    pub fn add_variable(&mut self, variable_name: &str, variable_data_type: DataType) -> Result<(), TokenizerError> {
        // Initiate this variable to manage error handling
        let mut new_variable_location: Option<usize> = None;

        // Find the first instance of a None value
        for (variable_index, variable) in self.data.iter().enumerate() {
            if let None = variable { new_variable_location = Some(variable_index) }
        }

        // Replace that None value with the new variable...
        if let Some(unwrapped_new_variable_location) = new_variable_location {
            self.data[unwrapped_new_variable_location] = Some(StackVariable {
                name: variable_name.to_string(),
                data_type: variable_data_type,
            });
            if unwrapped_new_variable_location == self.data.len() - 1 {
                self.data.push(None);
            }

            return Ok(())
        // and fail if no None character was found (hence initiating the target index as it as None)
        } else {
            return Err(TokenizerError::IncorrectStackDataFormatting)
        }
    }
    
    /// Finds a variable (by name) and returns the slot it's held in
    ///
    /// Returns None if the variable is not found
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Initialte stack memory representation
    /// let mut stack_memory = StackMemory::init(8);
    ///
    /// // Add variable `x` to stack memory
    /// stack_memory.add_variable("x", DataType::INTEGER);
    ///
    /// // Passes
    /// let location_of_x = stack_memory.find_variable("x");
    /// assert_ne!(None::<usize>, location_of_x) 
    ///
    /// // Panics. The y variable does not exist
    /// let location_of_y = stack_memory.find_variable("y");
    /// assert_ne!(None::<usize>, location_of_x) 
    /// ```
    pub fn find_variable(&self, variable_name: &str) -> Option<usize> {
        let mut result: Option<usize> = None;

        for (i, variable) in self.data.iter().enumerate() { if let Some(variable) = variable {
            if variable_name == variable.name { result = Some(i) }
        }} 

        return result
    }
}
