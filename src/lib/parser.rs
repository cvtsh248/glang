use super::lexer;

#[derive(Debug, Clone)]
pub enum NodeType {
    Program,
    NumericLiteral,
    StringLiteral,
    Boolean,
    Identifier,
    BinaryExpr(String),
    UnaryExpr(String),
    Assignment,
    Declaration,
    Scope,
    Loop,
    Break,
    If,
    Else,
    ElseIf,
    Print,
    Function(String),
    FunctionCall(String),
    EOL
}
impl NodeType {
    pub fn extract_binexp_operator(&self) -> Option<&str> {
        if let NodeType::BinaryExpr(op) = self {
            Some(op)
        } else {
            None
        }
    }
    pub fn extract_unexp_operator(&self) -> Option<&str> {
        if let NodeType::UnaryExpr(op) = self {
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
        } else if matches!(&tokens.at().token_type, lexer::TokenType::Function) {
            tokens.pop();
            let fn_identifier = tokens.at().clone();
            let fn_identifier_str = fn_identifier.token_type.extract_str_value().unwrap();
            tokens.pop();
            if matches!(tokens.at().token_type, lexer::TokenType::OpenBracket){
                let mut body: Vec<Node> = vec![]; // Zeroeth item in body is input, next is scope
                let mut declarations: Vec<Node> = vec![]; // Zeroeth item of body; just a bunch of declarations for the scope
                while !matches!(tokens.at().token_type, lexer::TokenType::CloseBracket) {
                    tokens.pop();
                    if matches!(tokens.at().token_type, lexer::TokenType::Punctuation(op) if op == ",".to_string()) || matches!(tokens.at().token_type, lexer::TokenType::CloseBracket){
                        continue
                    } else {
                        declarations.push(Node {
                            node_type: NodeType::Declaration,
                            value: Some(tokens.at()), 
                            body: vec![Node {
                                node_type: NodeType::NumericLiteral,
                                value: Some(lexer::Token { token_type: lexer::TokenType::Integer(0) }),
                                body: vec![]
                            }]
                        });
                    }
                    // println!("{:?}",tokens);

                }
                tokens.pop();
                if matches!(tokens.at().token_type, lexer::TokenType::OpenCurlyBracket){
                    body.push(self.parse_expr(tokens));
                    for node in declarations {
                        body[0].body.insert(0,node); // Injecting declerations into scope, there's probably a way more efficient way of doing this
                    }
                    let ret = Node {node_type: NodeType::Function(fn_identifier_str.to_string()), value: None, body: body};
                    return ret
                } 
                panic!()
            }
            panic!()
        } else if matches!(&tokens.at().token_type, lexer::TokenType::FunctionCall(_)){
            let op = &tokens.at().token_type;
            let mut body: Vec<Node> = vec![]; // Body items are input values
            tokens.pop();
            if matches!(tokens.at().token_type, lexer::TokenType::OpenBracket){
                while !matches!(tokens.at().token_type, lexer::TokenType::CloseBracket) {
                    // println!("hi");
                    tokens.pop();
                    if matches!(tokens.at().token_type, lexer::TokenType::Punctuation(pn) if pn == ",".to_string()){
                        tokens.pop();
                    }
                    body.push(self.parse_expr(tokens));
                    // println!("{:?}",tokens);
                }
                tokens.pop();
            }
            
            let ret = Node {node_type: NodeType::FunctionCall(op.extract_fncall_identifier().unwrap().to_string()), value: None, body: body};
            ret
        } else {
            self.parse_expr(tokens)
        }
    }

    fn parse_expr(&mut self, tokens: &mut lexer::TokenStream) -> Node{
        self.parse_comparative_expr(tokens)
    }

