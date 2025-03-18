use std::fs;
use std::process;


mod tokenizer;
    use tokenizer::Tokenizer;

mod assembler;
    use assembler::Assembler;

mod optimizer;
    use optimizer::Optimizer;

#[allow(dead_code)]
mod type_traits;

#[allow(dead_code)]
mod data;


fn main() {
    // Read from file and flatten it
    let file_content: String = fs::read_to_string("./examples/main.uml")
        .expect("Failed to read file");
    let mut optimizer = Optimizer::init();
    optimizer.generate_optimized_content(&file_content);

    // Tokenize the flattened content
    let mut tokenizer = Tokenizer::init(8);
    tokenizer.create_token_tree(&optimizer.content);
    println!("Token Tree: {:?}", tokenizer.token_tree);

    // Essemble the generated token tree
    let mut assembler = Assembler::init();
    assembler.generate_instructions(&tokenizer.token_tree, &tokenizer.stack_memory).unwrap();
    
    // Write the assembled content to a file
    let program_content = assembler.instructions.join("\n");
    fs::write("./output.asm", program_content)
        .expect("Failed to write a.asm");

    // Assemble (using nasm/ld) the final assembly file
    // Note this is compiled in debug mode
    process::Command::new("nasm")
        .args(["-f", "elf64"])
        .args(["-g"])
        .args(["-F", "dwarf"])
        .args(["output.asm"])
        .args(["-o", "a.o"])
        .status().unwrap();
    process::Command::new("ld")
        .args(["a.o"])
        .status().unwrap();

    // Clean up extra files
    fs::remove_file("a.o")
        .expect("Failed to remove a.o");
    //fs::remove_file("output.asm")
    //    .expect("Failed to remove output.asm");
}

#[cfg(test)]
mod testing {
    use crate::optimizer::Optimizer;
    use crate::tokenizer::Tokenizer;
    use std::fs;

    #[test]
    fn test() {
        // Read from file and flatten it
        let file_content: String = fs::read_to_string("./examples/main.uml")
            .expect("Failed to read file");
        let mut optimizer = Optimizer::init();
        optimizer.generate_optimized_content(&file_content);

        // Tokenize the flattened content
        let mut tokenizer = Tokenizer::init(8);
        tokenizer.create_token_tree(&optimizer.content);
        println!("Token Tree: {:?}", tokenizer.token_tree);
    }
}
