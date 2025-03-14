# Compiled Coding Language
This is an experimental project where I create a coding language from scratch (without the help of tools like `LLVM` or the like)

## Table of Contents
* [Overview][/#Overview]

## Overview
This is a minimalistic compiler that converts C-like syntaxing to an x86_64 NASM ELF assembly script for Linux distributions  
At the moment, it can only process variable declarations (floating point numbers `float` and integers `int`) that can be declared just like you would in C or C++
___
For example...
```custom
int x = 1;
float y = 2;
```
Would initalize the variable `x` as an integer of `1` and `y` as a floating point number of `2`
___

## Math Declarations
It can also perform basic arithmetic with numbers and stored variables with `+`, `-`, `*`, `/`
**Note that the `/` operator only works with floating point numbers. `int i = 1/2;` will return an error.**
**`float i = 1/2` will parse and execute correctly**
___
For example...
```custom
```
___
