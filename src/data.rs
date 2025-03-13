use std::collections::HashMap;


pub struct SyntaxElements {
    type_names: Vec<String>,
    type_name_table: HashMap<String, String>,
    math_symbols: Vec<String>,
    math_symbol_table: HashMap<String, String>,
    comparison_symbols: Vec<String>,
    comparson_symbol_table: HashMap<String, String>,
} impl SyntaxElements {
}

pub mod syntactic_elements {

    pub mod types {
        pub fn get_type_names() -> Vec<String> { return vec![
            "int",
            "float",

        ].into_iter().map(|x| x.to_string()).collect()}
        pub const INTEGER: &str = "int";
        pub const FLOAT:   &str = "float";
    }

    pub mod math {
        pub const ADD:      &str = "+";
        pub const SUBTRACT: &str = "-";
        pub const MULTIPLY: &str = "*";
        pub const DIVIDE:   &str = "/";
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
        "<",
        ">",

    ].into_iter().map(|x| x.to_string()).collect()}
}
