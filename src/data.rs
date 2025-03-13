use crate::hashmap;
use crate::type_traits::str_str_hashmap::StrStrHashMapExtra;

use std::collections::HashMap;


pub struct SyntaxElements {
    type_name_table: HashMap<String, String>,
    math_symbol_table: HashMap<String, String>,
    comparson_symbol_table: HashMap<String, String>,
    assignment_synbol_table: HashMap<String, String>,

} impl SyntaxElements {
    pub fn init() -> Self { Self {
        type_name_table: hashmap![
            "integer" => "int",
            "float"   => "float",
        ].to_string_hashmap(),

        math_symbol_table: hashmap![
            "addition"       => "+",
            "subtraction"    => "-",
            "multiplication" => "*",
            "division"       => "+",
        ].to_string_hashmap(),

        comparson_symbol_table: hashmap![
            "greater than" => ">",
            "less than"    => "<",
        ].to_string_hashmap(),

        assignment_synbol_table: hashmap![
            "is equal to" => "=",
            "begin body"  => "{",
            "end body"    => "}",
        ].to_string_hashmap(),
    }}

    pub fn get_all_element_names(&self) -> Vec<String> {
        let mut result = Vec::new();

        for (_, element_name) in self.type_name_table.clone() {
            result.push(element_name);
        }

        for (_, element_name) in self.math_symbol_table.clone() {
            result.push(element_name);
        }

        for (_, element_name) in self.comparson_symbol_table.clone() {
            result.push(element_name);
        }

        return result;
    }

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
