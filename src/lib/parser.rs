use super::lexer;

#[derive(Debug, Clone)]
pub enum NodeType {
    Program,
    NumericLiteral,
    StringLiteral,
    Identifier,
    BinaryExpr(String),
    Assignment,
    Declaration
}
impl NodeType {
    pub fn extract_binexp_operator(&self) -> Option<&str> {
        if let NodeType::BinaryExpr(op) = self {
            Some(op)
        } else {
            None
        }
    }
}
#[derive(Debug, Clone)]
pub struct Node{ // Node
    pub node_type: NodeType,
    pub value: Option<lexer::Token>,
    pub body: Vec<Node>
}
impl Node { // Master node will ALWAYS be of type Program and will always have all tokens in tokens
    fn parse_stmt(&mut self, tokens: &mut lexer::TokenStream) -> Node{
        if matches!(&tokens.at().token_type, lexer::TokenType::Let){ // Declaration
            tokens.pop();
            if !matches!(tokens.at().token_type, lexer::TokenType::Identifier(_)){
                panic!("Expected identifier after let")
            } else {
                let mut ret = Node {node_type: NodeType::Declaration, value: Some(tokens.at()), body: vec![]};
                tokens.pop();
                while !(matches!(&tokens.at().token_type, lexer::TokenType::EOL) || matches!(&tokens.at().token_type, lexer::TokenType::EOF)) {
                    tokens.pop();
                    ret.body.push(self.parse_expr(tokens));
                    
                }
                
                return ret 
            }
        } else {
            self.parse_expr(tokens)
        }
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
                let parsed: Node = self.parse_additive_expr(tokens);
                // tokens.pop();
                // println!("{:?}", parsed);

                if matches!(tokens.at().token_type, lexer::TokenType::CloseBracket){
                    tokens.pop();
                    parsed
                } else {
                    panic!("Unexpected token within brackets - expected closing bracket")
                }
            },
            lexer::TokenType::Identifier(_) => {
                let identifier = tokens.at();
                let ret = Node {node_type: NodeType::Identifier, value: Some(identifier.clone()), body: vec![]};
                tokens.pop();
                if matches!(tokens.at().token_type, lexer::TokenType::Operator(val) if val == "="){ // Assignment
                    // tokens.pop();
                    let mut body: Vec<Node> = vec![];
                    while !matches!(tokens.at().token_type, lexer::TokenType::EOF) && !matches!(tokens.at().token_type, lexer::TokenType::EOL) {
                        tokens.pop();
                        body.push(self.parse_expr(tokens));
                    }
                    let ret = Node {node_type: NodeType::Assignment, value: Some(identifier.clone()), body: body};
                    return ret
                }
                ret
            },
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