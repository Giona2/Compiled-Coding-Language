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

pub trait HashMapStrStrExtra {
    /// Converts a `HashMap<&str, &str>` to a `HashMap<String, String>`
    fn to_string(&self) -> HashMap<String, String>;
} impl HashMapStrStrExtra for HashMap<&str, &str> {
    fn to_string(&self) -> HashMap<String, String> {
        let mut result: HashMap<String, String> = HashMap::new();

        for (key, value) in self.clone() {
            result.insert(key.to_string(), value.to_string());
        }

        return result
    }
}
