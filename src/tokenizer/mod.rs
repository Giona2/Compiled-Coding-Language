use crate::type_traits::string_vec::StringVecExtra;


#[allow(dead_code)]
pub mod representations;
    use representations::StackMemory;

#[allow(dead_code)]
pub mod constructors;
    use constructors::*;

#[allow(dead_code)]
pub mod keyword_names;

#[allow(dead_code)]
#[allow(unused_macros)]
pub mod types;
    use types::*;

#[allow(dead_code)]
pub mod error;


#[derive(Debug)]
#[allow(dead_code)]
pub enum Token {
    FUNCTION(Function),
    TERMINATINGLOOP(TerminatingLoop),
    DECLARATION(Declaration),
}


#[derive(Debug)]
pub struct TokenTree {
    pub token_tree: Vec<Token>,
    pub stack_memory: StackMemory,

} impl TokenTree {
    pub fn optimize_file_content(file_content: &str) -> Vec<String> {
        // Replace all newline and tab characters eith spaces
        let script_content_flattened: Vec<char> = file_content
            .replace("\n", " ")
            .replace("\t", " ")
            .chars().collect();

        // Seperate all characters in the syntax list with spaces
        fn get_script_content_seperate_syntax_chars(script_content_flattened: Vec<char>) -> Vec<char> {
            let mut result = Vec::new();

            for (character_index, character) in script_content_flattened.iter().enumerate() {
                // If the character is folowed/proceeded by a space, place a space before and after
                // it
                if keyword_names::get_syntax_chars().contains(&character.to_string()) && script_content_flattened[character_index-1] != ' ' {
                    let updated_script_content = vec![script_content_flattened[..character_index].to_vec(), [' '].to_vec(), [character.clone()].to_vec(), [' '].to_vec(), script_content_flattened[character_index+1..].to_vec()].concat();
                    result = get_script_content_seperate_syntax_chars(updated_script_content);
                    break;
                }
                else if keyword_names::get_syntax_chars().contains(&character.to_string()) && script_content_flattened[character_index+1] != ' ' {
                    let updated_script_content = vec![script_content_flattened[..character_index].to_vec(), [' '].to_vec(), [character.clone()].to_vec(), [' '].to_vec(), script_content_flattened[character_index+1..].to_vec()].concat();
                    result = get_script_content_seperate_syntax_chars(updated_script_content);
                    break;
                }
                // If the loop gets to the end without spotting above, finally return it
                else if character_index == script_content_flattened.len()-1 {
                    result = script_content_flattened.clone();
                }
            }
            return result;
        }
        let script_content_seperate_syntax_chars = get_script_content_seperate_syntax_chars(script_content_flattened)
            .iter().collect::<String>();

        // Split by spaces
        let script_content_split = script_content_seperate_syntax_chars.split(" ");

        // Remove empty chars created by multiple spaces
        let script_content_trimmed: Vec<&str> = script_content_split.into_iter()
            .filter_map(|x| if x != "" {Some(x)} else {None})
            .collect();
        
        // Return the result
        return script_content_trimmed.iter()
            .map(|x| x.to_string())
            .collect();
    }

    pub fn from_file_content(optimized_file_content: Vec<String>) -> Self {
        let mut token_tree: Vec<Token> = Vec::new();
        let mut stack_memory = StackMemory::init(8);

        for (current_word_index, current_word) in optimized_file_content.iter().enumerate() {
            // Variable handling
            if keyword_names::types::get_type_names().contains(current_word) {
                let full_declaration = optimized_file_content.index_to_pattern(current_word_index, ";")
                    .unwrap();
                let equal_sign_index = full_declaration.find("=").unwrap();

                let string_assignment = full_declaration[equal_sign_index+1..full_declaration.len()-1].to_vec();

                let name = optimized_file_content[current_word_index+1].clone();
                let data_type = DataType::check_token_type(current_word).unwrap();
                let assignment = Assignment::from_string_vec(&stack_memory, string_assignment).unwrap();

                let declaration = Declaration {
                    name: name.to_string(),
                    data_type: data_type.clone(),
                    value: Some(assignment),
                };

                stack_memory.add_variable(&name, data_type)
                    .expect("stack_memory does not conclude with None");

                token_tree.push(Token::DECLARATION(declaration));
            }
        };
    
        return Self {
            stack_memory,
            token_tree,
        }
    }
}
