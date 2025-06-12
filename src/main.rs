mod lib;
fn main() {
    let source: String = "x += (2*(3+1));".to_string();
    let tokens = lib::lexer::tokenise(source);
    println!("{:?}", tokens);
}
