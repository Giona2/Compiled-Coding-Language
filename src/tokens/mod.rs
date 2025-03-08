use crate::type_traits::string_vec::StringVecExtra;


#[allow(dead_code)]
pub mod constructors;
    use constructors::*;

#[allow(dead_code)]
pub mod keyword_names;

#[allow(dead_code)]
#[allow(unused_macros)]
pub mod types;
    use types::*;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Token {
    FUNCTION(Function),
    TERMINATINGLOOP(TerminatingLoop),
    DECLARATION(Declaration),
}


/// Structure representing the stack memory of the program (with some constraints)
///   - All variables are of equal size
#[derive(Debug)]
#[allow(dead_code)]
pub struct Memory {
    stack_data: Vec<Option<Declaration>>,
    stack_step: usize,

} impl Memory {
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
        stack_data: vec![None],
        stack_step: step,
    }}

    #[allow(dead_code)]
    pub fn add_variable(&mut self, variable_name: &str, variable_data_type: DataType, variable_value: Option<Assignment>) {
        let mut new_variable_location: Option<usize> = None;

        for (variable_index, variable) in self.stack_data.iter().enumerate() {
            if let None = variable { new_variable_location = Some(variable_index) }
        }

        if let Some(unwrapped_new_variable_location) = new_variable_location {
            self.stack_data[unwrapped_new_variable_location] = Some(Declaration::new(
                variable_name,
                unwrapped_new_variable_location,
                variable_data_type,
                variable_value
            ))
        } else {
            self.stack_data.push(Some());
        }
    }
}


#[derive(Debug)]
pub struct SyntaxTree {
    token_tree: Vec<Token>,
    memory: Memory,

} impl SyntaxTree {
    pub fn optimize_file_content(file_content: &str) -> Vec<String> {
        // Replace all newline and tab characters eith spaces
        let script_content_flattened: Vec<char> = file_content
            .replace("\n", " ")
            .replace("\t", " ")
            .chars().collect();

        // Seperate all characters in the syntax list with spaces
        fn get_script_content_seperate_syntax_chars(script_content_flattened: Vec<char>) -> Vec<char> {
            let mut result = Vec::new();

            for (character_index, character) in script_content_flattened.iter().enumerate() {
                // If the character is folowed/proceeded by a space, place a space before and after
                // it
                if keyword_names::get_syntax_chars().contains(&character.to_string()) && script_content_flattened[character_index-1] != ' ' {
                    let updated_script_content = vec![script_content_flattened[..character_index].to_vec(), [' '].to_vec(), [character.clone()].to_vec(), [' '].to_vec(), script_content_flattened[character_index+1..].to_vec()].concat();
                    result = get_script_content_seperate_syntax_chars(updated_script_content);
                    break;
                }
                else if keyword_names::get_syntax_chars().contains(&character.to_string()) && script_content_flattened[character_index+1] != ' ' {
                    let updated_script_content = vec![script_content_flattened[..character_index].to_vec(), [' '].to_vec(), [character.clone()].to_vec(), [' '].to_vec(), script_content_flattened[character_index+1..].to_vec()].concat();
                    result = get_script_content_seperate_syntax_chars(updated_script_content);
                    break;
                }
                // If the loop gets to the end without spotting above, finally return it
                else if character_index == script_content_flattened.len()-1 {
                    result = script_content_flattened.clone();
                }
            }
            return result;
        }
        let script_content_seperate_syntax_chars = get_script_content_seperate_syntax_chars(script_content_flattened)
            .iter().collect::<String>();

        // Split by spaces
        let script_content_split = script_content_seperate_syntax_chars.split(" ");

        // Remove empty chars created by multiple spaces
        let script_content_trimmed: Vec<&str> = script_content_split.into_iter()
            .filter_map(|x| if x != "" {Some(x)} else {None})
            .collect();
        
        // Return the result
        return script_content_trimmed.iter()
            .map(|x| x.to_string())
            .collect();
    }

    pub fn from_file_content(optimized_file_content: Vec<String>) -> Self {
        let mut token_tree: Vec<Token> = Vec::new();
        let mut memory = Memory::init(8);

        for (current_word_index, current_word) in optimized_file_content.iter().enumerate() {
            // Variable handling
            if keyword_names::types::get_type_names().contains(current_word) {
                let full_declaration = optimized_file_content.index_to_pattern(current_word_index, ";")
                    .unwrap();
                let equal_sign_index = full_declaration.find("=").unwrap();

                let string_assignment = full_declaration[equal_sign_index+1..full_declaration.len()-1].to_vec();
                let assignment = Assignment::from_string_vec(string_assignment);

                let declaration = Declaration::new(
                    &optimized_file_content[current_word_index+1],
                    DataType::check_token_type(current_word).unwrap(),
                    Some(assignment.unwrap()),
                );

                token_tree.push(Token::DECLARATION(declaration));
            }
        };
    
        return Self {
            memory,
            token_tree,
        }
    }

}
