use crate::lib::{environment, eval::RuntimeValType};
use std::io::{self, Write};

mod lib;
fn main() {
    let source: String = 
    "x!=3*(6-4)" .to_string();
    // let tokens = lib::lexer::tokenise(source);
    let ast = lib::parser::generate_ast(source);
    println!("{:?}", ast);
    let mut environment = lib::environment::Environment {parent: None, variables: vec![lib::environment::Variable{ name: "x".to_string(), value: lib::eval::RuntimeVal{runtime_val_type:RuntimeValType::NumericInteger(2)} }]};
    let evaluate = lib::eval::eval_program(&ast, &mut environment);
    println!("{:?}", evaluate);

    // loop {
    //     print!("> ");
    //     io::stdout().flush().unwrap();
    //     let mut input = String::new();
    //     if io::stdin().read_line(&mut input).is_err() {
    //         println!("Failed to read input.");
    //         continue;
    //     }
    //     let input = input.trim();
    //     if input == "exit" || input == "quit" {
    //         break;
    //     }
    //     let ast = lib::parser::generate_ast(input.to_string());
    //     // println!("{:?}", ast);
    //     let result = lib::eval::eval_program(&ast, &mut environment);
    //     println!("{:?}", result);
    // }
}
