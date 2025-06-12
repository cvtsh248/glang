mod lib;
fn main() {
    let source: String = " '(hello+2)' +2 " .to_string();
    let tokens = lib::lexer::tokenise(source);
    println!("{:?}", tokens);
}
