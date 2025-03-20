use super::enumerators::{DataType, Assignment};


#[derive(Debug, Clone)]
pub struct Declaration {
    pub name: String,
    pub location: usize,
    pub data_type: DataType,
    pub value: Option<Assignment>,
}
