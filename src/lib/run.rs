use super::eval;
use super::environment;
use super::parser;
use std::rc::Rc;
use std::cell::RefCell;

pub fn run_script(source: String) -> eval::RuntimeVal{
    let ast = parser::generate_ast(source);
    let mut environment = Rc::new(RefCell::new(environment::Environment {parent: None, variables: vec![]}));
    let evaluate = eval::eval_program(&ast, environment);
    evaluate
}