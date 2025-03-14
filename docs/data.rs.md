`data.rs` functions as a global data file. It's meant to hold simple data (usually a `&str` or `f`/`i`/`u32` constant) every module should have access to.

# SyntaxElements
`SyntaxElements` is a `struct` that holds the names of syntax elements and keywords (`{`, `}`, `;`, `+`, etc.)  
These syntactic elements are stored in the form of `HashMap`s, where...
- the key is the descriptor/name of the syntax element
- the  value is the actual keyword the compiler needs to detect in the script

___
eg
```
"integer":  "int",
"addition": "+"
```
___
This allows other components to retrieve syntax elements by name and family  
If the syntax elements change for whatever reason, the conversion, therefore, will much more painless.

## Syntactic Element Family Structure
Each family of syntax elements has its own `HashMap`. Currently, there are four families the compiler can detect:
- `type_names`: keywords for each variable type
    - `int`, `float`
- `math_symbols`: arithmetic symbols to perform basic math
    - `+`, `-`, `/`, `*`
- `comparison_symbols` (*WORK IN PROGRESS*): symbols that return a boolean value through comparing two conditions
    - `>`, `<`
- `assignment_symbols`: symbols used in declaring variables, functions, and code bodies
    - `=`, `{`, `}`, `;`, `(`, `)`

## Associated Functions
The other modules can recieve these keywords and symbols using the associated functions like `get_type_names(/* ARGS */)`.  
The final iteration of `SyntaxElements` will have a function that retrieves each syntax element family (as a `Vec<String>`), and a function that gets every syntax element held in `SyntaxElements`  
*Currently, there only exists the function that grabs all syntax elements, and a function that grabs only type names*
