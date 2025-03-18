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


pub struct Tokenizer {
    pub token_tree: Vec<Token>,
    pub stack_memory: StackMemory,

    syntax_elements: SyntaxElements,

} impl Tokenizer {
    pub fn init(stack_memory_step: usize) -> Self { Self {
        token_tree: Vec::new(),
        stack_memory: StackMemory::init(stack_memory_step),

        syntax_elements: SyntaxElements::init(),
    }}

    pub fn create_token_tree(&mut self, optimized_file_content: &Vec<String>) {
        let mut stack_memory = StackMemory::init(self.stack_memory.step);
        let token_tree = self.generate_token_tree(&mut stack_memory, optimized_file_content);

        self.stack_memory = stack_memory;
        self.token_tree = token_tree;
    }

    pub fn generate_token_tree(&self, stack_memory: &mut StackMemory, content_to_tokenize: &Vec<String>) -> Vec<Token> {
        let mut result: Vec<Token> = Vec::new();

        let mut i: usize = 0;
        while i < content_to_tokenize.len() {
            let current_word = content_to_tokenize[i].clone();

            // Declarion handling
            if self.syntax_elements.type_names.values_contains(&current_word) { match &content_to_tokenize[i] {
                val if val == self.syntax_elements.type_names.get("integer").unwrap() => {
                    // Get the first instance of the end assignment character after the
                    // declaration (therefore ending it)
                    let declaration_stop_char = self.syntax_elements.assignment_symbols.get("end assignment").unwrap();
                    let declaration_stop_index = content_to_tokenize.find_after_index(i, declaration_stop_char).unwrap();

                    // Get the slice from this index (the declaration start) to the end
                    // assignment char (the declaration end)
                    let declaration_to_evaluate = content_to_tokenize[i..declaration_stop_index].to_vec();

                    // Parse the slice into a token and add it to the result
                    let created_token = self.parse_integer(stack_memory, declaration_to_evaluate);
                    result.push(created_token);

                    // Move the current word to one word after the end of this declaration and
                    // continue the loop
                    i = declaration_stop_index;
                    continue;
                }

                val if val == self.syntax_elements.type_names.get("float").unwrap() => {
                    // Get the first instance of the end assignment character after the
                    // declaration (therefore ending it)
                    let declaration_stop_char = self.syntax_elements.assignment_symbols.get("end assignment").unwrap();
                    let declaration_stop_index = content_to_tokenize.find_after_index(i, declaration_stop_char).unwrap();

                    // Get the slice from this index (the declaration start) to the end
                    // assignment char (the declaration end)
                    let declaration_to_evaluate = content_to_tokenize[i..declaration_stop_index].to_vec();

                    // Parse the slice into a token and add it to the result
                    let created_token = self.parse_float(stack_memory, declaration_to_evaluate);
                    result.push(created_token);

                    // Move the current word to one word after the end of this declaration and
                    // continue the loop
                    i = declaration_stop_index;
                    continue;
                }

                val if val == self.syntax_elements.type_names.get("function").unwrap() => {
                    // Get the first instance of the end block character after the
                    // declaration (therefore ending it)
                    let block_stop_char = self.syntax_elements.assignment_symbols.get("end body").unwrap();
                    let declaration_stop_index = content_to_tokenize.find_after_index(i, &block_stop_char).unwrap();

                    // Get the slice from this index (the declaration start) to the
                    // block char (the declaration end)
                    let declaration_to_evaluate = content_to_tokenize[i..=declaration_stop_index].to_vec();

                    // Parse the slice into a token and add it to the result
                    let created_token = self.parse_function(stack_memory, declaration_to_evaluate);
                    result.push(created_token);

                    // Move the current word to one word after the end of this declaration and
                    // continue the loop
                    i = declaration_stop_index;
                    continue;
                }

                _ => {}
            }}

            i += 1
        }

        return result
    }

    fn parse_integer(&self, stack_memory: &mut StackMemory, declaration: Vec<String>) -> Token {
        // Parse the declaration
        let equal_sign_index = declaration.find("=").unwrap();

        // Get the assignment part (everything after equals and before `;`)
        let string_assignment = declaration[equal_sign_index+1..declaration.len()].to_vec();

        // Retrieve the name of te variable, its data_type, and what it's assigned to
        let name = declaration[1].clone();
        let data_type = DataType::check_token_type(&declaration[0]).unwrap();
        let assignment = IntegerAssignment::from_string_vec(&self.stack_memory, string_assignment).unwrap();

        // Add it to representation stack_memory
        stack_memory.add_variable(&name, data_type.clone())
            .expect("stack_memory does not conclude with None");

        // Build the declaration token
        let declaration = Declaration {
            name: name.to_string(),
            location: stack_memory.find_variable(&name).unwrap(),
            data_type,
            value: Some(Assignment::INTEGER(assignment)),
        };

        return Token::DECLARATION(declaration)
    }

    fn parse_float(&self, stack_memory: &mut StackMemory, declaration: Vec<String>) -> Token {
        // Parse the declaration
        let equal_sign_index = declaration.find("=").unwrap();
 
        // Get the assignment part (everything after equals and before `;`)
        let string_assignment = declaration[equal_sign_index+1..declaration.len()].to_vec();
 
        // Retrieve the name of te variable, its data_type, and what it's assigned to
        let name = declaration[1].clone();
        let data_type = DataType::check_token_type(&declaration[0]).unwrap();
        let assignment = FloatAssignment::from_string_vec(&self.stack_memory, string_assignment).unwrap();
 
        // Add it to representation stack_memory
        stack_memory.add_variable(&name, data_type.clone())
            .expect("stack_memory does not conclude with None");
 
        // Build the declaration token
        let declaration = Declaration {
            name: name.to_string(),
            location: stack_memory.find_variable(&name).unwrap(),
            data_type,
            value: Some(Assignment::FLOAT(assignment)),
        };
 
        return Token::DECLARATION(declaration)
    }

    fn parse_function(&self, stack_memory: &mut StackMemory, declaration: Vec<String>) -> Token {
        let block_start_char = self.syntax_elements.assignment_symbols.get("begin body")
            .unwrap();
        let return_this_char = self.syntax_elements.assignment_symbols.get("return this")
            .unwrap();

        let block_start_index = declaration.find(&block_start_char).unwrap();
        let return_this_index = declaration.find(&return_this_char).unwrap();

        let inline_block = declaration[block_start_index+1..].to_vec();

        let name = declaration[1].to_string();
        let return_type_text = declaration[return_this_index+1].to_owned();
        let return_type = DataType::check_token_type(&return_type_text).unwrap();
        let inline_block_tokenized = self.generate_token_tree(stack_memory, &inline_block);

        let function = Function {
            name,
            return_type,
            args: Vec::new(),
            functionaliy: inline_block_tokenized,
        };

        return Token::FUNCTION(function)
    }
}
