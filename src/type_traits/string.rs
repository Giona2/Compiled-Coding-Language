pub trait StringExtra {
    /// Check this string to see if it can be converted into a number
    ///
    /// # Examples
    ///
    /// ```rust
    /// let str_1: String = "13".to_string()
    /// let str_2: String = "hello".to_string()
    ///
    /// assert!(str_1.is_number())  // Passes
    /// assert!(str_2.is_number())  // Panics
    /// ```
    fn is_number(&self) -> bool;
}

impl StringExtra for String {
    fn is_number(&self) -> bool {
        if let Err(_) = self.parse::<f64>(){
            return false
        }
        if let Err(_) = self.parse::<i64>() {
            return false
        }
        return true
    }
}
