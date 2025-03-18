use std::vec;

use crate::tokenizer::{constructors::Declaration, representations::StackMemory, types::Assignment, Token};


#[allow(dead_code)]
pub mod error;
    use error::AssemblerError;

#[allow(dead_code)]
pub mod types_translator;
    use types_translator::AssignmentToAssembly;


pub struct Assembler {
    pub instructions: Vec<String>
}
impl Assembler {
    pub fn init() -> Self { Self {
        instructions: Vec::new(),
    }}

    pub fn generate_instructions(&mut self, token_tree: &Vec<Token>, stack_memory: &StackMemory) -> Result<(), AssemblerError> {
        // Write the start to the program
        let mut program_instructions: Vec<String> = vec![
            "global _start",
            "_start:",
            "  push rbp",
            "  mov rbp, rsp",
            "",
        ].iter().map(|x| x.to_string()).collect();

        // Iterate over each token and translate it accordingly
        for token in token_tree.iter() { match token {
            Token::DECLARATION(declaration) => { program_instructions = vec![
                program_instructions,
                self.assemble_declaration(stack_memory, declaration)
            ].concat()}
            _ => {}
        }}

        // Write the end to the program
        program_instructions.append(&mut vec![
            "",
            ".exit:",
	        "  mov rsp, rbp",
	        "  pop rbp",

	        "  mov rax, 60",
	        "  mov rdi, 0",
	        "  syscall",
        ].iter().map(|x| x.to_string()).collect());

        self.instructions = program_instructions;

        return Ok(())
    }

    fn assemble_declaration(&self, stack_memory: &StackMemory, declaration: &Declaration) -> Vec<String> { match declaration.clone().value.unwrap() {
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
