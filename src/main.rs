mod lib;
fn main() {
    let source: String = "(3*(2+1))*5+4-1" .to_string();
    // let tokens = lib::lexer::tokenise(source);
    let ast = lib::parser::generate_ast(source);
    println!("{:?}", ast);
}
