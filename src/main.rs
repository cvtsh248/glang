use crate::lib::{environment, eval::RuntimeValType};

mod lib;
fn main() {
    let source: String = "(2*x+3)*2+6+9;" .to_string();
    // let tokens = lib::lexer::tokenise(source);
    let ast = lib::parser::generate_ast(source);
    // println!("{:?}", ast);
    let mut environment = lib::environment::Environment {parent: None, variables: vec![lib::environment::Variable{ name: "x".to_string(), value: lib::eval::RuntimeVal{runtime_val_type:RuntimeValType::NumericInteger(2)} }]};
    let evaluate = lib::eval::eval_program(&ast, &mut environment);
    println!("{:?}", evaluate);
}
