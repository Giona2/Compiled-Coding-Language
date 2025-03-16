use crate::data::SyntaxElements;
use crate::type_traits::vector::StringVecExtra;


pub struct Optimizer {
    pub content: Vec<String>
} impl Optimizer{
    pub fn from_file_content(file_content: &str) -> Self {

        // Replace all newline and tab characters eith spaces
        let script_content_flattened: Vec<char> = file_content
            //.replace("\n", " ")
            .replace("\t", " ")
            .chars().collect();

        // Seperate all characters in the symbols list with spaces
        fn seperate_symbols(vector: Vec<char>, current_index: usize) -> String {
            println!("optimizer::from_file_content::seperate_symbols");
            println!("  current_index: {}", current_index);
            if current_index == vector.len() - 1 {
                return vector.iter().collect();
            }

            let all_symbols_sorted = SyntaxElements::init().get_all_symbols().sort_by_size();

            for symbol in all_symbols_sorted {
                println!("  symbol: {:?}", symbol);
                if symbol.len() > vector[current_index..].len() {
                    continue;
                }

                let chars_to_scan = vector[current_index..current_index + symbol.len()].to_vec();
                println!("  chars_to_scan: {:?}",  chars_to_scan);
                let word_to_scan: String = chars_to_scan.iter().collect();

                if symbol == word_to_scan {
                    let mut updated_vector = vector.clone();
                    updated_vector.insert(current_index, ' ');
                    updated_vector.insert(current_index + symbol.len() + 1, ' ');

                    return seperate_symbols(updated_vector, current_index + symbol.len() + 2);
                }
            }

            return seperate_symbols(vector, current_index + 1);
        }
        let script_content_seperate_symbols = seperate_symbols(script_content_flattened, 0);

        // Split by spaces
        let script_content_split = script_content_seperate_symbols.split(" ");

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
