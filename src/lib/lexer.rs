#[derive(Debug, Clone)]
pub enum TokenType {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Identifier(String),
    StringLiteral(String),
    Operator(String),
    OpenBracket,
    CloseBracket,
    OpenCurlyBracket,
    CloseCurlyBracket,
    Let,
    Punctuation(String),
    If,
    Loop,
    EOL,
    EOF
}
impl TokenType {
    fn check_reserved_keywords(word: &str) -> Option<TokenType>{
        match word{
            "let" => Some(TokenType::Let),
            "true" => Some(TokenType::Boolean(true)),
            "false" => Some(TokenType::Boolean(false)),
            "if" => Some(TokenType::If),
            "loop" => Some(TokenType::Loop),
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

    pub fn extract_bool_value(&self) -> Option<&bool> {
        if let TokenType::Boolean(boolean) = self {
            Some(boolean)
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
        if self.current_pos < self.tokens.len(){
            self.current_pos += 1;
        } else {
            panic!("Out of range")
        }
    }

    pub fn at(&self) -> Token{
        self.tokens[self.current_pos].clone()
    }
}

#[derive(Debug, Clone)]
struct DataStream {
    pub characters: Vec<char>,
    pub current_pos: usize
}
impl DataStream {
    // Last in first out
    pub fn push(&mut self, character: char){
        self.characters.push(character);
    }

    pub fn pop(&mut self){
        if self.current_pos < self.characters.len(){
            self.current_pos += 1;
        } else {
            panic!("Out of range")
        }
    }

    pub fn at(&self) -> char{
        self.characters[self.current_pos].clone()
    }
}



pub fn tokenise(source: String) -> TokenStream {
    let mut source_split: Vec<char> = source.chars().collect();

    let mut source_datastream = DataStream {
        characters: source_split,
        current_pos: 0
    };

    let mut tokens: Vec<Token> = Vec::new();
    while source_datastream.current_pos < source_datastream.characters.len(){
        let mut is_alphanumeric = false;
        let mut is_string_literal = false;
        if source_datastream.at() == '('{
            tokens.push(Token {
                token_type: TokenType::OpenBracket
            });
        } else if source_datastream.at() == ')'{
            tokens.push(Token {
                token_type: TokenType::CloseBracket
            });
        } else if source_datastream.at() == '{'{
            tokens.push(Token {
                token_type: TokenType::OpenCurlyBracket
            }); 
        } else if source_datastream.at() == '}'{
            tokens.push(Token {
                token_type: TokenType::CloseCurlyBracket
            });  
        } else if source_datastream.at() == '%'{
            tokens.push(Token {
                token_type: TokenType::Operator("%".to_string())
            }); 
        } else if source_datastream.at() == '+'{
            if source_datastream.characters.len() - source_datastream.current_pos > 1 && source_datastream.characters[source_datastream.current_pos+1] == '='{
                tokens.push(Token {
                    token_type: TokenType::Operator("+=".to_string())
                });
                source_datastream.pop()
                // source_split.remove(1);
            } else {
                tokens.push(Token {
                    token_type: TokenType::Operator("+".to_string())
                });
            }
        } else if source_datastream.at() == '-'{
            if source_datastream.characters.len() - source_datastream.current_pos > 1 && source_datastream.characters[source_datastream.current_pos+1] == '='{
                tokens.push(Token {
                    token_type: TokenType::Operator("-=".to_string())
                });
                source_datastream.pop()
                // source_split.remove(1);
            } else if source_datastream.characters.len() - source_datastream.current_pos > 1 && source_datastream.characters[source_datastream.current_pos+1].is_ascii_digit() && !source_datastream.characters[source_datastream.current_pos-1].is_ascii_digit(){
                // Negative numbers
                source_datastream.pop();
                let mut numeral: Vec<char> = Vec::new();
                while source_datastream.current_pos < source_datastream.characters.len() && (source_datastream.at().is_ascii_digit() || source_datastream.at() == '.'){
                    numeral.push(source_datastream.at());
                    source_datastream.pop();
                }

                let numeral_string: String = numeral.into_iter().collect();

                if numeral_string.contains("."){
                    let float_proper: f64 = numeral_string.parse::<f64>().unwrap();
                    tokens.push(Token {
                                    token_type: TokenType::Float(-1.0*float_proper)
                                })
                } else {
                    let integer_proper: i64 = numeral_string.parse::<i64>().unwrap();
                    tokens.push(Token {
                        token_type: TokenType::Integer(-1*integer_proper)
                    })
                }
                
                is_alphanumeric = true;
            } else {
                tokens.push(Token {
                    token_type: TokenType::Operator("-".to_string())
                });
            }
        } else if source_datastream.at() == '*'{
            if source_datastream.characters.len() - source_datastream.current_pos > 1 && source_datastream.characters[source_datastream.current_pos+1] == '=' {
                tokens.push(Token {
                    token_type: TokenType::Operator("*=".to_string())
                });
                source_datastream.pop()
                // source_split.remove(1);
            } else {
                tokens.push(Token {
                    token_type: TokenType::Operator("*".to_string())
                });
            }
        } else if source_datastream.at() == '/'{
            if source_datastream.characters.len() - source_datastream.current_pos > 1 && source_datastream.characters[source_datastream.current_pos+1] == '=' {
                tokens.push(Token {
                    token_type: TokenType::Operator("/=".to_string())
                });
                source_datastream.pop()
                // source_split.remove(1);
            } else {
                tokens.push(Token {
                    token_type: TokenType::Operator("/".to_string())
                });
            }
        } else if source_datastream.at() == '='{
            if source_datastream.characters.len() - source_datastream.current_pos > 1 && source_datastream.characters[source_datastream.current_pos+1] == '='{
                tokens.push(Token {
                    token_type: TokenType::Operator("==".to_string())
                });

                source_datastream.pop()
                // source_split.remove(1);
            } else {
                tokens.push(Token {
                    token_type: TokenType::Operator("=".to_string())
                });
            }
        } else if source_datastream.at() == '!' {
            if source_datastream.characters.len() - source_datastream.current_pos > 1 && source_datastream.characters[source_datastream.current_pos+1] == '='{
                tokens.push(Token {
                    token_type: TokenType::Operator("!=".to_string())
                });
                
                source_datastream.pop()
                // source_split.remove(1);
            } else {
                tokens.push(Token {
                    token_type: TokenType::Operator("!".to_string())
                });
            }
        } else if source_datastream.at() == '>' {
            if source_datastream.characters.len() - source_datastream.current_pos > 1 && source_datastream.characters[source_datastream.current_pos+1] == '='{
                tokens.push(Token {
                    token_type: TokenType::Operator(">=".to_string())
                });
                
                source_datastream.pop()
                // source_split.remove(1);
            } else {
                tokens.push(Token {
                    token_type: TokenType::Operator(">".to_string())
                });
            }
        } else if source_datastream.at() == '<' {
            if source_datastream.characters.len() - source_datastream.current_pos > 1 && source_datastream.characters[source_datastream.current_pos+1] == '='{
                tokens.push(Token {
                    token_type: TokenType::Operator("<=".to_string())
                });
                
                source_datastream.pop()
                // source_split.remove(1);
            } else {
                tokens.push(Token {
                    token_type: TokenType::Operator("<".to_string())
                });
            }
        } else if source_datastream.at() == ';' || source_datastream.at() == '\n'{
            tokens.push(Token {
                token_type: TokenType::EOL
            });
        } else if source_datastream.at() == '"'{
            let mut string_literal: Vec<char> = Vec::new();

            is_string_literal = true;
            source_datastream.pop();

            while source_datastream.current_pos < source_datastream.characters.len() && source_datastream.at() != '"' {
                string_literal.push(source_datastream.at());
                source_datastream.pop();
            }

            let string_literal_string: String = string_literal.iter().collect();
            
            tokens.push(Token {
                token_type: TokenType::StringLiteral(string_literal_string),
            });
        } else if source_datastream.at() == '\''{
            let mut string_literal: Vec<char> = Vec::new();

            is_string_literal = true;
            source_datastream.pop();

            while source_datastream.current_pos < source_datastream.characters.len() && source_datastream.at() != '\'' {
                string_literal.push(source_datastream.at());
                source_datastream.pop();
            }

            let string_literal_string: String = string_literal.iter().collect();
            
            tokens.push(Token {
                token_type: TokenType::StringLiteral(string_literal_string),
            });

        } else if source_datastream.at() == '&' {
            if source_datastream.characters.len() - source_datastream.current_pos > 1 && source_datastream.characters[source_datastream.current_pos+1] == '&'{
                tokens.push(Token {
                    token_type: TokenType::Operator("&&".to_string())
                });
                source_datastream.pop()
                // source_split.remove(1);
            } else {
                tokens.push(Token {
                    token_type: TokenType::Operator("&".to_string())
                });
            }
        } else if source_datastream.at() == '|' {
            if source_datastream.characters.len() - source_datastream.current_pos > 1 && source_datastream.characters[source_datastream.current_pos+1] == '|'{
                tokens.push(Token {
                    token_type: TokenType::Operator("||".to_string())
                });
                source_datastream.pop()
                // source_split.remove(1);
            } else {
                tokens.push(Token {
                    token_type: TokenType::Operator("|".to_string())
                });
            } 
        } else if source_datastream.at().is_ascii_alphabetic(){
            let mut identifier: Vec<char> = Vec::new();

            while source_datastream.current_pos < source_datastream.characters.len() && source_datastream.at().is_alphanumeric(){
                identifier.push(source_datastream.at());
                source_datastream.pop();
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


        } else if source_datastream.at().is_ascii_digit(){
            let mut numeral: Vec<char> = Vec::new();
            while source_datastream.current_pos < source_datastream.characters.len() && (source_datastream.at().is_ascii_digit() || source_datastream.at() == '.'){
                numeral.push(source_datastream.at());
                source_datastream.pop();
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
            
        } else if source_datastream.at() == '.' {
            tokens.push(Token {
                token_type: TokenType::Punctuation(".".to_string())
            });
        } else if source_datastream.at() == ' ' || source_datastream.at() == '\n'{
            // Do nothing
        }

        if !is_alphanumeric{
            source_datastream.pop()
            // source_split.remove(0);
        }
        
    }
    tokens.push(Token { token_type: TokenType::EOF });

    TokenStream {
        tokens: tokens,
        current_pos: 0
    }
}