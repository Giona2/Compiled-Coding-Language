use std::collections::HashMap;


pub trait StrStrHashMapExtra {
    /// Converts a `HashMap<&str, &str>` to a `HashMap<String, String>`
    fn to_string_hashmap(&self) -> HashMap<String, String>;
} impl StrStrHashMapExtra for HashMap<&str, &str> {
    fn to_string_hashmap(&self) -> HashMap<String, String> {
        let mut result: HashMap<String, String> = HashMap::new();

        for (key, value) in self.clone() {
            result.insert(key.to_string(), value.to_string());
        }

        return result
    }
}
