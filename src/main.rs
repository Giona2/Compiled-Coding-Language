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

    let tokenizer = Tokenizer::from_file_content(&optimizer);

    let assembler = Assembler::from_token_tree(&tokenizer);
}

#[cfg(test)]
mod testing {}
