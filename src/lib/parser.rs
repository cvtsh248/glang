use crate::lib::lexer::Token;

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
    value: Option<Token>,
    body: Vec<Node>
}
impl Node { // Master node will ALWAYS be of type Program and will always have all tokens in tokens
    fn parse_stmt(&mut self, tokens: &mut lexer::TokenStream) -> Node{
        self.parse_expr(tokens)
    }

    fn parse_expr(&mut self, tokens: &mut lexer::TokenStream) -> Node{
        self.parse_additive_expr(tokens)
    }

    fn parse_additive_expr(&mut self, tokens: &mut lexer::TokenStream) -> Node{
        let mut left: Node = self.parse_multiplicative_expr(tokens);
        // tokens.pop();
        while matches!(&tokens.at().token_type, lexer::TokenType::Operator(op) if op == "+" || op == "-"){
            let operator = tokens.at();
            tokens.pop();
            let right = self.parse_multiplicative_expr(tokens);
            left = Node {
                node_type: NodeType::BinaryExpr(operator.token_type.extract_operator().unwrap().to_string()),
                value: None,
                body: vec![left, right]
            };
        }
        left
    }

    fn parse_primary_expr(&mut self, tokens: &mut lexer::TokenStream) -> Node{
        match &tokens.at().token_type{
            lexer::TokenType::Integer(_) => {
                let ret = Node { node_type: NodeType::NumericLiteral, value: Some(tokens.at()), body: vec![] };
                tokens.pop();
                ret
            },
            lexer::TokenType::Float(_) => {
                let ret = Node { node_type: NodeType::NumericLiteral, value: Some(tokens.at()), body: vec![] };
                tokens.pop();
                ret
            },
            lexer::TokenType::OpenBracket => {
                tokens.pop();
                let parsed = self.parse_primary_expr(tokens);
                println!("{:?}", parsed);
                tokens.pop();
                if matches!(tokens.at().token_type, lexer::TokenType::CloseBracket){
                    parsed
                } else {
                    panic!("Unexpected token")
                }
                
                
            }
            _ => panic!("{:?}", tokens.at().token_type)
        }
        
    }
    
    fn parse_multiplicative_expr(& mut self, tokens: &mut lexer::TokenStream) -> Node{
        let mut left: Node = self.parse_primary_expr(tokens);
        // tokens.pop();
        while matches!(&tokens.at().token_type, lexer::TokenType::Operator(op) if op == "*" || op == "/"){
            let operator = tokens.at();
            tokens.pop();
            let right = self.parse_primary_expr(tokens);
            left = Node {
                node_type: NodeType::BinaryExpr(operator.token_type.extract_operator().unwrap().to_string()),
                value: None,
                body: vec![left, right]
            };
        }
        left

    }

    fn generate_ast(&mut self, tokens: &mut lexer::TokenStream){
        while !matches!(tokens.at().token_type, lexer::TokenType::EOF) {
            let parsed = self.parse_stmt(tokens);
            self.body.push(parsed);
            // println!("{:?}",self.body);
            // tokens.pop();
        }
    }
}

pub fn generate_ast(source: String) -> Node {

    let mut program = Node{
        node_type: NodeType::Program,
        value: None,
        body: vec![]
    };

    let mut tokens = lexer::tokenise(source);

    program.generate_ast(&mut tokens);

    program

}