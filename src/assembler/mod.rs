use crate::tokenizer::{Token, representations::StackMemory};


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
    pub fn from_token_tree(token_tree: &Vec<Token>, stack_memory: &StackMemory) -> Result<Self, AssemblerError> {
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
            Token::DECLARATION(declaration) => {
                let assignment_instructions = declaration.clone()
                    .value.unwrap()
                    .to_assembly(stack_memory);

                println!("Assembler::from_token_tree");
                println!("From:     {declaration:?}");
                println!("Returned: {assignment_instructions:?}");

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
                program_instructions = vec![program_instructions.clone(), appended_instructions].concat();
            }
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

        return Ok(Self{
            instructions: program_instructions,
        })
    }
}
