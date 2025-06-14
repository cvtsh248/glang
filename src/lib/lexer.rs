#[derive(Debug, Clone)]
pub enum TokenType {
    Integer(i64),
    Float(f64),
    Identifier(String),
    StringLiteral(String),
    Operator(String),
    OpenBracket,
    CloseBracket,
    Let,
    Punctuation(String),
    EOL,
    EOF
}
impl TokenType {
    fn check_reserved_keywords(word: &str) -> Option<TokenType>{
        match word{
            "let" => Some(TokenType::Let),
            _=>None
        }
        
    }

    pub fn extract_operator(&self) -> Option<&str> {
        if let TokenType::Operator(op) = self {
            Some(op)
        } else {
            None
        }
    }

    pub fn extract_int_value(&self) -> Option<&i64> {
        if let TokenType::Integer(val) = self {
            Some(val)
        } else {
            None
        }
    }

    pub fn extract_float_value(&self) -> Option<&f64> {
        if let TokenType::Float(val) = self {
            Some(val)
        } else {
            None
        }
    }

    pub fn extract_str_value(&self) -> Option<&str> {
        if let TokenType::Identifier(str) = self {
            Some(str)
        } else if  let TokenType::Identifier(str) = self {
            Some(str)
        } else if  let TokenType::StringLiteral(str) = self {
            Some(str)
        } else if  let TokenType::Punctuation(str) = self {
            Some(str)
        } else {
            None
        }
    }

}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
}

#[derive(Debug, Clone)]
pub struct TokenStream {
    pub tokens: Vec<Token>,
    pub current_pos: usize
}
impl TokenStream {
    // Last in first out
    pub fn push(&mut self, token: Token){
        self.tokens.push(token);
    }

    pub fn pop(&mut self){
        self.current_pos += 1;
    }

    pub fn at(&self) -> Token{
        self.tokens[self.current_pos].clone()
    }
}

pub fn tokenise(source: String) -> Vec<Token> {
    let mut source_split: Vec<char> = source.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();
    while source_split.len() > 0{
        let mut is_alphanumeric = false;
        let mut is_string_literal = false;
        if source_split[0] == '('{
            tokens.push(Token {
                token_type: TokenType::OpenBracket
            });
        } else if source_split[0] == ')'{
            tokens.push(Token {
                token_type: TokenType::CloseBracket
            });
        } else if source_split[0] == '+'{
            if source_split.len() > 1 && source_split[1] == '='{
                tokens.push(Token {
                    token_type: TokenType::Operator("+=".to_string())
                });
                source_split.remove(1);
            } else {
                tokens.push(Token {
                    token_type: TokenType::Operator("+".to_string())
                });
            }
        } else if source_split[0] == '-'{
            if source_split.len() > 1 && source_split[1] == '='{
                tokens.push(Token {
                    token_type: TokenType::Operator("-=".to_string())
                });
                source_split.remove(1);
            } else {
                tokens.push(Token {
                    token_type: TokenType::Operator("-".to_string())
                });
            }
        } else if source_split[0] == '*'{
            if source_split.len() > 1 && source_split[1] == '=' {
                tokens.push(Token {
                    token_type: TokenType::Operator("*=".to_string())
                });
                source_split.remove(1);
            } else {
                tokens.push(Token {
                    token_type: TokenType::Operator("*".to_string())
                });
            }
        } else if source_split[0] == '/'{
            if source_split.len() > 1 && source_split[1] == '=' {
                tokens.push(Token {
                    token_type: TokenType::Operator("/=".to_string())
                });
                source_split.remove(1);
            } else {
                tokens.push(Token {
                    token_type: TokenType::Operator("/".to_string())
                });
            }
        } else if source_split[0] == '='{
            if source_split.len() > 1 && source_split[1] == '='{
                tokens.push(Token {
                    token_type: TokenType::Operator("==".to_string())
                });
                source_split.remove(1);
            } else {
                tokens.push(Token {
                    token_type: TokenType::Operator("=".to_string())
                });
            }
        } else if source_split[0] == ';' {
            tokens.push(Token {
                token_type: TokenType::EOL
            });
        } else if source_split[0] == '"'{
            let mut string_literal: Vec<char> = Vec::new();

            is_string_literal = true;
            source_split.remove(0);

            while source_split.len() > 0 && source_split[0] != '"' {
                string_literal.push(source_split[0]);
                source_split.remove(0);
            }

            let string_literal_string: String = string_literal.iter().collect();
            
            tokens.push(Token {
                token_type: TokenType::StringLiteral(string_literal_string),
            });
        } else if source_split[0] == '\''{
            let mut string_literal: Vec<char> = Vec::new();

            is_string_literal = true;
            source_split.remove(0);

            while source_split.len() > 0 && source_split[0] != '\'' {
                string_literal.push(source_split[0]);
                source_split.remove(0);
            }

            let string_literal_string: String = string_literal.iter().collect();
            
            tokens.push(Token {
                token_type: TokenType::StringLiteral(string_literal_string),
            });

        } else if source_split[0].is_ascii_alphabetic(){
            let mut identifier: Vec<char> = Vec::new();

            while source_split.len() > 0 && source_split[0].is_alphanumeric(){
                identifier.push(source_split[0]);
                source_split.remove(0);
            }

            is_alphanumeric = true;

            let identifier_string: String = identifier.into_iter().collect();

            let token_type = TokenType::check_reserved_keywords(&identifier_string);

            if token_type.is_some() {
                    tokens.push(Token {
                    token_type: token_type.expect("this is impossible to trigger")
                });
            } else {
                tokens.push(Token {
                    token_type: TokenType::Identifier(identifier_string)
                });
            }


        } else if source_split[0].is_ascii_digit(){
            let mut numeral: Vec<char> = Vec::new();
            while source_split.len() > 0 && (source_split[0].is_ascii_digit() || source_split[0] == '.'){
                numeral.push(source_split[0]);
                source_split.remove(0);
            }

            let numeral_string: String = numeral.into_iter().collect();

            if numeral_string.contains("."){
                let float_proper: f64 = numeral_string.parse::<f64>().unwrap();
                tokens.push(Token {
                                token_type: TokenType::Float(float_proper)
                            })
            } else {
                let integer_proper: i64 = numeral_string.parse::<i64>().unwrap();
                tokens.push(Token {
                    token_type: TokenType::Integer(integer_proper)
                })
            }
            

            is_alphanumeric = true;
            

        } else if source_split[0] == '.' {
            tokens.push(Token {
                token_type: TokenType::Punctuation(".".to_string())
            });
        } else if source_split[0] == ' ' || source_split[0] == '\n'{
            // Do nothing
        }

        if !is_alphanumeric{
            source_split.remove(0);
        }
        
    }
    tokens.push(Token { token_type: TokenType::EOF });
    tokens
}