    fn parse_comparative_expr(&mut self, tokens: &mut lexer::TokenStream) -> Node{
        let mut left: Node = self.parse_additive_expr(tokens);
        while matches!(&tokens.at().token_type, lexer::TokenType::Operator(op) if op == "==" || op == "!=" || op == ">" || op == "<" || op == ">=" || op == "<=" || op == "&&" || op == "||") {
            let operator = tokens.at();
            tokens.pop();
            let right = self.parse_additive_expr(tokens);
            left = Node {
                node_type: NodeType::BinaryExpr(operator.token_type.extract_operator().unwrap().to_string()),
                value: None,
                body: vec![left, right]
            };   
        }
        left
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
    
    fn parse_multiplicative_expr(& mut self, tokens: &mut lexer::TokenStream) -> Node{
        let mut left: Node = self.parse_power_expr(tokens);
        // tokens.pop();
        while matches!(&tokens.at().token_type, lexer::TokenType::Operator(op) if op == "*" || op == "/" || op == "%"){
            let operator = tokens.at();
            tokens.pop();
            let right = self.parse_power_expr(tokens);
            left = Node {
                node_type: NodeType::BinaryExpr(operator.token_type.extract_operator().unwrap().to_string()),
                value: None,
                body: vec![left, right]
            };
        }
        left

    }

    fn parse_power_expr(& mut self, tokens: &mut lexer::TokenStream) -> Node{
        let mut left: Node = self.parse_primary_expr(tokens);
        // tokens.pop();
        while matches!(&tokens.at().token_type, lexer::TokenType::Operator(op) if op == "**"){
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
                let parsed: Node = self.parse_comparative_expr(tokens);
                // tokens.pop();
                // println!("{:?}", parsed);

                if matches!(tokens.at().token_type, lexer::TokenType::CloseBracket){
                    tokens.pop();
                    parsed
                } else {
                    panic!("Unexpected token within brackets - expected closing bracket, got: {:?}",tokens.at().token_type)
                }
            },
            lexer::TokenType::OpenCurlyBracket => {
                tokens.pop();
                let mut body: Vec<Node> = vec![];
                while !matches!(tokens.at().token_type, lexer::TokenType::CloseCurlyBracket){
                    body.push(self.parse_stmt(tokens));
                }
                tokens.pop();
                Node {
                    node_type: NodeType::Scope,
                    value: None,
                    body: body,
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
            lexer::TokenType::Operator(op) if op == "!" => {
                tokens.pop();
                if matches!(tokens.at().token_type, lexer::TokenType::OpenBracket){
                    let mut body: Vec<Node> = vec![];
                        while !matches!(tokens.at().token_type, lexer::TokenType::EOF) && !matches!(tokens.at().token_type, lexer::TokenType::EOL) && !matches!(tokens.at().token_type, lexer::TokenType::CloseBracket) {
                            tokens.pop();
                            body.push(self.parse_expr(tokens));
                        }
                    tokens.pop();
                    let ret = Node {node_type: NodeType::UnaryExpr("!".to_string()), value: None, body: body};
                    return ret
                } else {
                    let right = self.parse_expr(tokens);
                    let body = vec![right];
                    let ret = Node {node_type: NodeType::UnaryExpr("!".to_string()), value: None, body: body};
                    return ret
                }
                // panic!();
            },
            lexer::TokenType::StringLiteral(_) => {
                let ret = Node {node_type: NodeType::StringLiteral, value: Some(tokens.at()), body: vec![]};
                tokens.pop();
                ret
            },
            lexer::TokenType::EOL => {
                let ret = Node { node_type: NodeType::EOL, value: None, body: vec![] };
                tokens.pop();
                ret
            },
            lexer::TokenType::Boolean(_) => {
                let ret = Node {node_type: NodeType::Boolean, value: Some(tokens.at()), body: vec![]};
                tokens.pop();
                ret
            },
            lexer::TokenType::If => {
                tokens.pop();
                if matches!(tokens.at().token_type, lexer::TokenType::OpenBracket){
                    let mut body: Vec<Node> = vec![]; // Zeroeth item in body is condition, next is scope
                    while !matches!(tokens.at().token_type, lexer::TokenType::EOF) && !matches!(tokens.at().token_type, lexer::TokenType::EOL) && !matches!(tokens.at().token_type, lexer::TokenType::CloseBracket) {
                        tokens.pop();
                        body.push(self.parse_expr(tokens));
                    }
                    tokens.pop();
                    if matches!(tokens.at().token_type, lexer::TokenType::OpenCurlyBracket){
                        body.push(self.parse_expr(tokens));
                        let ret = Node {node_type: NodeType::If, value: None, body: body};
                        return ret
                    } 
                    panic!()
                }
                panic!()
            },
            lexer::TokenType::ElseIf => {
                tokens.pop();
                if matches!(tokens.at().token_type, lexer::TokenType::OpenBracket){
                    let mut body: Vec<Node> = vec![]; // Zeroeth item in body is condition, next is scope
                    while !matches!(tokens.at().token_type, lexer::TokenType::EOF) && !matches!(tokens.at().token_type, lexer::TokenType::EOL) && !matches!(tokens.at().token_type, lexer::TokenType::CloseBracket) {
                        tokens.pop();
                        body.push(self.parse_expr(tokens));
                    }
                    tokens.pop();
                    if matches!(tokens.at().token_type, lexer::TokenType::OpenCurlyBracket){
                        body.push(self.parse_expr(tokens));
                        let ret = Node {node_type: NodeType::ElseIf, value: None, body: body};
                        return ret
                    } 
                    panic!()
                }
                panic!()
            },
            lexer::TokenType::Else => {
                tokens.pop();
                let mut body: Vec<Node> = vec![]; // Scope
                if matches!(tokens.at().token_type, lexer::TokenType::OpenCurlyBracket){
                    body.push(self.parse_expr(tokens));
                    let ret = Node {node_type: NodeType::Else, value: None, body: body};
                    return ret
                } 
                panic!()

            },
            lexer::TokenType::Loop => {
                tokens.pop();
                if matches!(tokens.at().token_type, lexer::TokenType::OpenBracket){
                    let mut body: Vec<Node> = vec![]; // Zeroeth item in body is condition, next is scope
                        while !matches!(tokens.at().token_type, lexer::TokenType::EOF) && !matches!(tokens.at().token_type, lexer::TokenType::EOL) && !matches!(tokens.at().token_type, lexer::TokenType::CloseBracket) {
                            tokens.pop();
                            body.push(self.parse_expr(tokens));
                        }
                    tokens.pop();
                    if matches!(tokens.at().token_type, lexer::TokenType::OpenCurlyBracket){
                        body.push(self.parse_expr(tokens));
                        let ret = Node {node_type: NodeType::Loop, value: None, body: body};
                        return ret
                    } 
                    panic!()
                }
                panic!()
            },
            lexer::TokenType::Print => {
                tokens.pop();
                if matches!(tokens.at().token_type, lexer::TokenType::OpenBracket){
                    let mut body: Vec<Node> = vec![]; // Item is what is to be printed
                        while !matches!(tokens.at().token_type, lexer::TokenType::EOF) && !matches!(tokens.at().token_type, lexer::TokenType::EOL) && !matches!(tokens.at().token_type, lexer::TokenType::CloseBracket) {
                            tokens.pop();
                            body.push(self.parse_expr(tokens));
                        }
                    tokens.pop();
                    let ret = Node {node_type: NodeType::Print, value: None, body: body};
                    return ret
                }
                panic!()
            }
            lexer::TokenType::Break => {
                let ret = Node {node_type: NodeType::Break, value: None, body: vec![]};
                tokens.pop();
                ret
            }
            _ => panic!("{:?}", tokens.at().token_type)
        }
        
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

    // println!("{:?}",tokens);

    program.generate_ast(&mut tokens);

    program

}