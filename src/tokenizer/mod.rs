use crate::type_traits::string_vec::StringVecExtra;
use crate::optimizer::Optimizer;
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
pub struct TokenTree {
    token_tree: Vec<Token>,
    stack_memory: StackMemory,

} impl TokenTree {
    pub fn from_file_content(optimizer: &Optimizer) -> Self {
        let mut token_tree: Vec<Token> = Vec::new();
        let mut stack_memory = StackMemory::init(8);

        for (current_word_index, current_word) in optimizer.content.iter().enumerate() {
            // Variable handling
            if syntactic_elements::types::get_type_names().contains(current_word) {
                // Parse the declaration
                let full_declaration = optimizer.content.index_to_pattern(current_word_index, ";").unwrap();
                let equal_sign_index = full_declaration.find("=").unwrap();

                // Get the assignment part (everything after equals and before `;`)
                let string_assignment = full_declaration[equal_sign_index+1..full_declaration.len()-1].to_vec();

                // Retrieve the name of te variable, its data_type, and what it's assigned to
                let name = optimizer.content[current_word_index+1].clone();
                let data_type = DataType::check_token_type(current_word).unwrap();
                let assignment = Assignment::from_string_vec(&stack_memory, string_assignment).unwrap();

                // Add it to representation stack_memory
                stack_memory.add_variable(&name, data_type.clone())
                    .expect("stack_memory does not conclude with None");

                // Build the declaration token
                let declaration = Declaration {
                    name: name.to_string(),
                    location: stack_memory.find_variable(&name).unwrap(),
                    data_type,
                    value: Some(assignment),
                };

                // Add the token to the token_tree
                token_tree.push(Token::DECLARATION(declaration));
            }
        };
    
        return Self {
            stack_memory,
            token_tree,
        }
    }
}
