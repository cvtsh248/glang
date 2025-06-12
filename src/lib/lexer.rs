use std::{any::Any, collections::HashMap, string};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum TokenType {
    Number(i64),
    Identifier(String),
    StringLiteral(String),
    Operator(String),
    OpenBracket,
    CloseBracket,
    Let,
    Punctuation(String),
    EOL
}
impl TokenType {
    fn check_reserved_keywords(keyword: &String) -> Option<TokenType>{
        let reserved: HashMap<String, TokenType> = HashMap::from([
            ("let".to_string(),TokenType::Let),
        ]);
        reserved.get(keyword).cloned()
    }
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
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
        } else if source_split[0] == '.' {
            tokens.push(Token {
                token_type: TokenType::Punctuation(".".to_string())
            });
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

            if TokenType::check_reserved_keywords(&identifier_string) == Some(TokenType::Let,) {
                    tokens.push(Token {
                    token_type: TokenType::Let
                });
            } else {
                tokens.push(Token {
                    token_type: TokenType::Identifier(identifier_string)
                });
            }


        } else if source_split[0].is_ascii_digit(){
            let mut numeral: Vec<char> = Vec::new();
            while source_split.len() > 0 && source_split[0].is_ascii_digit(){
                numeral.push(source_split[0]);
                source_split.remove(0);
            }

            let number_string: String = numeral.into_iter().collect();
            let number_proper: i64 = number_string.parse::<i64>().unwrap();

            is_alphanumeric = true;
            
            tokens.push(Token {
                token_type: TokenType::Number(number_proper)
            })
        } else if source_split[0] == ' ' || source_split[0] == '\n'{
            // Do nothing
        }

        if !is_alphanumeric{
            source_split.remove(0);
        }
        
    }
    tokens
}