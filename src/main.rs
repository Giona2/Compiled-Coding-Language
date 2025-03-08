use std::fs;


mod tokenizer;
    use tokenizer::SyntaxTree;

#[allow(dead_code)]
mod type_traits;


fn main() {
    let file_content: String = fs::read_to_string("./syntax_example.txt")
        .expect("Failed to read file");
    let optimized_file_content = SyntaxTree::optimize_file_content(&file_content);

    let syntax_tree = SyntaxTree::from_file_content(optimized_file_content);
    println!("token_tree: \n{:?}", syntax_tree.token_tree);
    println!();
    println!("stack_memory: \n{:?}", syntax_tree.stack_memory);
}

#[cfg(test)]
mod testing {}
