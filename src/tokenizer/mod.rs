use crate::type_traits::vector::StringVecExtra;
use crate::type_traits::hashmap::StringStringHashMapExtra;
use crate::data::SyntaxElements;


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
        let syntax_elements = SyntaxElements::init();

        let mut i: usize = 0;
        while i < optimized_file_content.len() {
            let current_word = optimized_file_content[i].clone();

            // Declarion handling
            if syntax_elements.type_names.values_contains(&current_word) {
                let declaration_stop_index = optimized_file_content[i..].to_vec()
                    .find("\n").unwrap();

                self.parse_declaration(optimized_file_content[i..declaration_stop_index].to_vec());
            }

            i += 1
        }
    }

    fn parse_declaration(&mut self, declaration: Vec<String>) { match DataType::check_token_type(&declaration[0]).unwrap() {
        DataType::INTEGER => {
            // Parse the declaration
            let equal_sign_index = declaration.find("=").unwrap();

            // Get the assignment part (everything after equals and before `;`)
            let string_assignment = declaration[equal_sign_index+1..declaration.len()].to_vec();

            // Retrieve the name of te variable, its data_type, and what it's assigned to
            let name = declaration[1].clone();
            let data_type = DataType::check_token_type(&declaration[0]).unwrap();
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
            let equal_sign_index = declaration.find("=").unwrap();
 
            // Get the assignment part (everything after equals and before `;`)
            let string_assignment = declaration[equal_sign_index+1..declaration.len()].to_vec();
 
            // Retrieve the name of te variable, its data_type, and what it's assigned to
            let name = declaration[1].clone();
            let data_type = DataType::check_token_type(&declaration[0]).unwrap();
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
