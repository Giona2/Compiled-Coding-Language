use std::fs;


mod tokens;
    use tokens::Token;

fn main() {
    let script_content_raw = fs::read_to_string("./syntax_example.txt")
        .expect("Could not read");
    let script_content_flattened = script_content_raw
        .replace("\n", " ")
        .replace("\t", " ");
    let script_content_split: Vec<&str> = script_content_flattened
        .split(" ")
        .collect();
    let script_content: Vec<&str> = script_content_split.into_iter()
        .filter_map(|x| if x != "" {Some(x)} else {None})
        .collect();

    let mut syntax_tree: Vec<Token> = Vec::new();

    for word in script_content {}
}
