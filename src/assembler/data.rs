pub const FUNCTION_ARGUMENT_REGISTERS: [&str; 6] = ["rdi", "rsi", "rdx", "r10", "r8", "r9"];


pub mod core_utils {
    use crate::type_traits::slice::StrSlice;


    pub fn get_all() -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        // Append CMP lib
        result.append(&mut CMP.to_string_vec());

        return result
    }

    pub const CMP: &[&str] = &[
        "cmp_eq:",
        "  cmp rdi, rsi",
        "  je .eq",
        "  jmp .neq",
        ".eq:",
        "  mov rax, 1",
        "  jmp .end",
        ".neq:",
        "  mov rax, 0",
        "  jmp .end",
        ".end:",
        "  ret",
        "",
        "cmp_neq:",
        "  cmp rdi, rsi",
        "  jne .eq",
        "  jmp .neq",
        ".eq:",
        "  mov rax, 1",
        "  jmp .end",
        ".neq:",
        "  mov rax, 0",
        "  jmp .end",
        ".end:",
        "  ret",
        "cmp_gt:",
        "  cmp rdi, rsi",
        "  jg .eq",
        "  jmp .neq",
        ".eq:",
        "  mov rax, 1",
        "  jmp .end",
        ".neq:",
        "  mov rax, 0",
        "  jmp .end",
        ".end:",
        "  ret",
        "",
        "cmp_geq:",
        "  cmp rdi, rsi",
        "  jge .eq",
        "  jmp .neq",
        ".eq:",
        "  mov rax, 1",
        "  jmp .end",
        ".neq:",
        "  mov rax, 0",
        "  jmp .end",
        ".end:",
        "  ret",
        "",
        "cmp_lt:",
        "  cmp rdi, rsi",
        "  jl .eq",
        "  jmp .neq",
        ".eq:",
        "  mov rax, 1",
        "  jmp .end",
        ".neq:",
        "  mov rax, 0",
        "  jmp .end",
        ".end:",
        "  ret",
        "",
        "cmp_leq:",
        "  cmp rdi, rsi",
        "  jle .eq",
        "  jmp .neq",
        ".eq:",
        "  mov rax, 1",
        "  jmp .end",
        ".neq:",
        "  mov rax, 0",
        "  jmp .end",
        ".end:",
        "  ret",
        "",
    ];
}
