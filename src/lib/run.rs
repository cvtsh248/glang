use super::eval;
use super::environment;
use super::parser;

pub fn run_script(source: String) -> eval::RuntimeVal{
    let ast = parser::generate_ast(source);
    let mut environments_array: Vec<&mut environment::Environment> = vec![];
    let mut environment = environment::Environment {parent: None, variables: vec![]};
    let evaluate = eval::eval_program(&ast, &mut environment);
    evaluate
}