use crate::type_traits::string_vec::StringVecExtra;
use crate::data::syntactic_elements;


#[allow(dead_code)]
pub mod representations;
    use representations::StackMemory;

#[allow(dead_code)]
pub mod constructors;
    use constructors::*;

#[allow(dead_code)]
#[allow(unused_macros)]
pub mod types;
    use types::*;

#[allow(dead_code)]
pub mod error;


#[derive(Debug)]
#[allow(dead_code)]
pub enum Token {
    FUNCTION(Function),
    TERMINATINGLOOP(TerminatingLoop),
    DECLARATION(Declaration),
}


#[derive(Debug)]
pub struct Tokenizer {
    pub token_tree: Vec<Token>,
    pub stack_memory: StackMemory,

} impl Tokenizer {
    pub fn init(stack_memory_step: usize) -> Self { Self {
        token_tree: Vec::new(),
        stack_memory: StackMemory::init(stack_memory_step),
    }}

    pub fn generate_token_tree(&mut self, optimized_file_content: &Vec<String>) {
        for (current_word_index, current_word) in optimized_file_content.iter().enumerate() {
            // Variable handling
            if syntactic_elements::types::get_type_names().contains(current_word) {
                self.parse_declaration(current_word, current_word_index, optimized_file_content);
            }
        };
    }

    fn parse_declaration(&mut self, current_word: &String, current_word_index: usize, optimized_file_content: &Vec<String>) { match DataType::check_token_type(&current_word).unwrap() {
        DataType::INTEGER => {
            // Parse the declaration
            let full_declaration = optimized_file_content.index_to_pattern(current_word_index, ";").unwrap();
            let equal_sign_index = full_declaration.find("=").unwrap();

            // Get the assignment part (everything after equals and before `;`)
            let string_assignment = full_declaration[equal_sign_index+1..full_declaration.len()-1].to_vec();

            // Retrieve the name of te variable, its data_type, and what it's assigned to
            let name = optimized_file_content[current_word_index+1].clone();
            let data_type = DataType::check_token_type(current_word).unwrap();
            let assignment = IntegerAssignment::from_string_vec(&self.stack_memory, string_assignment).unwrap();

            // Add it to representation stack_memory
            self.stack_memory.add_variable(&name, data_type.clone())
                .expect("stack_memory does not conclude with None");

            // Build the declaration token
            let declaration = Declaration {
                name: name.to_string(),
                location: self.stack_memory.find_variable(&name).unwrap(),
                data_type,
                value: Some(Assignment::INTEGER(assignment)),
            };

            // Add the token to the token_tree
            self.token_tree.push(Token::DECLARATION(declaration));
        }

        DataType::FLOAT => {
            println!("Found Float");
            // Parse the declaration
            let full_declaration = optimized_file_content.index_to_pattern(current_word_index, ";").unwrap();
            let equal_sign_index = full_declaration.find("=").unwrap();
 
            // Get the assignment part (everything after equals and before `;`)
            let string_assignment = full_declaration[equal_sign_index+1..full_declaration.len()-1].to_vec();
 
            // Retrieve the name of te variable, its data_type, and what it's assigned to
            let name = optimized_file_content[current_word_index+1].clone();
            let data_type = DataType::check_token_type(current_word).unwrap();
            let assignment = FloatAssignment::from_string_vec(&self.stack_memory, string_assignment).unwrap();
 
            // Add it to representation stack_memory
            self.stack_memory.add_variable(&name, data_type.clone())
                .expect("stack_memory does not conclude with None");
 
            // Build the declaration token
            let declaration = Declaration {
                name: name.to_string(),
                location: self.stack_memory.find_variable(&name).unwrap(),
                data_type,
                value: Some(Assignment::FLOAT(assignment)),
            };
 
            // Add the token to the token_tree
            self.token_tree.push(Token::DECLARATION(declaration));
        }
    }}
}
