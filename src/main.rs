mod lib;
fn main() {
    let source: String = "1+2*(3+4)" .to_string();
    // let tokens = lib::lexer::tokenise(source);
    let ast = lib::parser::generate_ast(source);
    println!("{:?}", ast);
    let evaluate = lib::eval::eval_program(&ast);
    println!("{:?}", evaluate);
}
