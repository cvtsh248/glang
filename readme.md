# Glang
This is an interpreted language written in Rust that I've been writing for fun. I started off by watching the first 3 videos of Tyler Laceby's "Build a custom scripting language in typescript" videos (https://www.youtube.com/playlist?list=PL_2VhOvlMk4UHGqYCLWc6GO8FaPl8fQTh) and then set off from there. 

### Project Structure
#### Lexer (lexer.rs)
* Tokenises source code into tokens
* Supports integers, booleans, floats, strings, identifiers, operators and keywords
* Currently supported operators include `+`,`-`,`*`,`/`,`%`,`=`,`==`,`!=`,`<`,`<=`,`>`,`>=`,`&&`,`||`,`!`
* Currently reserved keywords include `let`, `true`, `false`, `if`, `loop`, `print`
#### Parser (parser.rs)
* Generates an abstract syntax tree from the array of tokens generated in `lexer.rs`
* Supports expressions, declerations, loops, conditionals, scoped blocks and assignments
* AST node types currently include: `Program, NumericLiteral, StringLiteral, Boolean, Identifier, BinaryExpr, UnaryExpr, Assignment, Declaration, Scope, Loop, If, Print, EOL`
#### Environment (environment.rs)
* Manages variable storage and scoping
* Functions include variable declaration, assignment, lookup and environment resolution
* Utilises Rc and RefCell for shared ownership and mutability of references
#### Evaluation (eval.rs)
* Interprets the AST and executes the program, including arithmetic, comparison, logical operations and limited unary operations
* Handles control flow and scopes
#### Runtime (run.rs)
* Entry point that does lexing and parsing
#### Main (main.rs)
* Reads .glang files and runs them

### Current features
#### Supported data types
* Integers
* Floats
* Booleans (i.e. `true, false`)
* Strings

#### Supported operators
* Arithmetic (i.e. `+,-,*,/,%`)
* Comparison (i.e. `==,!=,<,<=,>,>=`)
* Logical (i.e. `&&,||,!`)

#### Control flow
* Variable declaration: `let x = 10;`
* Loops: `loop (condition) { ... }`
* Loop breaks: `loop (condition) { ... if (condition) {break;}}`
* Conditionals: `if (condition) { ... } elif (condition) { ... } else { ... }`, 
* Scopes (creastes new environment): `{ ... }`
* Print: `print(expression);`

### Example programs
Examples can be found in `demo_scripts`

### Running scripts
`cargo run path_to_file.glang`

### Todo
Once I complete the following I'll probably no longer touch the project:
* Break loops
* Arrays
* Functions
* Structs (or something of the sort)