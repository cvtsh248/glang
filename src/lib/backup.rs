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

// #[derive(Debug, Clone)]
// pub struct Stmt{ // Node
//     node_type: NodeType,
// }

// #[derive(Debug, Clone)]
// pub struct Program {
//     stmt: Stmt,
//     body: Vec<Stmt>
// }

// #[derive(Debug, Clone)]
// pub struct Expr {
//     stmt: Stmt
// }

// #[derive(Debug, Clone)]
// pub struct BinaryExpr {
//     expr: Expr,
//     left: lexer::TokenType,
//     right: lexer::TokenType,
//     operator: lexer::TokenType
// }

// pub fn generate_ast(source: String) -> Program {
//     let mut tokens: Vec<lexer::Token> = lexer::tokenise(source);

//     let mut program = Program {
//         stmt: Stmt { node_type: NodeType::Program },
//         body: vec![]
//     };

//     while !matches!(tokens[0].token_type, lexer::TokenType::EOF){ // Can't use != here cause can't implement eq/partial eq for a float
//         if matches!(tokens[0].token_type, lexer::TokenType::Integer(_)){
//             program.body.push(Stmt { node_type: NodeType::NumericLiteral });
//         }
//     }

//     program
// }

#[derive(Debug, Clone)]
pub struct Node{ // Node
    node_type: NodeType,
    body: Vec<Node>
}
impl Node { // Master node will ALWAYS be of type Program
    fn parse_stmt(&mut self, tokens: &Vec<lexer::Token>) -> Node {
        Node::parse_expr(self, &tokens)

    }

    fn parse_expr(&mut self, tokens: &Vec<lexer::Token>) -> Node {
        Node::parse_additive_expr(self, &tokens)
    }

    fn parse_additive_expr(&mut self, tokens: &Vec<lexer::Token>) -> Node {
        let left = Node::parse_multiplicative_expr(self, tokens);
        left
        
    }
    fn parse_primary_expr(tokens: &Vec<lexer::Token>) -> Node {
        match &tokens[0].token_type{
            lexer::TokenType::Integer(_) => {
                Node { node_type: NodeType::NumericLiteral, body: vec![] }
            },
            lexer::TokenType::Float(_) => {
                Node { node_type: NodeType::NumericLiteral, body: vec![]}
            },
            // lexer::TokenType::Operator(op) if op == "+" => { // Binary Expression
            //     Node { node_type: NodeType::BinaryExpr("+".to_string()), body: vec![]} // Body includes two nodes, one on the left one on the right
            // }
            _ => panic!("{:?}",&tokens[0].token_type)
        }
    }

    fn parse_multiplicative_expr(&mut self, tokens: &Vec<lexer::Token>) -> Node {
        let mut left = Node::parse_primary_expr(tokens);

        let mut counter = 0;
        while matches!(&tokens[counter].token_type, lexer::TokenType::Operator(op) if op == "*" || op == "/") && counter < tokens.len(){
            let operator = tokens[counter].token_type.extract_operator();
            counter += 1;
            // tokens.remove(0);
            let right = Node::parse_primary_expr(&tokens[counter..].to_vec());
            left = Node {
                node_type: NodeType::BinaryExpr(operator.unwrap().to_string()),
                body: vec![left, right]

            };
        }
        
        left

    }

    fn generate_ast(&mut self, source: String) {

        let mut tokens: Vec<lexer::Token> = lexer::tokenise(source);

        // let mut program = Node {
        //     node_type: NodeType::Program,
        //     body: vec![]
        // };
        self.node_type = NodeType::Program;
        self.body = vec![];

        while !matches!(tokens[0].token_type, lexer::TokenType::EOF){ // Can't use != here cause can't implement eq/partial eq for a float
            let parsed= Node::parse_stmt(self, &tokens);
            self.body.push(parsed);
            tokens.remove(0);

        }

        // self.clone()
    }
}

pub fn generate_ast(source: String) -> Node {
    let mut program = Node{
        node_type: NodeType::Program,
        body: vec![]
    };

    program.generate_ast(source);

    program
}