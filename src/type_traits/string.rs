pub trait StringExtra {
    /// Returns true if this number can be converted into a `float`
    fn is_float(&self) -> bool;

    /// Returns true if this number can be converted into an `integer`
    fn is_integer(&self) -> bool;
}

impl StringExtra for String {
    fn is_float(&self) -> bool {
        if let Err(_) = self.parse::<f64>(){
            return false
        }
        return true
    }

    fn is_integer(&self) -> bool {
        if let Err(_) = self.parse::<i64>(){
            return false
        }
        return true
    }
}
