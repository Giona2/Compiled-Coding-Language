pub trait StrVecExtra {
    /// Converts a `Vec<&str>` to a `Vec<String>`
    fn to_string_vec(&self) -> Vec<String>;
}
impl StrVecExtra for Vec<&str> {
    fn to_string_vec(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        for value in self.clone() {
            result.push(value.to_string());
        }

        return result
    }
}
