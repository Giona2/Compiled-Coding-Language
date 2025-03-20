use std::vec;

use crate::tokenizer::{
    declaration::Declaration,
    function::Function,
    structures::VariableHistory,
    enumerators::Assignment, Token
};


#[allow(dead_code)]
pub mod error;
    use error::AssemblerError;

#[allow(dead_code)]
pub mod types_translator;
    use types_translator::AssignmentToAssembly;


const FUNCTION_ARGUMENT_REGISTERS: [&str; 6] = ["rdi", "rsi", "rdx", "r10", "r8", "r9"];


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

        // Iterate over each token and translate it accordingly
        for token in token_tree.iter() { match token {
            Token::FUNCTION(function) => program_instructions.append(&mut self.assemble_function(function)),
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
        for (argument_index, argument) in function.args.iter().enumerate() {
            function_instructions.append(&mut vec![
                format!("  push {}", FUNCTION_ARGUMENT_REGISTERS[argument_index]),
                format!("  sub rsp, 8"),
                format!("  sub rsp, 8"),
                format!("  mov QWORD [rbp-{}]"),
            ]);
        }

        // Assemble the functionality
        for token in function.functionaliy.iter() { match token {
            Token::DECLARATION(declaration) => {
                function_instructions.append(&mut self.assemble_declaration(&function.memory, declaration));
            }
            _ => {}
        }}

        // Function end
        function_instructions.append(&mut vec![
            format!(".end:"),
            format!("  mov rsp, rbp"),
            format!("  pop rbp"),
            format!("  ret"),
            format!(""),
        ]);

        // Return the result
        return function_instructions
    }

    fn assemble_declaration(&self, stack_memory: &VariableHistory, declaration: &Declaration) -> Vec<String> { match declaration.clone().value.unwrap() {
        Assignment::INTEGER(integer_assignment) => {
            let assignment_instructions = integer_assignment.to_assembly(stack_memory);
            let appended_instructions: Vec<String> = vec![
                vec![
                    format!("  sub rsp, {}", stack_memory.step),
                    format!("  push rax"),
                ],
                assignment_instructions,
                vec![
                    format!("  mov QWORD [rbp-{}], rax", (declaration.location+1) * stack_memory.step),
                    format!("  pop rax"),
                ],
            ].concat().iter().map(|x| x.to_string()).collect();
            return appended_instructions
        }

        Assignment::FLOAT(float_assignment) => {
            let assignment_instructions = float_assignment.to_assembly(stack_memory);
            let appended_instructions: Vec<String> = vec![
                vec![
                    format!("  sub rsp, {}", stack_memory.step),
                    format!("  push rax"),
                ],
                assignment_instructions,
                vec![
                    format!("  mov QWORD [rbp-{}], rax", (declaration.location+1) * stack_memory.step),
                    format!("  pop rax"),
                ],
            ].concat().iter().map(|x| x.to_string()).collect();
            return appended_instructions
        }
    }}
}
