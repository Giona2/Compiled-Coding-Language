pub trait F64Extra {
    /// Converts this number into a string, where the returned string is formatted to cooperate
    /// with the `NASM` assembler
    fn to_assembly_value(&self) -> String;

} impl F64Extra for f64 {
    fn to_assembly_value(&self) -> String {
        let string_self = self.to_string();
        let integer_self: Result<i64, _> = string_self.parse();

        let result: String;

        if let Ok(integer) = integer_self {
            result = format!("__float64__({}.0)", integer.to_string());
        } else {
            result = format!("__float64__({})", string_self);
        }

        return result
    }
}
