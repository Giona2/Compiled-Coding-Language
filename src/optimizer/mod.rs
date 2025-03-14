use crate::data::SyntaxElements;


pub struct Optimizer {
    pub content: Vec<String>
} impl Optimizer{
    pub fn from_file_content(file_content: &str) -> Self {
        // Replace all newline and tab characters eith spaces
        let script_content_flattened: Vec<char> = file_content
            .replace("\n", " ")
            .replace("\t", " ")
            .chars().collect();

        // Seperate all characters in the syntax list with spaces
        fn get_script_content_seperate_syntax_chars(script_content_flattened: Vec<char>) -> Vec<char> {
            let mut result = Vec::new();

            for (character_index, character) in script_content_flattened.iter().enumerate() {
                let syntax_elements = SyntaxElements::init();

                // If the character is followed/proceeded by a space, place a space before and after
                // it
                if syntax_elements.get_all_elements().contains(&character.to_string()) && script_content_flattened[character_index-1] != ' ' {
                    let updated_script_content = vec![
                        script_content_flattened[..character_index].to_vec(), 
                        [' '].to_vec(), 
                        [character.clone()].to_vec(), [' '].to_vec(), 
                        [' '].to_vec(),
                        script_content_flattened[character_index+1..].to_vec()
                    ].concat();

                    result = get_script_content_seperate_syntax_chars(updated_script_content);
                    break;
                }
                else if syntax_elements.get_all_elements().contains(&character.to_string()) && script_content_flattened[character_index+1] != ' ' {
                    let updated_script_content = vec![
                        script_content_flattened[..character_index].to_vec(),
                        [' '].to_vec(),
                        [character.clone()].to_vec(),
                        [' '].to_vec(),
                        script_content_flattened[character_index+1..].to_vec()
                    ].concat();

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
        return Self {
            content: script_content_trimmed.iter()
                .map(|x| x.to_string())
                .collect(),
        }
    }
}
