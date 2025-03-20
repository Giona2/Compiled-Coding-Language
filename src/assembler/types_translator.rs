use crate::tokenizer::enumerators::{Assignment, FloatAssignment, IntegerAssignment};
use crate::tokenizer::structures::VariableHistory;
use crate::type_traits::float::F64Extra;

use super::error::AssemblerError;

pub trait AssignmentToAssembly {
    /// Converts this assignment to a sequence of assembly instructions
    /// 
    /// If the assignment is a calculation, all calculations will be performed by first pushing the using the call
    /// register (`rax`, for example), clearing it, and performing all calculations using that freed register
    ///
    /// Note that after you assign the final calculation to a variable you should to return the
    /// original value of the used register to minimize risk of segmentation faults and other
    /// miscellaneous bugs/errors (`pop rax`, for example)
    ///
    /// # Examples
    ///
    /// ## Equations with constants
    /// ```rust
    /// let assignment = Assignment.from_string_vec(/* Stack Memory Obj */, vec![
    ///     "3", "+", "2"
    /// ].into_iter().map(|x| x.to_string()).collect())
    ///
    /// let assembly_equivalent = assignment.to_assembly(/* Stack Memory Obj */);
    /// 
    /// // Result
    /// // ---
    /// // push rax
    /// // mov rax, 3
    /// // add rax, 2
    /// // ---
    /// println!("{assembly_equivalent:?}");
    /// ```
    ///
    /// ## Equations with variables
    /// ```rust
    /// let stack_step = 8;
    /// let mut stack = VariableHistory.init(stack_step);
    /// stack.add_variable("x", /* Data Type*/);
    /// stack.add_variable("y", /* Data Type*/);
    ///
    /// let assignment = Assignment.from_string_vec(stack, vec![
    ///     "x", "+", "y"
    /// ].into_iter().map(|x| x.to_string()).collect());
    ///
    /// let assembly_equivalent = assignment.to_assembly(stack);
    /// 
    /// // Result (the comments will not be included in the result)
    /// // ---
    /// // push rax
    /// // ; X is held in the first register slot
    /// // mov rax, [rbp-{stack_step * 1}]
    /// // ; Y is held in the second register slot
    /// // add rax, [rbp-{stack_step * 2}]
    /// // ---
    /// println!("{assembly_equivalent:?}");
    /// ```
    /// 
    /// ## Single assignments
    /// ```rust
    /// let assignment = Assignment.from_string_vec(/* Stack Memory Obj */, vec![
    ///     "3", "+", "2"
    /// ].into_iter().map(|x| x.to_string()).collect())
    ///
    /// let assembly_equivalent = assignment.to_assembly(/* Stack Memory Obj */);
    /// 
    /// // Result
    /// // ---
    /// // push rax
    /// // mov rax, 3
    /// // add rax, 2
    /// // ---
    /// println!("{assembly_equivalent:?}");
    /// ```
    fn to_assembly(&self, stack_memory: &VariableHistory) -> Vec<String>;

    /// Converts either CONST() or VAR() to its assembly value
    fn term_to_assembly_value(&self, stack_memory: &VariableHistory) -> Result<String, AssemblerError>;
}


impl AssignmentToAssembly for IntegerAssignment {
    fn to_assembly(&self, stack_memory: &VariableHistory) -> Vec<String> { match self {
        Self::ADD(term_1, term_2)    => {
            let term_1_value = term_1.term_to_assembly_value(stack_memory).unwrap();
            let term_2_value = term_2.term_to_assembly_value(stack_memory).unwrap();

            return vec![
                format!("  mov rax, {}", term_1_value),
                format!("  add rax, {}", term_2_value),
            ]
        }
        Self::SUB(term_1, term_2)    => {
            let term_1_value = term_1.term_to_assembly_value(stack_memory).unwrap();
            let term_2_value = term_2.term_to_assembly_value(stack_memory).unwrap();

            return vec![
                format!("  mov rax, {}", term_1_value),
                format!("  sub rax, {}", term_2_value),
            ]
        }
        Self::MUL(term_1, term_2)    => {
            let term_1_value = term_1.term_to_assembly_value(stack_memory).unwrap();
            let term_2_value = term_2.term_to_assembly_value(stack_memory).unwrap();

            return vec![
                format!("  mov rax, {}", term_1_value),
                format!("  imul rax, {}", term_2_value),
            ]
        }
        Self::CONST(constant)        => { return vec![
            format!("  mov rax, {}", constant),
        ].iter().map(|x| x.to_string()).collect()}
        Self::VAR(variable_location) => { return vec![
            format!("  mov rax, QWORD [rbp-{}]", stack_memory.step * (variable_location+1)),
        ].iter().map(|x| x.to_string()).collect()}
    }}

    fn term_to_assembly_value(&self, stack_memory: &VariableHistory) -> Result<String, AssemblerError> { match self {
        IntegerAssignment::CONST(constant)        => { Ok(constant.to_string()) }
        IntegerAssignment::VAR(variable_location) => { Ok(format!("QWORD [rbp-{}]", stack_memory.step * (variable_location+1))) }
                                         _ => { Err(AssemblerError::ValueRetrievedIsNotATerm) }
    }}
}

impl AssignmentToAssembly for FloatAssignment {
    fn to_assembly(&self, stack_memory: &VariableHistory) -> Vec<String> { match self {
        Self::ADD(term_1, term_2)    => {
            let term_1_value = term_1.term_to_assembly_value(stack_memory).unwrap();
            let term_2_value = term_2.term_to_assembly_value(stack_memory).unwrap();

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
            let term_1_value = term_1.term_to_assembly_value(stack_memory).unwrap();
            let term_2_value = term_2.term_to_assembly_value(stack_memory).unwrap();

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
            let term_1_value = term_1.term_to_assembly_value(stack_memory).unwrap();
            let term_2_value = term_2.term_to_assembly_value(stack_memory).unwrap();

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
            let term_1_value = term_1.term_to_assembly_value(stack_memory).unwrap();
            let term_2_value = term_2.term_to_assembly_value(stack_memory).unwrap();

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
            format!("  mov rax, QWORD [rbp-{}]", stack_memory.step * (variable_location+1)),
        ].iter().map(|x| x.to_string()).collect()}
    }}

    fn term_to_assembly_value(&self, stack_memory: &VariableHistory) -> Result<String, AssemblerError> { match self {
        FloatAssignment::CONST(constant)        => { Ok(constant.to_assembly_value()) }
        FloatAssignment::VAR(variable_location) => { Ok(format!("QWORD [rbp-{}]", stack_memory.step * (variable_location+1))) }
                                              _ => { Err(AssemblerError::ValueRetrievedIsNotATerm) }
    }}
}

impl AssignmentToAssembly for Assignment {
    fn to_assembly(&self, stack_memory: &VariableHistory) -> Vec<String> { match self {
        Assignment::FLOAT(float_assignent)      => { return float_assignent.to_assembly(stack_memory) }
        Assignment::INTEGER(integer_assignment) => { return integer_assignment.to_assembly(stack_memory) }
    }}

    fn term_to_assembly_value(&self, _: &VariableHistory) -> Result<String, AssemblerError> {
        panic!("Not compatible")
    }
}
