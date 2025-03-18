use crate::data::SyntaxElements;
use crate::type_traits::vector::StringVecExtra;


pub struct Optimizer {
    pub content: Vec<String>,

    syntax_elements: SyntaxElements,

} impl Optimizer{
    pub fn init() -> Self { return Self {
        content: Vec::new(),
        syntax_elements: SyntaxElements::init(),
    }}

    pub fn generate_optimized_content(&mut self, file_content: &str) {
        // Replace all newline and tab characters eith spaces
        let script_content_flattened: Vec<char> = file_content
            //.replace("\n", " ")
            .replace("\t", " ")
            .chars().collect();

        // Seperate all characters in the symbols list with spaces
        fn seperate_symbols(syntax_elements: &SyntaxElements, vector: Vec<char>, current_index: usize) -> String {
            if current_index == vector.len() - 1 {
                return vector.iter().collect();
            }

            let all_symbols_sorted = syntax_elements.get_all_symbols().sort_by_size();

            for symbol in all_symbols_sorted {
                if symbol.len() > vector[current_index..].len() {
                    continue;
                }

                let chars_to_scan = vector[current_index..current_index + symbol.len()].to_vec();
                let word_to_scan: String = chars_to_scan.iter().collect();

                if symbol == word_to_scan {
                    let mut updated_vector = vector.clone();
                    updated_vector.insert(current_index, ' ');
                    updated_vector.insert(current_index + symbol.len() + 1, ' ');

                    return seperate_symbols(syntax_elements, updated_vector, current_index + symbol.len() + 2);
                }
            }

            return seperate_symbols(syntax_elements, vector, current_index + 1);
        }
        let script_content_seperate_symbols = seperate_symbols(&self.syntax_elements, script_content_flattened, 0);

        // Split by spaces
        let script_content_split = script_content_seperate_symbols.split(" ");

        // Remove empty chars created by multiple spaces
        let script_content_trimmed: Vec<&str> = script_content_split.into_iter()
            .filter_map(|x| if x != "" {Some(x)} else {None})
            .collect();
        
        // Store the result into content
        self.content = script_content_trimmed.iter()
            .map(|x| x.to_string())
            .collect();
    }
}
