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
    pub type_names: HashMap<String, String>,
    pub math_symbols: HashMap<String, String>,
    pub assignment_symbols: HashMap<String, String>,

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
        type_names: hashmap![
            "integer"   => "int",
            "float"     => "flt",
            "character" => "chr",
            "function"  => "subroutine",
        ].to_string_hashmap(),

        math_symbols: hashmap![
            "addition"       => "+",
            "subtraction"    => "-",
            "multiplication" => "*",
            "division"       => "+",
        ].to_string_hashmap(),

        assignment_symbols: hashmap![
            "equals"           => "=",
            "begin body"       => ":",
            "end body"         => ";",
            "end assignment"   => ";",
            "begin conditions" => "::[",
            "end conditions"   => "]",
        ].to_string_hashmap(),
    }}

    /// Gets every value held in every HashMap as a Vec<String>
    pub fn get_all_elements(&self) -> Vec<String> {
        let mut result = Vec::new();

        for (_, element_name) in self.type_names.clone() {
            result.push(element_name);
        }

        for (_, element_name) in self.math_symbols.clone() {
            result.push(element_name);
        }

        for (_, element_name) in self.assignment_symbols.clone() {
            result.push(element_name);
        }

        return result;
    }

    /// Gets every value held in every HashMap as a Vec<String> of a given size/len
    pub fn get_all_elements_of_size(&self, size: usize) -> Vec<String> {
        let mut result = Vec::new();

        let all_elements = self.get_all_elements();

        for element in all_elements {
            let element_chars: Vec<char> = element.chars().collect();

            if element_chars.len() == size { result.push(element); }
        }

        return result;
    }

    /// Gets the length of the largest value held in every HashMap
    pub fn get_size_of_largest(&self) -> usize {
        let mut result = 0;

        let all_elements = self.get_all_elements();

        for element in all_elements {
            let element_chars: Vec<char> = element.chars().collect();

            if element_chars.len() > result { result = element_chars.len() }
        }

        return result
    }

    /// Checks if `character` is the first `char` in a multicharacter element
    ///
    /// If true, `is_start_of_element()` will also return the size of the character it's a part of
    pub fn is_start_of_element(&self, character: char) -> (bool, Option<usize>) {
        let result = false;

        todo!()
    }

    /// Gets every value stored in the type_names table
    pub fn get_type_names(&self) -> Vec<String> {
        let mut result = Vec::new();

        for (_, element_name) in self.type_names.clone() {
            result.push(element_name);
        }

        return result
    }

    /// Gets every value stored in the math_symbols table
    pub fn get_math_symbols(&self) -> Vec<String> {
        let mut result = Vec::new();

        for (_, element_name) in self.math_symbols.clone() {
            result.push(element_name);
        }

        return result
    }
}
