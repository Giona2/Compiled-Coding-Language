use crate::tokenizer::enumerators::{Assignment, Operator};
use crate::tokenizer::structures::VariableHistory;
use crate::type_traits::integer::I64Extra;
use crate::type_traits::float::F64Extra;
use crate::type_traits::vector::VecExtra;

use super::data::FUNCTION_ARGUMENT_REGISTERS;
use super::error::AssemblerError;


pub trait AssignmentToAssembly {
    /// Converts this assignment to a sequence of assembly instructions
    ///
    /// All equations will be conducted using and end up in the `rdi` register, if you need to
    /// assign the result to a variable, it will be heald in the `rdi` register
    ///
    /// Note that the rax register is reserved for performing arithmetic with the result of this
    /// function
    fn to_assembly_instructions(&self, variable_history: &VariableHistory) -> Vec<String>;

    /// Converts either `Self::INTEGER` or `Self::FLOAT` to its acocciated assembly value
    ///
    /// Returns Err() if this is not a valid branch
    fn to_assembly_value(&self) -> Result<String, AssemblerError>;
}

impl AssignmentToAssembly for Assignment {
    fn to_assembly_instructions(&self, variable_history: &VariableHistory) -> Vec<String> { match self {
        Self::EVAL(first_term_raw, operation, second_term_raw) => {
            let mut returned_instructions: Vec<String> = Vec::new();

            let first_term  = first_term_raw.to_assembly_instructions(variable_history);
            let second_term = second_term_raw.to_assembly_instructions(variable_history);

            returned_instructions.append_immut(&first_term);
            returned_instructions.append(&mut vec![
                format!("  mov rax, rdi")
            ]);
            returned_instructions.append_immut(&second_term);

            match operation {
                Operator::ADD => { returned_instructions.append(&mut vec![
                    format!("  add rax, rdi")
                ]);}
                Operator::SUB => { returned_instructions.append(&mut vec![
                    format!("  sub rax, rdi")
                ]);}
                Operator::MUL => { returned_instructions.append(&mut vec![
                    format!("  imul rax, rdi")
                ]);}
                _ => { panic!("Not yet implemented") }
            }

            returned_instructions.append(&mut vec![
                format!("  mov rdi, rax")
            ]);

            return returned_instructions
        }

        Self::INTEGER(returned_number) => {
            let mut returned_instructions: Vec<String> = Vec::new();

            returned_instructions.append(&mut vec![
                format!("  mov rdi, {}", returned_number.to_assembly_value()),
            ]);

            return returned_instructions
        }

        Self::FLOAT(returned_number) => {
            let mut returned_instructions: Vec<String> = Vec::new();

            returned_instructions.append(&mut vec![
                format!("  mov rdi, {}", returned_number.to_assembly_value()),
            ]);

            return returned_instructions
        }

        Self::FUNC(function_name, function_args) => {
            let mut returned_instructions: Vec<String> = Vec::new();

            // Write the function arguments
            for (argument_index, argument) in function_args.iter().enumerate() { returned_instructions.append(&mut vec![
                format!("  mov {}, {}", FUNCTION_ARGUMENT_REGISTERS[argument_index], argument.to_assembly_value().unwrap()),
            ])};

            // Call the function
            returned_instructions.append(&mut vec![
                format!("  call {}", function_name),
                format!("  mov rdi, rax")
            ]);

            return returned_instructions;
        }

        Self::VAR(variable_index) => {
            let mut returned_instructions: Vec<String> = Vec::new();

            returned_instructions.append(&mut vec![
                format!("  mov rdi, QWORD [rbp-{}]", variable_history.step * (variable_index+1)),
            ]);

            return returned_instructions
        }
    }}

    fn to_assembly_value(&self) -> Result<String, AssemblerError> { match self {
        Assignment::INTEGER(returned_num) => { return Ok(returned_num.to_assembly_value()) }
        Assignment::FLOAT(returned_num)   => { return Ok(returned_num.to_assembly_value()) }
                                        _ => { return Err(AssemblerError::IncorrectAssignmentAttemptedToConvert) }
    }}
}

