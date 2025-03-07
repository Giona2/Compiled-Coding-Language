use std::fs;


mod tokens;
    use tokens::SyntaxTree;
mod type_traits;

fn main() {
    let file_content: String = fs::read_to_string("./syntax_example.txt")
        .expect("Failed to read file");

    let optimized_file_content = SyntaxTree::optimize_file_content(&file_content);
    println!("Optimized files content: {:?}", optimized_file_content);
    let syntax_tree = SyntaxTree::from_file_content(optimized_file_content);
    println!("Syntax tree: {:?}", syntax_tree);
}
