pub trait StrSlice {
    /// Converts this &[&str] into a Vec<String>
    fn to_string_vec(&self) -> Vec<String>;
} impl StrSlice for &[&str] {
    fn to_string_vec(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        for element in self.to_vec() {
            result.push(element.to_string());
        }

        return result
    }
}
