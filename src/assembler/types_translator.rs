use crate::tokenizer::declaration::DataType;
use crate::tokenizer::enumerators::{Assignment, ComparisonOperator, MathOperator};
use crate::tokenizer::structures::VariableHistory;
use crate::type_traits::integer::I64Extra;
use crate::type_traits::float::F64Extra;

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
    fn to_assembly_instructions(&self, target_register: &str, variable_history: &VariableHistory) -> Result<Vec<String>, AssemblerError>;

    /// Converts either `Self::INTEGER` or `Self::FLOAT` to its acocciated assembly value
    ///
    /// Returns Err() if this is not a valid branch
    fn to_assembly_value(&self) -> Result<String, AssemblerError>;
}

impl AssignmentToAssembly for Assignment {
    fn to_assembly_instructions(&self, target_register: &str, variable_history: &VariableHistory) -> Result<Vec<String>, AssemblerError> { match self {
        Self::EVAL(first_term_assignment, operation, second_term_assignment) => { match self.evaluate_type(variable_history) {
            DataType::INTEGER => {
                // Convert the first and second terms into assembly
                let mut returned_instructions: Vec<String> = vec![
                    first_term_assignment.to_assembly_instructions("rax", variable_history).unwrap(),
                    second_term_assignment.to_assembly_instructions("rdi", variable_history).unwrap(),
                ].concat();

                // Perform the operation
                match operation {
                    MathOperator::ADD => { returned_instructions.append(&mut vec![
                        format!("  add rax, rdi")
                    ]);}
                    MathOperator::SUB => { returned_instructions.append(&mut vec![
                        format!("  sub rax, rdi")
                    ]);}
                    MathOperator::MUL => { returned_instructions.append(&mut vec![
                        format!("  imul rax, rdi")
                    ]);}
                    MathOperator::DIV => { return Err(AssemblerError::CouldNotParseEvaluation) }
                }

                // Place the result into the target register
                if target_register != "rax" { returned_instructions.append(&mut vec![
                    format!("  mov {}, rax", target_register)
                ]);}

                // Return it
                return Ok(returned_instructions)
            }
            DataType::FLOAT => {
                // Convert the first term into assembly
                let mut returned_instructions: Vec<String> = vec![
                    first_term_assignment.to_assembly_instructions("rax", variable_history).unwrap(),
                    vec![format!("  movq xmm0, rax")],
                    second_term_assignment.to_assembly_instructions("rax", variable_history).unwrap(),
                    vec![format!("  movq xmm1, rax")],
                ].concat();

                // Perform the Operation
                match operation {
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
                }

                // Place the result into the target register
                returned_instructions.append(&mut vec![
                    format!("  movq {}, xmm0", target_register)
                ]);

                // Return it
                return Ok(returned_instructions)
            }
            _ => {return Err(AssemblerError::CouldNotParseEvaluation)}
        }}

        Self::CMP(first_term_assignment, operator, second_term_assignment) => {
            // Convert first and second terms
            let mut returned_instructions: Vec<String> = vec![
                first_term_assignment.to_assembly_instructions("rdi", variable_history).unwrap(),
                second_term_assignment.to_assembly_instructions("rsi", variable_history).unwrap(),
            ].concat();

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

            // Put the result into the target register
            if target_register != "rax" { returned_instructions.append(&mut vec![
                format!("  mov {}, rax", target_register),
            ]);}

            // Return the final result
            return Ok(returned_instructions)
        }

        Self::INTEGER(returned_number) => {
            let mut returned_instructions: Vec<String> = Vec::new();

            returned_instructions.append(&mut vec![
                format!("  mov {}, {}", target_register, returned_number.to_assembly_value()),
            ]);

            return Ok(returned_instructions)
        }

        Self::FLOAT(returned_number) => {
            let mut returned_instructions: Vec<String> = Vec::new();

            returned_instructions.append(&mut vec![
                format!("  mov {}, {}", target_register, returned_number.to_assembly_value()),
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
            ]);

            // Place the result of the function into the associated register
            if target_register != "rax" { returned_instructions.append(&mut vec![
                format!("  mov {}, rax", target_register)
            ]);}

            return Ok(returned_instructions);
        }

        Self::VAR(variable_index) => {
            let mut returned_instructions: Vec<String> = Vec::new();

            returned_instructions.append(&mut vec![
                format!("  mov {}, QWORD [rbp-{}]", target_register, variable_history.step * (variable_index+1)),
            ]);

            return Ok(returned_instructions)
        }

        Self::BOOL(boolean_val) => {
            let mut returned_instructions: Vec<String> = Vec::new();

            returned_instructions.append(&mut vec![
                format!("  mov {}, {}", target_register, boolean_val.to_assembly_value()),
            ]);

            return Ok(returned_instructions)
        }

        _ => { return Err(AssemblerError::ImproperUseOfTypesTranslator) }
    }}

    fn to_assembly_value(&self) -> Result<String, AssemblerError> { match self {
        Assignment::INTEGER(returned_num) => { return Ok(returned_num.to_assembly_value()) }
        Assignment::FLOAT(returned_num)   => { return Ok(returned_num.to_assembly_value()) }
                                        _ => { return Err(AssemblerError::IncorrectAssignmentAttemptedToConvert) }
    }}
}
