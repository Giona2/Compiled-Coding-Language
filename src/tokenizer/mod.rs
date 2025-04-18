use crate::type_traits::vector::VecExtra;
use crate::data::{SyntaxElements, MEMORY_STEP};


#[allow(dead_code)]
pub mod conditional_loop;
    use conditional_loop::ConditionalLoop;

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
mod error;
    use error::TokenizerError;

#[allow(dead_code)]
pub mod reassignment;
    use reassignment::Reassignment;

#[allow(dead_code)]
pub mod conditional_statement;
    use conditional_statement::ConditionalStatement;


#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Token {
    ConditionalLoop(ConditionalLoop),
    Function(Function),
    TerminatingLoop(TerminatingLoop),
    ConditionalStatement(ConditionalStatement),
    Declaration(Declaration),
    Reassignment(Reassignment),
    Return(Return),
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
        let mut result: Vec<Token> = Vec::new();

        let mut i: usize = 0;
        while i < content_to_tokenize.len() {
            let current_word = content_to_tokenize[i].clone();

            // Declarion handling
            match &current_word {
                val if *val == self.syntax_elements.declaration_names["variable"] => { if let Some(parent) = parent_ref {
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

                val if *val == self.syntax_elements.declaration_names["reassignment"] => { if let Some(parent) = parent_ref {
                    // Get the first instance of the end assignment character after the
                    // declaration (therefore ending it)
                    let declaration_stop_char = self.syntax_elements.assignment_symbols.get("end assignment").unwrap();
                    let declaration_stop_index = content_to_tokenize.find_after_index(i, declaration_stop_char).unwrap();

                    // Get the slice from this index (the declaration start) to the end
                    // assignment char (the declaration end)
                    let declaration_to_evaluate = content_to_tokenize[i..declaration_stop_index].to_vec();

                    // Parse the slice into a token and add it to the result
                    let created_token = self.parse_reassignment(&parent.variable_history, declaration_to_evaluate);
                    result.push(created_token);

                    // Move the current word to one word after the end of this declaration and
                    // continue the loop
                    i = declaration_stop_index;
                    continue;
                }}

                val if *val == self.syntax_elements.declaration_names["function"] => {
                    // Get the first instance of the end block character after the
                    // declaration (therefore ending it)
                    let block_start_char = self.syntax_elements.assignment_symbols.get("begin body").unwrap();
                    let block_start_index = content_to_tokenize.find_after_index(i, block_start_char).unwrap();
                    let declaration_stop_index = self.find_end_of_block(&content_to_tokenize, block_start_index).unwrap();

                    // Get the slice from this index (the declaration start) to the
                    // block char (the declaration end)
                    let declaration_to_evaluate = content_to_tokenize[i..=declaration_stop_index].to_vec();

                    // Parse the slice into a token and add it to the result
                    let created_token = self.parse_function(declaration_to_evaluate);
                    if let Token::Function(function) = created_token.clone() {
                        self.function_history.add_function(function);
                    }
                    result.push(created_token);

                    // Move the current word to one word after the end of this declaration and
                    // continue the loop
                    i = declaration_stop_index;
                    continue;
                }

                val if *val == self.syntax_elements.declaration_names["conditional statement"] => { if let Some(parent) = parent_ref.as_mut() {
                    // Get necessary chars
                    let block_start_char = self.syntax_elements.assignment_symbols["begin body"].clone();

                    // Get the index of the chars
                    let block_start_index = content_to_tokenize.find_after_index(i, &block_start_char).unwrap();

                    // Get the index of the end of this comparison
                    let declaration_stop_index = self.find_end_of_block(content_to_tokenize, block_start_index).unwrap();

                    // Parse the slice into a token and add it to the result
                    let created_token = self.parse_conditional_statement(parent, content_to_tokenize[i..=declaration_stop_index].to_vec())
                        .unwrap();
                    result.push(created_token);

                    // Move the current word to one word after the end of this declaration and
                    // continue the loop
                    i = declaration_stop_index;
                    continue;
                }}

                val if *val == self.syntax_elements.declaration_names["conditional loop"] => { if let Some(parent) = parent_ref.as_mut() {
                    // Get necessary chars
                    let block_start_char = self.syntax_elements.assignment_symbols["begin body"].clone();

                    // Get the index of the chars
                    let block_start_index = content_to_tokenize.find_after_index(i, &block_start_char).unwrap();

                    // Get the index of the end of this comparison
                    let declaration_stop_index = self.find_end_of_block(content_to_tokenize, block_start_index).unwrap();

                    // Parse the slice into a token and add it to the result
                    let created_token = self.parse_conditional_loop(parent, content_to_tokenize[i..=declaration_stop_index].to_vec());
                    result.push(created_token);

                    // Move the current word to one word after the end of this declaration and
                    // continue the loop
                    i = declaration_stop_index;
                    continue;
                }}

                val if *val == self.syntax_elements.declaration_names["return"] => { if let Some(parent) = parent_ref.as_mut() {
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

    /// Finds the index of the end of a code block given the start of said block
    ///
    /// This is usefull for finding the end of a function containing blocks within itself (if
    /// statements, for example)
    ///
    /// Note that you should place the start_index at the beginning of the block
    fn find_end_of_block(&self, content: &[String], start_index: usize) -> Result<usize, TokenizerError> {
        // Keep track of the end_of_block_index
        let mut end_of_block_index: Option<usize> = None;

        // Keep track of the amount of blocks currently counted
        let mut current_block_counter: usize = 0;

        // Iterate through each character
        let mut i = start_index;
        while i < content.len() {
            if content[i] == self.syntax_elements.assignment_symbols["begin body"] {
                current_block_counter += 1
            }
            else if content[i] == self.syntax_elements.assignment_symbols["end body"] {
                current_block_counter -= 1
            }

            // if the block counter is 0, you reached the end so set the result
            if current_block_counter == 0 {
                end_of_block_index = Some(i);
                break
            }

            i+=1
        }

        // If the end of the block was found, return it
        // Otherwise, return an error
        match end_of_block_index {
            Some(index) => { return Ok(index) }
            None        => { return Err(TokenizerError::CouldNotFindEndOfBlock) }
        }
    }

    fn parse_conditional_loop(&mut self, parent: &mut Function, conditional_loop: Vec<String>) -> Token {
        // Get necessary characters
        let begin_condition_char = self.syntax_elements.assignment_symbols["begin loop condition"].clone();
        let end_condition_char   = self.syntax_elements.assignment_symbols["end loop condition"].clone();
        let begin_body_char      = self.syntax_elements.assignment_symbols["begin body"].clone();

        // Parse the condition
        let begin_condition_index = conditional_loop.find(&begin_condition_char).unwrap();
        let end_condition_index   = conditional_loop.find(&end_condition_char).unwrap();
        let condition_slice       = conditional_loop[begin_condition_index+1..=end_condition_index-1].to_vec();
        let condition             = Assignment::from_string_vec(self, &parent.variable_history, condition_slice);

        // Parse the body
        let begin_body_index = conditional_loop.find(&begin_body_char).unwrap();
        let end_body_index   = self.find_end_of_block(&conditional_loop, begin_body_index).unwrap();
        let body_slice       = conditional_loop[begin_body_index+1..=end_body_index-1].to_owned();
        let functionality    = self.generate_token_tree(&mut Some(parent), &body_slice);

        // Construct the token
        let conditional_loop_token = ConditionalLoop {
            condition,
            functionality,
        };

        // Return it
        return Token::ConditionalLoop(conditional_loop_token);
    }

    fn parse_conditional_statement(&mut self, parent: &mut Function, conditional_statement: Vec<String>) -> Result<Token, TokenizerError> {
        // Get necessary characters
        let begin_comparison_conditions_char = self.syntax_elements.assignment_symbols["begin comparison conditions"].clone();
        let end_comparison_conditions_char   = self.syntax_elements.assignment_symbols["end comparison conditions"].clone();
        let begin_enclosure_char             = self.syntax_elements.assignment_symbols["begin enclosure"].clone();
        let end_enclosure_char               = self.syntax_elements.assignment_symbols["end enclosure"].clone();
        let begin_block_char                 = self.syntax_elements.assignment_symbols["begin body"].clone();
        let else_comparison_statement_char   = self.syntax_elements.declaration_names["else conditional statement"].clone();

        // Get active variables slice
        let begin_comparison_conditions_index = conditional_statement.find(&begin_comparison_conditions_char).unwrap();
        let end_comparison_conditions_index   = conditional_statement.find(&end_comparison_conditions_char  ).unwrap();
        let comparison_conditions_slice_raw: Vec<String> = conditional_statement[begin_comparison_conditions_index+1..=end_comparison_conditions_index-1].to_vec();
        let comparison_conditions_slice: Vec<&[String]> = comparison_conditions_slice_raw.split(|x| x==",").collect();

        // Parse it by iterating over each variable passed
        let mut active_variables: Vec<usize> = Vec::new();
        for variable in comparison_conditions_slice.iter() {
            let variable_location = parent.variable_history.find_variable(&variable[0]).unwrap();
            active_variables.push(variable_location);
        }

        // Get each condition field
        let mut condition_fields_slices: Vec<(Option<Assignment>, Vec<Token>)> = Vec::new();
        let mut i = conditional_statement.find(&begin_enclosure_char).unwrap();
        while i < conditional_statement.len() {
            // get index of necessary chars
            let end_enclosure_index = conditional_statement.find_after_index(i, &end_enclosure_char).unwrap();
            let enclosure_slice     = conditional_statement[i+1..=end_enclosure_index-1].to_owned();

            // get the condition of current field
            let field_condition: Option<Assignment>;
            if enclosure_slice == [else_comparison_statement_char.clone()] {
                field_condition = None;
            } else {
                field_condition = Some(Assignment::from_string_vec(self, &parent.variable_history, enclosure_slice));
            }

            // get the block index and parse it
            let block_start_index  = conditional_statement.find_after_index(i, &begin_block_char).unwrap();
            let block_end_index    = self.find_end_of_block(&conditional_statement, block_start_index).unwrap();
            let inline_block_slice = conditional_statement[block_start_index+1..=block_end_index-1].to_owned();
            let inline_block = self.generate_token_tree(&mut Some(parent), &inline_block_slice);

            // make sure each variable was passed
            if let Some(assignment) = &field_condition {
                let vars_used_in_assignment = assignment.get_all_vars_used();
                for var in vars_used_in_assignment.iter() {
                    if let None = active_variables.find(var) { return Err(TokenizerError::VarNotUsedInComparison) }
                }
            }

            // push this
            condition_fields_slices.push((field_condition, inline_block));

            // if there is another enclosure, jump i to there. If not, break the loop
            if let Some(found_index) = conditional_statement.find_after_index(end_enclosure_index, &begin_enclosure_char) {
                i = found_index
            } else {
                break;
            }
        }

        let conditional_statement_token = ConditionalStatement {
            active_variables,
            condition_fields: condition_fields_slices,
        };

        return Ok(Token::ConditionalStatement(conditional_statement_token))
    }

    fn parse_reassignment(&self, variable_history: &VariableHistory, reassignment: Vec<String>) -> Token {
        // Get the necessary characters
        let equals_char = self.syntax_elements.assignment_symbols.get("equals").unwrap();

        // Parse the declaration
        let equal_sign_index = reassignment.find(equals_char).unwrap();

        // Get the assignment part (everything after equals and before `\n`)
        let string_assignment = reassignment[equal_sign_index+1..reassignment.len()].to_vec();

        // Retrieve the name of te variable and what it's newly assigned to
        let name = reassignment[1].clone();
        let new_assignment: Assignment = Assignment::from_string_vec(&self, variable_history, string_assignment);

        // Ensure the variable is in variable_history
        variable_history.find_variable(&name).expect("Variable does not exist");

        // Build the declaration token
        let reassignment_token = Reassignment {
            name,
            new_assignment,
        };

        return Token::Reassignment(reassignment_token)
    }

    fn parse_variable(&self, variable_history: &mut VariableHistory, declaration: Vec<String>) -> Token {
        // Get the necessary characters
        let equals_char         = self.syntax_elements.assignment_symbols.get("equals").unwrap();
        let begin_set_type_char = self.syntax_elements.assignment_symbols.get("begin set type").unwrap();
        let end_set_type_char   = self.syntax_elements.assignment_symbols.get("end set type").unwrap();

        // Parse the declaration
        let equal_sign_index     = declaration.find(equals_char).unwrap();
        let begin_set_type_index = declaration.find(begin_set_type_char).unwrap();
        let end_set_type_index   = declaration.find(end_set_type_char).unwrap();

        // Get the assignment part (everything after equals and before `;`)
        let string_assignment = declaration[equal_sign_index+1..declaration.len()].to_vec();
        let data_type_slice = declaration[begin_set_type_index+1..=end_set_type_index-1].to_vec();

        // Retrieve the name of te variable, its data_type, and what it's assigned to
        let name = declaration[1].clone();
        let data_type = DataType::check_token_type(&data_type_slice[0]).unwrap();
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
            value: assignment,
        };

        return Token::Declaration(declaration)
    }

    fn parse_function(&mut self, declaration: Vec<String>) -> Token {
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
        return Token::Function(function)
    }

    fn parse_return(&self, parent: &Function, return_statement: Vec<String>) -> Token {
        let assignment_slice = return_statement[1..].to_vec();

        let assignment: Assignment = {
            Assignment::from_string_vec(&self, &parent.variable_history, assignment_slice)
        };
        
        let return_token = Return {
            assignment,
        };

        return Token::Return(return_token)
    }
}
