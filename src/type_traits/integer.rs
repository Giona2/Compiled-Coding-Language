pub trait I64Extra {
    /// Converts this number into a string, where the returned string is formatted to cooperate
    /// with the `NASM` assembler
    fn to_assembly_value(&self) -> String;

} impl I64Extra for i64 {
    fn to_assembly_value(&self) -> String {
        return self.to_string()
    }
}
