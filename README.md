# Compiled Coding Language
This is an experimental project where I create a coding language from scratch (without the help of tools like `LLVM` or the like)  
If you're here to critique this repository, please see [Source code deep-dive website for anylizers](https://giona2.github.io/Compiled-Coding-Language/) for a more in-depth explination on my source code

## Table of Contents
* [Overview](#overview)
* [Variable Declaration](#variable-declaration)
* [Function Declaration](#function-declaration)

## Overview
Unimal is very simple procedural coding language that closely follows assembly in the same vane as C. Because of the way this compiler functions, most actions you write require prefixing using reserved keywords.

## Variable Declaration
If you wanted to create an integer, `i`, equal to 1, then you wanted to change the value to 2, it would translate to the following C code:
```c
int main() {
    int i = 1;
    i = 2;

    return 0;
}
```

The equivalent Unimal code would look very similar, but you'd need to declare the third line above as a variable reassignment with the keyword, `chng`. Here's an example
```unimal
subroutine main::[] -> int :
    decl i:=[int] = 1
    chng i = 2
    
    expose 0
;
```

## Function Declaration
Functions in unimal are constructed by first using the `subroutine` keyword, followed by the name, the arguments (held between `::[` and `]`), and the return type.  
*Note*: The inline function block is between `:` and `;`, where the end of a line is created using a newline character  
Say you needed a function, `add`, that returned the sum of the integers `first` and `second` then returned the result as an integer. This hypothetical situation would translate to the following  
```unimal
subroutine add::[int first, int second] -> int:
    expose first + second
;
```
