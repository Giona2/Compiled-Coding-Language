The `main.rs` file is the most simplistic file in this project, as it's designed only to tell the other modules what to do, in what order, and what information it should work with.  
  
*The following steps only serve as a quick/simple explanation for each module. Please see the dedicated pages for a more in-depth explanation of how each module works*
# Steps of Operation
## The Optimizer
`main.rs` first calls the `optimizer` which formats the raw text into a formatted, flattened list that separates each important word/symbol
___
```custom
int i = 1 ;
int y = 2 ;
```
and
```custom
int i=1;
int y=2;
```
would both become
```rust
vec!["int", "i", "=", "1", ";", "int", "y", "=", "2", ";"]
```
___
## The Tokenizer
The output from the `optimizer` (the flattened, formatted list) is given to the `tokenizer` to convert the script into a series of tokens.
___
So the above would become
```txt
╭──────────────────────────────────╮    ╭───────────────────────────────────╮
│ Declaration                      │    │ Declaration                       │
│ name: i                          │    │ name: y                           │
│ location: {first slot in memory} │ -> │ location: {second slot in memory} │
│ data type: Integer               │    │ data type: Integer                │
│ value: Integer(CONST(1))         │    │ value: Integer(CONST(2))          │
╰──────────────────────────────────╯    ╰───────────────────────────────────╯
```
___

## The Assembler
Then, the token tree taken from the `tokenizer` is given to the `assembler` who converts each token into its assembly instructions.  
The output of the `assembler` is a `Vec<String>` representing each line/instruction in the outputted assembly script.

## Assembling the Outputted Assembly Script
Finally, the list of lines/instructions is used to create the final assembly file. This file will then be compiled using `NASM` and `ld` to get the final executable
