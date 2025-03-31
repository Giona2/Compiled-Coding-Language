use std::fmt::{self, format, Display};

use crate::tokenizer::declaration::Declaration;

use super::enumerators::Assignment;
use super::{Token, Tokenizer};
use super::declaration::DataType;
use super::structures::{Variable, VariableHistory};




impl fmt::Display for Tokenizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::from("Tokenizer\n");

        fn construct_result(result: &mut String, current_token_vec: &Vec<Token>, current_indent: usize) {
            let indent = "  ".repeat(current_indent);

            for token in current_token_vec { match token {
                Token::Function(function) => { 
                    let name             = &function.name;
                    let return_type      = &function.return_type;
                    let arguments        = DisplayVecWrapper(function.arguments.clone());
                    let variable_history = &function.variable_history;

                    *result += &format!("{indent}function\n");
                    *result += &format!("{indent}|- name: {name}");
                    *result += &format!("{indent}|- return_type: {return_type}\n");
                    *result += &format!("{indent}|- arguments: {arguments}\n");
                    *result += &format!("{indent}|- variable_history: {variable_history}\n");
                    construct_result(result, &function.functionaliy, current_indent+1) }
                Token::ConditionalStatement(conditional_statement) => {
                    *result += &format!("{indent}ConditionalStatement\n");
                    construct_result(result, &conditional_statement.functionality, current_indent+1);
                }
                Token::Declaration(declaration) => {
                    let name = &declaration.name;
                    let location = &declaration.location;
                    let data_type = &declaration.data_type;
                    let value = &declaration.value;

                    *result += &format!("{indent}Declaration");
                    *result += &format!("{indent}|- name: {name}");
                    *result += &format!("{indent}|- location: {location}");
                    *result += &format!("{indent}|- data_type: {data_type}");
                    *result += &format!("{indent}|- value: {value}");
                }
                _ => {
                    panic!("You forgot to implement display for this token, dumbass")
                }
            }}
        }
        construct_result(&mut result, &self.token_tree, 0);

        write!(f, "{}", result)
    }
}

impl fmt::Display for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { match self {
        Self::CMP(first_assignment, , )
    }}
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        match self {
            DataType::INTEGER => { result += "DataType::INTEGER" }
            DataType::FLOAT   => { result += "DataType::FLOAT"   }
            DataType::BOOL    => { result += "DataType::BOOL"    }
        }

        write!(f, "{}", result)
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = &self.name;
        let data_type = &self.data_type;

        write!(f, "Variable(name: {name}, data_type: {data_type})")
    }
}

impl fmt::Display for VariableHistory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = DisplayVecOptionWrapper::from(self.data.clone());
        let step = &self.step;

        write!(f, "VariableHistory(data: {data}, step: {step}")
    }
}

struct DisplayOptionWrapper<T: Display>(Option<T>);
impl<T: fmt::Display> fmt::Display for DisplayOptionWrapper<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { match &self.0  {
        Some(inner) => { write!(f, "Some({inner})") }
        None        => { write!(f, "None")          }
    }}
}

struct DisplayVecOptionWrapper<T: fmt::Display>(Vec<DisplayOptionWrapper<T>>);
impl<T: Display> DisplayVecOptionWrapper<T> {
    pub fn from(from: Vec<Option<T>>) -> Self {
        let mut result = Self ( Vec::new() );

        for element in from {
            result.0.push(DisplayOptionWrapper(element));
        }

        return result
    }
}
impl<T: fmt::Display> fmt::Display for DisplayVecOptionWrapper<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::from("[");

        for element in self.0.iter() { match &element.0 {
            Some(inner) => { result += &format!("Some({inner}), ") }
            None        => { result += &format!("None, ") }
        }}

        write!(f, "{}", result)
    }
}

struct DisplayVecWrapper<T: fmt::Display>(Vec<T>);
impl<T: fmt::Display> fmt::Display for DisplayVecWrapper<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::from("[");

        for element in self.0.iter() {
            result += &format!("{element}, ")
        }

        write!(f, "{}", result)
    }
}
