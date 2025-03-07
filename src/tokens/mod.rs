use crate::type_traits::string_vec::StringVecExtra;


pub mod constructors;

    use constructors::*;
pub mod keyword_names;
pub mod types;
    use types::*;

#[derive(Debug)]
pub enum Token {
    FUNCTION(Function),
    TERMINATINGLOOP(TerminatingLoop),
    DECLARATION(Declaration),
}


#[derive(Debug)]
pub struct SyntaxTree {
    variables: Vec<String>,
    token_tree: Vec<Token>,

} impl SyntaxTree {
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
        let variables: Vec<String> = Vec::new();

        for (current_word_index, current_word) in optimized_file_content.iter().enumerate() {
            // Variable handling
            if keyword_names::types::get_type_names().contains(current_word) {
                let full_declaration = optimized_file_content.index_to_pattern(current_word_index, ";")
                    .unwrap();
                let equal_sign_index = full_declaration.find("=").unwrap();

                let string_assignment = full_declaration[equal_sign_index+1..full_declaration.len()-1].to_vec();
                let assignment = Assignment::from_string_vec(string_assignment);

                let declaration = Declaration::new(
                    &optimized_file_content[current_word_index+1],
                    DataType::check_token_type(current_word).unwrap(),
                    Some(assignment.unwrap()),
                );
                token_tree.push(Token::DECLARATION(declaration));
            }
        };
    
        return Self {
            variables,
            token_tree,
        }
    }

}