/*
impl AssignmentToAssembly for IntegerAssignment {
    fn to_assembly(&self, variable_history: &VariableHistory) -> Vec<String> { match self {
        Self::ADD(term_1, term_2)    => {
            let term_1_value = term_1.term_to_assembly_value(variable_history).unwrap();
            let term_2_value = term_2.term_to_assembly_value(variable_history).unwrap();

            return vec![
                format!("  mov rax, {}", term_1_value),
                format!("  add rax, {}", term_2_value),
            ]
        }
        Self::SUB(term_1, term_2)    => {
            let term_1_value = term_1.term_to_assembly_value(variable_history).unwrap();
            let term_2_value = term_2.term_to_assembly_value(variable_history).unwrap();

            return vec![
                format!("  mov rax, {}", term_1_value),
                format!("  sub rax, {}", term_2_value),
            ]
        }
        Self::MUL(term_1, term_2)    => {
            let term_1_value = term_1.term_to_assembly_value(variable_history).unwrap();
            let term_2_value = term_2.term_to_assembly_value(variable_history).unwrap();

            return vec![
                format!("  mov rax, {}", term_1_value),
                format!("  imul rax, {}", term_2_value),
            ]
        }
        Self::CONST(constant)        => { return vec![
            format!("  mov rax, {}", constant),
        ].iter().map(|x| x.to_string()).collect()}
        Self::VAR(variable_location) => { return vec![
            format!("  mov rax, QWORD [rbp-{}]", variable_history.step * (variable_location+1)),
        ].iter().map(|x| x.to_string()).collect()}
    }}

    fn term_to_assembly_value(&self, variable_history: &VariableHistory) -> Result<String, AssemblerError> { match self {
        IntegerAssignment::CONST(constant)        => { Ok(constant.to_string()) }
        IntegerAssignment::VAR(variable_location) => { Ok(format!("QWORD [rbp-{}]", variable_history.step * (variable_location+1))) }
                                         _ => { Err(AssemblerError::ValueRetrievedIsNotATerm) }
    }}
}

impl AssignmentToAssembly for FloatAssignment {
    fn to_assembly(&self, variable_history: &VariableHistory) -> Vec<String> { match self {
        Self::ADD(term_1, term_2)    => {
            let term_1_value = term_1.term_to_assembly_value(variable_history).unwrap();
            let term_2_value = term_2.term_to_assembly_value(variable_history).unwrap();

            return vec![
                format!("  mov rax, __float64__({})", term_1_value),
                format!("  movq xmm0, rax"),
                format!("  mov rax, __float64__({})", term_2_value),
                format!("  movq xmm1, rax"),
                format!("  addsd xmm0, xmm1"),
                format!("  movq rax, xmm0"),
            ]
        }
        Self::SUB(term_1, term_2)    => {
            let term_1_value = term_1.term_to_assembly_value(variable_history).unwrap();
            let term_2_value = term_2.term_to_assembly_value(variable_history).unwrap();

            return vec![
                format!("  mov rax, __float64__({})", term_1_value),
                format!("  movq xmm0, rax"),
                format!("  mov rax, __float64__({})", term_2_value),
                format!("  movq xmm1, rax"),
                format!("  subsd xmm0, xmm1"),
                format!("  movq rax, xmm0"),
            ]
        }
        Self::MUL(term_1, term_2)    => {
            let term_1_value = term_1.term_to_assembly_value(variable_history).unwrap();
            let term_2_value = term_2.term_to_assembly_value(variable_history).unwrap();

            return vec![
                format!("  mov rax, __float64__({})", term_1_value),
                format!("  movq xmm0, rax"),
                format!("  mov rax, __float64__({})", term_2_value),
                format!("  movq xmm1, rax"),
                format!("  mulsd xmm0, xmm1"),
                format!("  movq rax, xmm0"),
            ]
        }
        Self::DIV(term_1, term_2) => {
            let term_1_value = term_1.term_to_assembly_value(variable_history).unwrap();
            let term_2_value = term_2.term_to_assembly_value(variable_history).unwrap();

            return vec![
                format!("  mov rax, __float64__({})", term_1_value),
                format!("  movq xmm0, rax"),
                format!("  mov rax, __float64__({})", term_2_value),
                format!("  movq xmm1, rax"),
                format!("  divsd xmm0, xmm1"),
                format!("  movq rax, xmm0"),
            ]
        }
        Self::CONST(constant)        => { return vec![
            format!("  mov rax, __float64__({})", constant.to_assembly_value()),
        ].iter().map(|x| x.to_string()).collect()}
        Self::VAR(variable_location) => { return vec![
            format!("  mov rax, QWORD [rbp-{}]", variable_history.step * (variable_location+1)),
        ].iter().map(|x| x.to_string()).collect()}
    }}

    fn term_to_assembly_value(&self, variable_history: &VariableHistory) -> Result<String, AssemblerError> { match self {
        FloatAssignment::CONST(constant)        => { Ok(constant.to_assembly_value()) }
        FloatAssignment::VAR(variable_location) => { Ok(format!("QWORD [rbp-{}]", variable_history.step * (variable_location+1))) }
                                              _ => { Err(AssemblerError::ValueRetrievedIsNotATerm) }
    }}
}

impl AssignmentToAssembly for Assignment {
    fn to_assembly(&self, variable_history: &VariableHistory) -> Vec<String> { match self {
        Assignment::FLOAT(float_assignent)      => { return float_assignent.to_assembly(variable_history) }
        Assignment::INTEGER(integer_assignment) => { return integer_assignment.to_assembly(variable_history) }
    }}

    fn term_to_assembly_value(&self, _: &VariableHistory) -> Result<String, AssemblerError> {
        panic!("Not compatible")
    }
}
*/
