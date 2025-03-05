use super::types::DataType;


pub struct Function {
    name: String,
    return_type: DataType,
    args: Vec<String>,

} impl Function {
    pub fn new(name: &str, return_type: DataType, args: Vec<&str>) -> Self { return Self {
        name: name.to_string(),
        return_type,
        args: args.into_iter().map(|x| x.to_string()).collect(),
    }}
}
