use std::collections::HashMap;


#[macro_export]
macro_rules! hashmap {
    ($($key:expr => $val:expr),* $(,)?) => {{
        let mut result = std::collections::HashMap::new();
        $(
            result.insert($key, $val);
        )*
        result
    }};
}


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


pub trait StringStringHashMapExtra {
    /// Gets every value held in this HashMap as a Vec<String> of a given size/len
    fn values_of_size(&self, size: usize) -> Vec<String>;

    /// Gets the length of the longest string in this `HashMap` values
    fn len_of_largest(&self) -> usize;

} impl StringStringHashMapExtra for HashMap<String, String> {
    fn values_of_size(&self, size: usize) -> Vec<String> {
        let mut result = Vec::new();

        for value in self.values() {
            let value_chars: Vec<char> = value.chars().collect();

            if value_chars.len() == size { result.push(value.to_owned()); }
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


pub trait HashMapExtra<T: Eq> {
    /// Checks this hashmap to see if it contains `target`
    fn contains_value(&self, target: &T) -> bool;

    /// Checks this hashmap to see if it contains a pattern in 'targets'
    fn contains_value_from_vec(&self, targets: &[T]) -> bool;

} impl<T: Eq> HashMapExtra<T> for HashMap<String, T> {
    fn contains_value(&self, target: &T) -> bool {
        let mut result = false;

        for value in self.values() {
            if value == target { result = true; break }
        }

        return result
    }

    fn contains_value_from_vec(&self, targets: &[T]) -> bool {
        let mut result = false;

        let mut match_is_found = false;
        for value in self.values() {
            for target in targets {
                if value == target { result = true; match_is_found = true; break }
            }

            if match_is_found { break }
        }

        return result
    }
}
