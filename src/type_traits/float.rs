pub trait F64Extra {
    fn to_assembly_value(&self) -> String;

} impl F64Extra for f64 {
    fn to_assembly_value(&self) -> String {
        let string_self = self.to_string();
        let integer_self: Result<i64, _> = string_self.parse();

        let result: String;

        if let Ok(integer) = integer_self {
            result = integer.to_string() + ".0";
        } else {
            result = string_self;
        }

        return result
    }
}
