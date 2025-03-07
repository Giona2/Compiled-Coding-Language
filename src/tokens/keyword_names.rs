pub mod types {
    pub fn get_type_names() -> Vec<String> { return vec![
        "int",

    ].into_iter().map(|x| x.to_string()).collect()}
    pub const INTEGER: &str = "int";
}

pub fn get_syntax_chars() -> Vec<String> { return vec![
    "{",
    "}",
    "int",
    "=",
    ";",
    "(",
    ")",
    ",",
    "+",
    "*",
    "/",
    "-",

].into_iter().map(|x| x.to_string()).collect()}
