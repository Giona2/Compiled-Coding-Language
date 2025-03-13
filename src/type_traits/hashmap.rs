#[macro_export]
macro_rules! hashmap {
    ($($key:expr => $val:expr),*) => {{
        let mut result = std::collections::Hashmap::new();
        $(
            result.insert($key, $val)
        )*
        return result
    }};
}
