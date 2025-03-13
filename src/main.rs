use std::fs;


mod tokenizer;
    use tokenizer::Tokenizer;

mod assembler;
    use assembler::Assembler;

mod optimizer;
    use optimizer::Optimizer;

#[allow(dead_code)]
mod type_traits;

#[allow(dead_code)]
mod data;


fn main() {
    let file_content: String = fs::read_to_string("./examples/syntax_example.txt")
        .expect("Failed to read file");

    let optimizer = Optimizer::from_file_content(&file_content);

    let mut tokenizer = Tokenizer::init(8);
    tokenizer.generate_token_tree(&optimizer.content);
    println!("tokenizer: {tokenizer:?}");

    let mut assembler = Assembler::init();
    assembler.generate_instructions(&tokenizer.token_tree, &tokenizer.stack_memory).unwrap();
    
    let program_content = assembler.instructions.join("\n");
    fs::write("./a.asm", program_content)
        .expect("Failed to write a.asm");
}

#[cfg(test)]
mod testing {
    #[test]
    fn number_parsing() {
        let number_to_parse: String = "3.2".to_string();
        let result: i32 = number_to_parse.parse()
            .expect("Failed");
        println!("result: {}", result);
    }
}
