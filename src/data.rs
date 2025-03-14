use crate::hashmap;
use crate::type_traits::str_str_hashmap::StrStrHashMapExtra;

use std::collections::HashMap;


/// Holds every syntactic character in the form of a HashMap
///
/// SyntaxElements holds a HashMap for each family of syntactic characters
/// - type names
/// - math operators (math symbols)
/// - assignment symbols
/// where the keys are descriptors/names indicating what the element does while the key is the
/// element itself
///
/// for example:
/// ```rust
/// hashmap![
///     "integer" => "int",
/// ]
/// ```
/// or
/// ```rust
/// hashmap![
///     "greater than" => ">",
/// ]
/// ```
pub struct SyntaxElements {
    type_name_table: HashMap<String, String>,
    math_symbol_table: HashMap<String, String>,
    comparson_symbol_table: HashMap<String, String>,
    assignment_symbol_table: HashMap<String, String>,

} impl SyntaxElements {
    /// Initialize the SyntaxElements
    ///
    /// SyntaxElements holds a HashMap for each family of syntactic characters
    /// - type names
    /// - math operators (math symbols)
    /// - assignment symbols
    /// where the keys are descriptors/names indicating what the element does while the key is the
    /// element itself
    ///
    /// for example:
    /// ```rust
    /// hashmap![
    ///     "integer" => "int",
    /// ]
    /// ```
    /// or
    /// ```rust
    /// hashmap![
    ///     "greater than" => ">",
    /// ]
    /// ```
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

        assignment_symbol_table: hashmap![
            "equals"           => "=",
            "begin body"       => "{",
            "end body"         => "}",
            "end assignment"   => ";",
            "begin conditions" => "(",
            "end conditions"   => ")",
        ].to_string_hashmap(),
    }}

    /// Gets every value held in every HashMap as a Vec<String>
    pub fn get_all_elements(&self) -> Vec<String> {
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

    /// Gets every value stored in the type_name_table table
    pub fn get_type_names(&self) -> Vec<String> {
        let mut result = Vec::new();

        for (_, element_name) in self.type_name_table.clone() {
            result.push(element_name);
        }

        return result
    }
}
