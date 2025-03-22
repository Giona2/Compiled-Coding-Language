use crate::type_traits::vector::VecExtra;
use crate::type_traits::hashmap::StringStringHashMapExtra;
use crate::data::{SyntaxElements, MEMORY_STEP};


#[allow(dead_code)]
pub mod structures;
    use structures::{FunctionHistory, Variable, VariableHistory};

#[allow(dead_code)]
pub mod function;
    use function::{Function, Return};

#[allow(dead_code)]
pub mod terminating_loop;
    use terminating_loop::TerminatingLoop;

#[allow(dead_code)]
pub mod declaration;
    use declaration::{Declaration, DataType};

#[allow(dead_code)]
#[allow(unused_macros)]
pub mod enumerators;
    use enumerators::Assignment;

#[allow(dead_code)]
pub mod error;


#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Token {
    FUNCTION(Function),
    TERMINATINGLOOP(TerminatingLoop),
    DECLARATION(Declaration),
    RETURN(Return),
}


pub struct Tokenizer {
    pub token_tree: Vec<Token>,

    function_history: FunctionHistory,
    syntax_elements: SyntaxElements,

} impl Tokenizer {
    pub fn init() -> Self { Self {
        token_tree: Vec::new(),

        function_history: FunctionHistory::init(),
        syntax_elements: SyntaxElements::init(),
    }}

    pub fn create_token_tree(&mut self, optimized_file_content: &Vec<String>) {
        let token_tree = self.generate_token_tree(&mut None, optimized_file_content);

        self.token_tree = token_tree;
    }

    pub fn generate_token_tree(&mut self, parent_ref: &mut Option<&mut Function>, content_to_tokenize: &Vec<String>) -> Vec<Token> {
        println!("coding_language::tokenizer::Tokenizer::generate_token_tree()");
        let mut result: Vec<Token> = Vec::new();

        let mut i: usize = 0;
        while i < content_to_tokenize.len() {
            let current_word = content_to_tokenize[i].clone();
            println!("  current_word: {current_word:?}");

            // Declarion handling
            match &content_to_tokenize[i] {
                val if val == self.syntax_elements.declaration_names.get("variable").unwrap() => { if let Some(parent) = parent_ref {
                    // Get the first instance of the end assignment character after the
                    // declaration (therefore ending it)
                    let declaration_stop_char = self.syntax_elements.assignment_symbols.get("end assignment").unwrap();
                    let declaration_stop_index = content_to_tokenize.find_after_index(i, declaration_stop_char).unwrap();

                    // Get the slice from this index (the declaration start) to the end
                    // assignment char (the declaration end)
                    let declaration_to_evaluate = content_to_tokenize[i..declaration_stop_index].to_vec();

                    // Parse the slice into a token and add it to the result
                    let created_token = self.parse_variable(&mut parent.variable_history, declaration_to_evaluate);
                    result.push(created_token);

                    // Move the current word to one word after the end of this declaration and
                    // continue the loop
                    i = declaration_stop_index;
                    continue;
                }}

                val if val == self.syntax_elements.declaration_names.get("function").unwrap() => {
                    println!("  found function");
                    // Get the first instance of the end block character after the
                    // declaration (therefore ending it)
                    let block_stop_char = self.syntax_elements.assignment_symbols.get("end body").unwrap();
                    let declaration_stop_index = content_to_tokenize.find_after_index(i, &block_stop_char).unwrap();

                    // Get the slice from this index (the declaration start) to the
                    // block char (the declaration end)
                    let declaration_to_evaluate = content_to_tokenize[i..=declaration_stop_index].to_vec();

                    // Parse the slice into a token and add it to the result
                    let created_token = self.parse_function(declaration_to_evaluate);
                    if let Token::FUNCTION(function) = created_token.clone() {
                        self.function_history.add_function(function);
                    }
                    result.push(created_token);

                    // Move the current word to one word after the end of this declaration and
                    // continue the loop
                    i = declaration_stop_index;
                    continue;
                }

                val if val == self.syntax_elements.declaration_names.get("return").unwrap() => { if let Some(parent) = parent_ref.as_mut() {
                    // Get the first instance of the end block character after the
                    // declaration (therefore ending it)
                    let block_stop_char = self.syntax_elements.assignment_symbols.get("end assignment").unwrap();
                    let declaration_stop_index = content_to_tokenize.find_after_index(i, &block_stop_char).unwrap();

                    // Get the slice from this index (the declaration start) to the
                    // block char (the declaration end)
                    let declaration_to_evaluate = content_to_tokenize[i..=declaration_stop_index-1].to_vec();

                    // Parse the slice into a token and add it to the result
                    let created_token = self.parse_return(parent, declaration_to_evaluate);
                    result.push(created_token);

                    // Move the current word to one word after the end of this declaration and
                    // continue the loop
                    i = declaration_stop_index;
                    continue;
                }}

                _ => {}
            }

            i += 1
        }

        return result
    }

    fn parse_variable(&self, variable_history: &mut VariableHistory, declaration: Vec<String>) -> Token {
        // Get the necessary characters
        let equals_char   = self.syntax_elements.assignment_symbols.get("equals").unwrap();
        let set_type_char = self.syntax_elements.assignment_symbols.get("set type").unwrap();
        

        // Parse the declaration
        let equal_sign_index = declaration.find(equals_char).unwrap();
        let set_type_index   = declaration.find(set_type_char).unwrap();

        // Get the assignment part (everything after equals and before `;`)
        let string_assignment = declaration[equal_sign_index+1..declaration.len()].to_vec();

        // Retrieve the name of te variable, its data_type, and what it's assigned to
        let name = declaration[1].clone();
        let data_type = DataType::check_token_type(&declaration[set_type_index+1]).unwrap();
        let assignment: Assignment = Assignment::from_string_vec(&self, variable_history, string_assignment);
        // Add it to representation variable_history
        let variable_representation = Variable {
            name: name.clone(),
            data_type: data_type.clone(),
        };
        variable_history.add_variable(variable_representation)
            .expect("variable_history does not conclude with None");

        // Build the declaration token
        let declaration = Declaration {
            name: name.to_string(),
            location: variable_history.find_variable(&name).unwrap(),
            data_type,
            value: Some(assignment),
        };

        return Token::DECLARATION(declaration)
    }

    fn parse_float(&self, variable_history: &mut VariableHistory, declaration: Vec<String>) -> Token {
        // Parse the declaration
        let equal_sign_index = declaration.find(&"=".to_owned()).unwrap();
 
        // Get the assignment part (everything after equals and before `;`)
        let string_assignment = declaration[equal_sign_index+1..declaration.len()].to_vec();
 
        // Retrieve the name of te variable, its data_type, and what it's assigned to
        let name = declaration[1].clone();
        let data_type = DataType::check_token_type(&declaration[0]).unwrap();
        let assignment = Assignment::from_string_vec(&self, variable_history, string_assignment);

        // Add it to representation variable_history
        let variable_representation = Variable {
            name: name.clone(),
            data_type: data_type.clone(),
        };
        variable_history.add_variable(variable_representation)
            .expect("variable_history does not conclude with None");
 
        // Build the declaration token
        let declaration = Declaration {
            name: name.clone(),
            location: variable_history.find_variable(&name).unwrap(),
            data_type,
            value: Some(assignment),
        };
 
        return Token::DECLARATION(declaration)
    }

    fn parse_function(&mut self, declaration: Vec<String>) -> Token {
        println!("coding_language::tokenizer::Tokenizer::parse_function()");

        // Get necessary characters
        let block_start_char = self.syntax_elements.assignment_symbols.get("begin body")
            .unwrap();
        let return_this_char = self.syntax_elements.assignment_symbols.get("return this")
            .unwrap();
        let begin_conditions_char = self.syntax_elements.assignment_symbols.get("begin conditions")
            .unwrap();
        let end_conditions_char = self.syntax_elements.assignment_symbols.get("end conditions")
            .unwrap();

        // Get the indexes of the necessary characters
        let block_start_index = declaration.find(block_start_char).unwrap();
        let return_this_index = declaration.find(return_this_char).unwrap();
        let begin_conditions_index = declaration.find(begin_conditions_char).unwrap();
        let end_conditions_index = declaration.find(end_conditions_char).unwrap();

        // Get the function block and given argument slices
        let inline_block_slice = declaration[block_start_index+1..].to_vec();
        let argument_slice_raw = declaration[begin_conditions_index+1..=end_conditions_index-1].to_vec();
        let argument_slice: Vec<&[String]> = argument_slice_raw.split(|x| x==",").collect();
        println!("  argument_slice: {:?}", argument_slice);

        // Parse the arguments by iterating over each of them
        let mut arguments: Vec<Variable> = Vec::new();
        if argument_slice[0].len() > 0 { for argument in argument_slice {
            arguments.push( Variable::from_function_arg(argument.to_vec()) );
        }}

        // Create the function's variable history and add the arguments to it
        let mut variable_history = VariableHistory::init(MEMORY_STEP);
        for argument in arguments.iter() {
            variable_history.add_variable(argument.to_owned()).unwrap();
        }

        // Parse the function with the given infomation
        let name = declaration[1].to_string();
        let return_type_text = declaration[return_this_index+1].to_owned();
        let return_type = DataType::check_token_type(&return_type_text).unwrap();

        // Construct the function
        let mut function = Function {
            name,
            return_type,
            variable_history,
            arguments,
            functionaliy: Vec::new(),
        };

        // Define the function's functionality
        let inline_block = self.generate_token_tree(&mut Some(&mut function), &inline_block_slice);
        function.functionaliy = inline_block;

        // Return it
        return Token::FUNCTION(function)
    }

    fn parse_return(&self, parent: &Function, return_statement: Vec<String>) -> Token {
        println!("coding_language::tokenizer::Tokenizer::parse_return()");
        println!("  recieved: {:?}", return_statement);

        let assignment_slice = return_statement[1..].to_vec();

        let assignment: Assignment = {
            Assignment::from_string_vec(&self, &parent.variable_history, assignment_slice)
        };
        
        let return_token = Return {
            assignment,
        };

        return Token::RETURN(return_token)
    }
}
