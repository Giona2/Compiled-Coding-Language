use super::enumerators::Assignment;


#[derive(Debug, Clone)]
pub struct Reassignment {
    pub name: String,
    pub new_assignment: Assignment,
} 
