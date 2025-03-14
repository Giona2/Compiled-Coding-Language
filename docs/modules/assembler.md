The `Assembler` takes the token flowchart from the [Tokenizer](modules/tokenizer.md) and parses it into assembly instructions

# Compiled Assembly Layout
The compiler will automatically create the beginning and end to the outputted assembly. The file content written by the user will be "filled in" by the compiler
___
The outputted assembly will always start with
```asm
global _start
_start:
  push rbp
  mov rbp, rsp
```
This snippet first defines _start as the entry point to the script, then sets up the `rbp` and `rsp` registers to manage stack memory
___
___
The outputted assembly will always end with  
```asm
.exit:
  mov rsp, rbp
  pop rbp

  mov rax, 60
  mov rdi, 0
  syscall
```
This snippet creates the `_start.exit` section. `_start.exit` will return the `rbp` and `rsp` registers to their original states and call the `exit` system call
___
___
An "Empty" file would output the following x86_64 NASM ELF assembly:
```asm
global _start
_start:
  push rbp
  mov rbp, rsp
.exit:
  mov rsp, rbp
  pop rbp

  mov rax, 60
  mov rdi, 0
  syscall
```
___
