use crate::data::SyntaxElements;
use crate::tokenizer::declaration::DataType;
use crate::tokenizer::enumerators::{Assignment, ComparisonOperator, MathOperator};
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
    fn to_assembly_instructions(&self, variable_history: &VariableHistory) -> Result<Vec<String>, AssemblerError>;

    /// Converts either `Self::INTEGER` or `Self::FLOAT` to its acocciated assembly value
    ///
    /// Returns Err() if this is not a valid branch
    fn to_assembly_value(&self) -> Result<String, AssemblerError>;
}

impl AssignmentToAssembly for Assignment {
    fn to_assembly_instructions(&self, variable_history: &VariableHistory) -> Result<Vec<String>, AssemblerError> { match self {
        Self::EVAL(first_term_assignment, operation, second_term_assignment) => {
            // Initalize return variable
            let mut returned_instructions: Vec<String> = Vec::new();

            // Get the assembly instructions of the first and second terms
            let first_term  = first_term_assignment.to_assembly_instructions(variable_history).unwrap();
            let second_term = second_term_assignment.to_assembly_instructions(variable_history).unwrap();

            // Append the first term assembly instructions and suffix it by storing the resulting
            // value into the accociated register for processing 
            returned_instructions.append_immut(&first_term);
            match self.evaluate_type(variable_history) {
                DataType::INTEGER => { returned_instructions.append(&mut vec![
                    format!("  mov rax, rdi")
                ]);}
                DataType::FLOAT => { returned_instructions.append(&mut vec![
                    format!("  movq xmm0, rdi")
                ]);}
                _ => {
                    return Err(AssemblerError::CouldNotParseEvaluation)
                }
            }
            
            // Append the second term assembly instructions and suffix it by storing the resulting
            // value into the accociated register for processing 
            returned_instructions.append_immut(&second_term);
            match self.evaluate_type(variable_history) {
                DataType::INTEGER => {}
                DataType::FLOAT => { returned_instructions.append(&mut vec![
                    format!("  movq xmm1, rdi")
                ]);}
                _ => {
                    return Err(AssemblerError::CouldNotParseEvaluation)
                }
            }

            // Perform the math operation
            let type_of_self = self.evaluate_type(variable_history);
            match type_of_self {
                DataType::INTEGER => { match operation {
                    MathOperator::ADD => { returned_instructions.append(&mut vec![
                        format!("  add rax, rdi")
                    ]);}
                    MathOperator::SUB => { returned_instructions.append(&mut vec![
                        format!("  sub rax, rdi")
                    ]);}
                    MathOperator::MUL => { returned_instructions.append(&mut vec![
                        format!("  imul rax, rdi")
                    ]);}
                    MathOperator::DIV => {
                        panic!("Division not yet implemented")
                    }
                }}
                DataType::FLOAT => { match operation {
                    MathOperator::ADD => { returned_instructions.append(&mut vec![
                        format!("  addsd xmm0, xmm1")
                    ]);}
                    MathOperator::SUB => { returned_instructions.append(&mut vec![
                        format!("  subsd xmm0, xmm1")
                    ]);}
                    MathOperator::MUL => { returned_instructions.append(&mut vec![
                        format!("  mulsd xmm0, xmm1")
                    ]);}
                    MathOperator::DIV => { returned_instructions.append(&mut vec![
                        format!("  divsd xmm0, xmm1")
                    ]);}
                }}
                _ => {
                    return Err(AssemblerError::CouldNotParseEvaluation)
                }
            }

            // Prefix the math equation by storing the result into the rdi register
            match self.evaluate_type(variable_history) {
                DataType::INTEGER => { returned_instructions.append(&mut vec![
                    format!("  mov rdi, rax")
                ]); }
                DataType::FLOAT => { returned_instructions.append(&mut vec![
                    format!("  movq rdi, xmm0")
                ]);}
                _ => {
                    return Err(AssemblerError::CouldNotParseEvaluation)
                }
            }

            // Return it
            return Ok(returned_instructions)
        }

        Self::CMP(first_term_assignment, operator, second_term_assignment) => {
            let mut returned_instructions: Vec<String> = Vec::new();

            let first_term = first_term_assignment.to_assembly_instructions(variable_history).unwrap();
            let second_term = second_term_assignment.to_assembly_instructions(variable_history).unwrap();

            // Get the value of the first term and put it in rax
            returned_instructions.append_immut(&first_term);
            returned_instructions.append(&mut vec![
                format!("  push rdi"),
            ]);

            // Get the value of the second term and put it in rdi
            returned_instructions.append_immut(&second_term);
            returned_instructions.append(&mut vec![
                format!("  mov rsi, rdi"),
                format!("  pop rdi"),
            ]);

            // Run it through the associated cmp_ function to determine the result
            match operator {
                ComparisonOperator::EQ  => {returned_instructions.append(&mut vec![
                    format!("  call cmp_eq")
                ])}
                ComparisonOperator::NEQ => {returned_instructions.append(&mut vec![
                    format!("  call cmp_neq")
                ])}
                ComparisonOperator::GT  => {returned_instructions.append(&mut vec![
                    format!("  call cmp_gt")
                ])}
                ComparisonOperator::GEQ => {returned_instructions.append(&mut vec![
                    format!("  call cmp_geq")
                ])}
                ComparisonOperator::LT  => {returned_instructions.append(&mut vec![
                    format!("  call cmp_lt")
                ])}
                ComparisonOperator::LEQ => {returned_instructions.append(&mut vec![
                    format!("  call cmp_leq")
                ])}
            }

            // Put the result into rdi
            returned_instructions.append(&mut vec![
                format!("  mov rdi, rax"),
            ]);

            // Return the final result
            return Ok(returned_instructions)
        }

        Self::INTEGER(returned_number) => {
            let mut returned_instructions: Vec<String> = Vec::new();

            returned_instructions.append(&mut vec![
                format!("  mov rdi, {}", returned_number.to_assembly_value()),
            ]);

            return Ok(returned_instructions)
        }

        Self::FLOAT(returned_number) => {
            let mut returned_instructions: Vec<String> = Vec::new();

            returned_instructions.append(&mut vec![
                format!("  mov rdi, {}", returned_number.to_assembly_value()),
            ]);

            return Ok(returned_instructions)
        }

        Self::FUNC(function_name, _, function_args) => {
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

            return Ok(returned_instructions);
        }

        Self::VAR(variable_index) => {
            let mut returned_instructions: Vec<String> = Vec::new();

            returned_instructions.append(&mut vec![
                format!("  mov rdi, QWORD [rbp-{}]", variable_history.step * (variable_index+1)),
            ]);

            return Ok(returned_instructions)
        }

        Self::BOOL(boolean_val) => {
            let mut returned_instructions: Vec<String> = Vec::new();

            returned_instructions.append(&mut vec![
                format!("  mov rdi, {}", boolean_val.to_assembly_value()),
            ]);

            return Ok(returned_instructions)
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
