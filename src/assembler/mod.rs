use std::vec;

use crate::{tokenizer::{
    conditional_statement::ConditionalStatement, declaration::Declaration, enumerators::Assignment, function::{Function, Return}, reassignment::Reassignment, structures::VariableHistory, Token
}, type_traits::vector::{StrVecExtra, VecExtra}};


#[allow(dead_code)]
pub mod error;
    use error::AssemblerError;

#[allow(dead_code)]
pub mod types_translator;
    use types_translator::AssignmentToAssembly;

#[allow(dead_code)]
pub mod data;
    use data::FUNCTION_ARGUMENT_REGISTERS;
    use data::core_utils;


pub struct Assembler {
    pub instructions: Vec<String>
}
impl Assembler {
    pub fn init() -> Self { Self {
        instructions: Vec::new(),
    }}

    pub fn generate_instructions(&mut self, token_tree: &Vec<Token>) -> Result<(), AssemblerError> {
        // Write the program's entry point
        let mut program_instructions: Vec<String> = vec![
            "global _start",
            "_start:",
            "  call main",
            ".exit:",
            "  mov rdi, rax",
            "  mov rax, 60",
	        "  syscall",
            "",
        ].iter().map(|x| x.to_string()).collect();

        // Write the basic utilities
        program_instructions.append(&mut core_utils::get_all());

        // Iterate over each token and translate it accordingly
        for token in token_tree.iter() { match token {
            Token::Function(function) => program_instructions.append(&mut self.assemble_function(function)),
            _ => {}
        }}

        self.instructions = program_instructions;

        return Ok(())
    }

    fn assemble_function(&self, function: &Function) -> Vec<String> {
        // Function start
        let mut function_instructions: Vec<String> = vec![
            format!("{}:", function.name),
            format!("  push rbp"),
            format!("  mov rbp, rsp"),
            format!(""),
        ];

        // Initialize arguments
        for (argument_index, argument) in function.arguments.iter().enumerate() {
            let variable_location_index = function.variable_history.find_variable(&argument.name).unwrap();
            let variable_location = (variable_location_index+1) * function.variable_history.step;
            let active_register = FUNCTION_ARGUMENT_REGISTERS[argument_index];

            function_instructions.append(&mut vec![
                format!("  sub rsp, 8"),
                format!("  mov QWORD [rbp-{}], {}", variable_location, active_register),
                format!("  push {}", active_register),
            ]);
        }

        // Assemble the functionality
        for token in function.functionaliy.iter() { match token {
            Token::Declaration(declaration) => {
                function_instructions.append(&mut self.assemble_declaration(&function.variable_history, declaration));
            }
            Token::Reassignment(reassignment) => {
                function_instructions.append(&mut self.assemble_reassignment(&function.variable_history, reassignment));
            }
            Token::Return(return_statement) => {
                function_instructions.append(&mut self.assemble_return(&function.variable_history, return_statement));
            } Token::ConditionalStatement(conditional_statement) => {
                function_instructions.append(&mut self.assemble_conditional_statement(&function.variable_history, conditional_statement));
            }
            _ => {}
        }}

        // Begin the function's end
        function_instructions.append(&mut vec![
                format!(".end:"),
        ]);

        // Return the active registers to their original values
        for (argument_index, _) in function.arguments.iter().enumerate() {
            let active_register = FUNCTION_ARGUMENT_REGISTERS[argument_index];

            function_instructions.append(&mut vec![
                format!("  pop {}", active_register),
            ]);
        }

        // Reset the stack frame and return
        function_instructions.append(&mut vec![
            format!("  mov rsp, rbp"),
            format!("  pop rbp"),
            format!("  ret"),
            format!(""),
        ]);

        // Return the result
        return function_instructions
    }

    fn assemble_conditional_statement(&self, variable_history: &VariableHistory, conditional_statement: &ConditionalStatement) -> Vec<String> {
        let mut appended_instructions: Vec<String> = Vec::new();

        let conditional_statement_label_name = format!(".cs{}", conditional_statement.index);

        let condition_instructions = conditional_statement.condition.to_assembly_instructions(variable_history).unwrap();
        appended_instructions.append_immut(&condition_instructions);

        appended_instructions.append(&mut vec![
            format!("  cmp rdi, 1"),
            format!("  je {}_t", conditional_statement_label_name),
            format!("  jmp {}_f", conditional_statement_label_name),
        ]);

        appended_instructions.append(&mut vec![
            format!("{}_t:", conditional_statement_label_name),
        ]);

        return appended_instructions
    }

    fn assemble_declaration(&self, stack_memory: &VariableHistory, declaration: &Declaration) -> Vec<String> {
        let assignment_instructions = declaration.value.clone().to_assembly_instructions(stack_memory);
        let appended_instructions: Vec<String> = vec![
            vec![
                format!("  sub rsp, {}", stack_memory.step),
            ],
            assignment_instructions.unwrap(),
            vec![
                format!("  mov rax, rdi"),
                format!("  mov QWORD [rbp-{}], rax", (declaration.location+1) * stack_memory.step),
            ],
        ].concat().iter().map(|x| x.to_string()).collect();
        return appended_instructions
    }

    fn assemble_reassignment(&self, variable_history: &VariableHistory, reassignment: &Reassignment) -> Vec<String> {
        let assignment_instructions = reassignment.new_assignment.clone().to_assembly_instructions(variable_history);

        let variable_location = variable_history.find_variable(&reassignment.name).unwrap();

        let appended_instructions: Vec<String> = vec![
            assignment_instructions.unwrap(),
            vec![
                format!("  mov rax, rdi"),
                format!("  mov QWORD [rbp-{}], rax", (variable_location+1) * variable_history.step),
            ],
        ].concat().iter().map(|x| x.to_string()).collect();
        return appended_instructions
    }

    fn assemble_return(&self, variable_history: &VariableHistory, return_statement: &Return) -> Vec<String> {
        let mut assignment_instructions = return_statement.assignment.to_assembly_instructions(variable_history)
            .unwrap();

        assignment_instructions.append(&mut vec![
            format!("  mov rax, rdi")
        ]);

        return assignment_instructions
    }
}
