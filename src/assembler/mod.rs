use crate::tokenizer::{TokenTree, Token};


#[allow(dead_code)]
pub mod error;
    use error::AssemblerError;

#[allow(dead_code)]
pub mod types_translator;


pub struct Assembler {
    file_content: Vec<String>
}
impl Assembler {
    pub fn from_token_tree(token_tree: &TokenTree) -> Result<Self, AssemblerError> {
        // Write the start to the program
        let mut file_content: Vec<String> = vec![
            "global _start",
            "_start:",
            "  push rbp",
            "  mov rbp, rsp",
        ].iter().map(|x| x.to_string()).collect();

        for token in token_tree.token_tree.iter() { match token {
            Token::DECLARATION(declaration) => {
                let appended_file_content: Vec<String> = vec![
                    format!("  sub rsp {}", token_tree.stack_memory.step),
                    format!("  mov QWORD [rbp-{}], {}", token_tree.stack_memory.step, "")
                ].iter().map(|x| x.to_string()).collect();
            }
            _ => {}
        }}

        file_content.append(&mut vec![
	        "    mov rsp, rbp",
	        "    pop rbp",

	        "    mov rax, 60",
	        "    mov rdi, 0",
	        "    syscall",
        ].iter().map(|x| x.to_string()).collect());

        todo!()
    }
}
