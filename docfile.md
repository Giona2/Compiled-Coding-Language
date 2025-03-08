Well, now that you asked... This compiler is build from the ground up from technologies I build also from the ground up. Let me break down the steps the compiler will go through for generating the assembly.

# Step 1: "Optimize" written code  
I use the term "Optimize" loosely here. All optimize means is making the code consistent for more easy readability for the compiler by adding regular spaces around syntax/special/reserved characters, then converting it to a list.  
Here's an example:  
```custom
int i = 0
int i= 0
int i=0
```
will all become ["int", "i", "=", "0"]. You can probably see now how this step is crucial. Without it the compiler would get irregular spacing and weird cunjunctions making it more difficult to read

# Step 2: Parse the Optimized Code to a Syntax Tree of Sorts 
The compiler will then take the optimized code list and convert it to a representation of the processes the final assembly code will take, I call a syntax tree (though I know that probably isn't what it is.
Here's an example:
```custom
int i = 1;
int y = 2;
```
will be converted into something resembling this:
```syntax_tree
-----------------  -----------------
| Declaration   |  | Declaration   |
| location: 0   |->| location: 1   |
| type: Integer |  | type: Integer |
| value: 1      |  | value: 2      |
-----------------  -----------------
```
As stated above, the code given will be converted into this flowchart that tells the program it must first declare a variable in the first location in stack memory (variables will be placed in stack memory at an equal size) then declare a second variable in the second location in stack memory. Stack memory will grow/shrink dynamically as variables are added/dropped. In a case that a variable in the first spot in stack is dropped but the second isn't and you want to create a new variable, the new variable will occupy the same space as the previously dropped variable to save space.

# Stop 3: Convert the Generated "Syntax Tree" into assembly
This part is still a work in progress, but the compiler will take the generated tree/flowchart and convert that into the accossiated assembly code. Here's what it would be opproximately minus the memory/stack management if we set every variable to occupy 4 bytes
```asm
global _start

section .text
_start:
    push rbp
    mov rbp, rsp

    sub rsp, 4
    mov QWORD [rbp-4], 1

    sub rsp, 4
    mov QWORD [rbp-8], 2
    
    mov rsp, rbp
    pop rbp
    
    mov rax, 60
    mov rdi, 0
    syscall
```
