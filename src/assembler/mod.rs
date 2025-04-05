use std::vec;

use crate::tokenizer::{
    conditional_statement::ConditionalStatement, declaration::Declaration, enumerators::{Assignment, ComparisonOperator}, function::{Function, Return}, reassignment::Reassignment, structures::VariableHistory, Token
};


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
            }
            Token::ConditionalStatement(conditional_statement) => {
                function_instructions.append(&mut self.assemble_conditional_statement(&function.variable_history, conditional_statement).unwrap());
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

    fn assemble_conditional_statement(&self, variable_history: &VariableHistory, conditional_statement: &ConditionalStatement) -> Result<Vec<String>, AssemblerError> {
        let mut appended_instructions: Vec<String> = Vec::new();

        let branch_name = format!(".cmp{}", conditional_statement.index);

        // Assemble the header
        for (i, (condition_wrapped, _)) in conditional_statement.condition_fields.iter().enumerate() {

            if let Some(condition) = condition_wrapped { if let Assignment::CMP(first_assignment, operator, second_assignment) = condition {
                // put first and second values into registers
                appended_instructions.append(&mut vec![
                    first_assignment.to_assembly_instructions("rdi", variable_history).unwrap(),
                    second_assignment.to_assembly_instructions("rsi", variable_history).unwrap(),
                ].concat());
                appended_instructions.append(&mut vec![
                    format!("  cmp rdi, rsi")
                ]);

                match operator {
                    ComparisonOperator::EQ  => { appended_instructions.append(&mut vec![
                        format!("  je {}_br{}", branch_name, i),
                    ]);}
                    ComparisonOperator::NEQ => { appended_instructions.append(&mut vec![
                        format!("  jne {}_br{}", branch_name, i),
                    ]);}
                    ComparisonOperator::GT  => { appended_instructions.append(&mut vec![
                        format!("  jg {}_br{}", branch_name, i),
                    ]);}
                    ComparisonOperator::GEQ => { appended_instructions.append(&mut vec![
                        format!("  jge {}_br{}", branch_name, i),
                    ]);}
                    ComparisonOperator::LT  => { appended_instructions.append(&mut vec![
                        format!("  jl {}_br{}", branch_name, i),
                    ]);}
                    ComparisonOperator::LEQ => { appended_instructions.append(&mut vec![
                        format!("  jle {}_br{}", branch_name, i),
                    ]);}
                }
            } else { return Err(AssemblerError::AssignmentInComparisonNotComparison); } }
            else { appended_instructions.append(&mut vec![
                format!("  jmp {}_br{}", branch_name, i)
            ]);}
        }
        appended_instructions.append(&mut vec![
            format!("  jmp {}_end", branch_name)
        ]);

        // Assemble the branches
        for (i, (_, token_tree)) in conditional_statement.condition_fields.iter().enumerate() {
            // declare the branch
            appended_instructions.append(&mut vec![
                format!("{}_br{}:", branch_name, i)
            ]);

            // assemble its declaration
            for token in token_tree.iter() { match token {
                Token::Declaration(declaration) => {
                    appended_instructions.append(&mut self.assemble_declaration(variable_history, declaration));
                }
                Token::Reassignment(reassignment) => {
                    appended_instructions.append(&mut self.assemble_reassignment(variable_history, reassignment));
                }
                Token::Return(return_statement) => {
                    appended_instructions.append(&mut self.assemble_return(variable_history, return_statement));
                }
                _ => {}
            }}

            // conclude it by jumping to the end
            appended_instructions.append(&mut vec![
                format!("  jmp {}_end", branch_name)
            ]);
        }

        // Assemble the end branch
        appended_instructions.append(&mut vec![
            format!("{}_end:", branch_name)
        ]);

        return Ok(appended_instructions)
    }

    fn assemble_declaration(&self, stack_memory: &VariableHistory, declaration: &Declaration) -> Vec<String> {
        let assignment_instructions = declaration.value.clone().to_assembly_instructions("rdi", stack_memory);
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
        let assignment_instructions = reassignment.new_assignment.clone().to_assembly_instructions("rdi", variable_history);

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
        let mut assignment_instructions = return_statement.assignment.to_assembly_instructions("rdi", variable_history)
            .unwrap();

        assignment_instructions.append(&mut vec![
            format!("  mov rax, rdi")
        ]);

        return assignment_instructions
    }
}
