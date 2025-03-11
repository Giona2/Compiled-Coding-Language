use std::fs;


mod tokenizer;
    use tokenizer::TokenTree;

mod assembler;

mod optimizer;
    use optimizer::Optimizer;

#[allow(dead_code)]
mod type_traits;

#[allow(dead_code)]
mod data;


fn main() {
    let file_content: String = fs::read_to_string("./examples/syntax_example.txt")
        .expect("Failed to read file");
    let optimized_file_content = Optimizer::from_file_content(&file_content);

    let syntax_tree = TokenTree::from_file_content(&optimized_file_content);
    println!("{syntax_tree:?}")
}

#[cfg(test)]
mod testing {}
