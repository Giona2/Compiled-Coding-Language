use crate::hashmap;
use crate::type_traits::hashmap::StrStrHashMapExtra;

use std::collections::HashMap;


pub const MEMORY_STEP: usize = 8;


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
    pub declaration_names: HashMap<String, String>,
    pub math_symbols: HashMap<String, String>,
    pub assignment_symbols: HashMap<String, String>,
    pub comparision_symbols: HashMap<String, String>,
    pub comparision_names: HashMap<String, String>,

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
            "boolean"   => "tof",
        ].to_string_hashmap(),

        declaration_names: hashmap![
            "variable"              => "decl",
            "function"              => "subroutine",
            "return"                => "expose",
            "reassignment"          => "chng",
            "conditional statement" => "cmpr",
        ].to_string_hashmap(),

        math_symbols: hashmap![
            "addition"       => "+",
            "subtraction"    => "-",
            "multiplication" => "*",
            "division"       => "/",
        ].to_string_hashmap(),

        assignment_symbols: hashmap![
            "equals"              => "=",
            "begin body"          => ":",
            "end body"            => ";",
            "begin comparison"    => ":&[",
            "end comparison"      => "]",
            "associate with"      => "=>",
            "begin set type"      => ":-[",
            "end set type"        => "]",
            "end assignment"      => "\n",
            "begin conditions"    => "::[",
            "condition seperator" => ",",
            "end conditions"      => "]",
            "return this"         => "->",
        ].to_string_hashmap(), 

        comparision_symbols: hashmap![
            "equal to"                 => "==",
            "not equal to"             => "!=",
            "greater than"             => ">",
            "greater than or equal to" => ">=",
            "less than"                => "<",
            "less than or equal to"    => "<=",
        ].to_string_hashmap(),

        comparision_names: hashmap![
            "true"  => "true",
            "false" => "false",
        ].to_string_hashmap()
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

        for (_, element_name) in self.comparision_symbols.clone() {
            result.push(element_name);
        }

        for (_, element_name) in self.comparision_names.clone() {
            result.push(element_name);
        }

        return result;
    }

    pub fn get_all_symbols(&self) -> Vec<String> {
        let mut result = Vec::new();

        for (_, element_name) in self.math_symbols.clone() {
            result.push(element_name);
        }

        for (_, element_name) in self.assignment_symbols.clone() {
            result.push(element_name);
        }

        for (_, element_name) in self.comparision_symbols.clone() {
            result.push(element_name);
        }

        return result;
    }

    pub fn get_all_operator_symbols(&self) -> Vec<String> {
        let mut result = Vec::new();

        for (_, element_name) in self.math_symbols.clone() {
            result.push(element_name);
        }

        for (_, element_name) in self.comparision_symbols.clone() {
            result.push(element_name);
        }

        return result
    }
}
