mod lib;
fn main() {
    let source: String = "2*2+1" .to_string();
    // let tokens = lib::lexer::tokenise(source);
    let ast = lib::parser::generate_ast(source);
    println!("{:?}", ast);
}
