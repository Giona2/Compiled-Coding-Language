The `Optimizer`'s job is to take the raw script file and convert it into a flattened list  
  
**For the remainder of this file, we will use the following example script:**  
*The inconsistent spacing is intentional*
```custom
int x= 1;
int y =2;
int result = x + y;
```
# Steps of Operation
## Flatten
The `Optimizer` will first flatten the file content. This is acheived just by replacing all newline and tab characters with spaces
___
This will return...
```custom
int x= 1; int y =2; int result = x + y;
```
___
## Space Seperation
Next, each detected symbol in held in [SyntaxElements](data.rs.md#SyntaxElements) will be proceeded and followed by a space character (if there wasn't one there already).  
This allows users to use inconsistent spacing in their scripts
___
This will return...
```custom
int x = 1; int y = 2; int result = x + y;
```
___
## Split Each Space Into a List
Then, the given content will be split into a list. You may have wondered up until this point why the compiler heavily used spaces to replaced and seperate characters. That was crucial for this step, which will use those spaces to split each word
___
This will return...
```rust
vec!["int", "x", "=", "1", ";", "int", "y", "=", "2", ";", "int", "result", "=", "x", "+", "y", ";",]
```
___
## Trim Empty Characters
Finally, the `Optimizer` will remove any empty (`''`) characters in the list  
This is primarily useful for indentation, as a newline character immediately proceeding a tab will result in two consecutive spaces, which will split and create an empty character
___
This will return...  
*The result is identical to above as no empty characters exist in this example*
```rust
vec!["int", "x", "=", "1", ";", "int", "y", "=", "2", ";", "int", "result", "=", "x", "+", "y", ";",]
```
___
## Final result
This final list will then be placed into the `content` variable for extraction
