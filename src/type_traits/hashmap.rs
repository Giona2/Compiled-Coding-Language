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

