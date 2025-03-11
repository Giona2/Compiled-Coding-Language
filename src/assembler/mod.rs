use crate::tokenizer::{Token, representations::StackMemory};


#[allow(dead_code)]
pub mod error;
    use error::AssemblerError;

#[allow(dead_code)]
pub mod types_translator;


pub struct Assembler {
    pub instructions: Vec<String>
}
impl Assembler {
    pub fn from_token_tree(token_tree: &Vec<Token>, stack_memory: &StackMemory) -> Result<Self, AssemblerError> {
        // Write the start to the program
        let mut program_instructions: Vec<String> = vec![
            "global _start",
            "_start:",
            "  push rbp",
            "  mov rbp, rsp",
        ].iter().map(|x| x.to_string()).collect();

        // Iterate over each token and translate it accordingly
        for token in token_tree.iter() { match token {
            Token::DECLARATION(declaration) => {
                let appended_instructions: Vec<String> = vec![
                    format!("  sub rsp {}", stack_memory.step),
                    format!("  mov QWORD [rbp-{}], {}", stack_memory.step, "")
                ].iter().map(|x| x.to_string()).collect();
                program_instructions = vec![program_instructions.clone(), appended_instructions].concat();
            }
            _ => {}
        }}

        // Write the end to the program
        program_instructions.append(&mut vec![
	        "  mov rsp, rbp",
	        "  pop rbp",

	        "  mov rax, 60",
	        "  mov rdi, 0",
	        "  syscall",
        ].iter().map(|x| x.to_string()).collect());

        return Ok(Self{
            instructions: program_instructions,
        })
    }
}
