use crate::data::SyntaxElements;
use crate::type_traits::vector::StringVecExtra;


pub struct Optimizer {
    pub content: Vec<String>
} impl Optimizer{
    pub fn from_file_content(file_content: &str) -> Self {
        let syntax_elements = SyntaxElements::init();

        // Replace all newline and tab characters eith spaces
        let script_content_flattened: Vec<char> = file_content
            .replace("\n", " ")
            .replace("\t", " ")
            .chars().collect();

        // Seperate all characters in the syntax list with spaces
        let script_content_seperate_syntax_chars = String::new();

        let all_elements_sorted = syntax_elements.get_all_elements().sort_by_size();
        for (character_index, character) in script_content_flattened.iter().enumerate() {
            // first check if char is the start to a multi-char symbol
        }

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
