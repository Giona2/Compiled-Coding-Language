use std::collections::HashMap;


pub trait StrStrHashMapExtra {
    /// Converts a `HashMap<&str, &str>` to a `HashMap<String, String>`
    fn to_string_hashmap(&self) -> HashMap<String, String>;

    /// Gets every value held in this HashMap as a Vec<String> of a given size/len
    fn values_of_size(&self, size: usize) -> Vec<String>;

    fn len_of_largest(&self) -> usize;
} impl StrStrHashMapExtra for HashMap<&str, &str> {
    fn to_string_hashmap(&self) -> HashMap<String, String> {
        let mut result: HashMap<String, String> = HashMap::new();

        for (key, value) in self.clone() {
            result.insert(key.to_string(), value.to_string());
        }

        return result
    }

    fn values_of_size(&self, size: usize) -> Vec<String> {
        let mut result = Vec::new();

        for value in self.values() {
            let value_chars: Vec<char> = value.chars().collect();

            if value_chars.len() == size { result.push(value.to_string()); }
        }

        return result;
    }

    fn len_of_largest(&self) -> usize {
        let mut result = 0;

        for value in self.values() {
            let value_chars: Vec<char> = value.chars().collect();

            if value_chars.len() > result { result = value_chars.len() }
        }

        return result
    }
}
