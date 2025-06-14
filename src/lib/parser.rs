use crate::lib::lexer::TokenType;

use super::lexer;

#[derive(Debug, Clone)]
pub enum NodeType {
    Program,
    NumericLiteral,
    StringLiteral,
    Identifier,
    BinaryExpr(String)
}

#[derive(Debug, Clone)]
pub struct Node{ // Node
    node_type: NodeType,
    body: Vec<Node>,
    tokens: Vec<lexer::Token>
}
impl Node { // Master node will ALWAYS be of type Program and will always have all tokens in tokens
    fn generate_ast(&mut self){
        while let lexer::TokenType::EOF = self.tokens[0].token_type {
            self.tokens.remove(0);
        } 
    }
}

pub fn generate_ast(source: String) -> Node {

    let mut program = Node{
        node_type: NodeType::Program,
        body: vec![],
        tokens: lexer::tokenise(source)
    };

    program.generate_ast();

    program

}