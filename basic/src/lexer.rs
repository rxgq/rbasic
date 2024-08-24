use core::panic;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(String),
    Str(String),
    Identifier(String),
    Op(String),
    Keyword(String)
}

pub struct Lexer {
    source: String,
    current: usize
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source,
            current: 0
        }
    }

    fn current(&self) -> Option<char> {
        return self.source.chars().nth(self.current);
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    pub fn numeric(&mut self) -> Token {
        let mut num_str = String::new();
        let mut has_decimal = false;

        while let Some(c) = self.current() {
            match c {
                '0'..='9' => {
                    num_str.push(c);
                    self.advance();
                }
                '.' if !has_decimal => {
                    has_decimal = true;
                    num_str.push(c);
                    self.advance();
                }
                _ => break,
            }
        }

        Token::Number(num_str)
    }

    pub fn string(&mut self) -> Token {
        self.advance();
        let str: String = self.source[self.current..]
            .chars().take_while(|&c| c != '\"').collect();

        self.current += str.len() + 1;
        return Token::Str(str);
    }

    pub fn identifier(&mut self) -> Token {
        let str: String = self.source[self.current..]
            .chars().take_while(|&c| c.is_alphabetic() || c == '_').collect();
        
        self.current += str.len() + 1;

        let keywords = ["LET", "IF", "ELSEIF", "ELSE", "THEN", "PRINT", "AND", "OR", "NOT", "INPUT", "TRUE", "FALSE"];
        if keywords.contains(&str.to_uppercase().as_str()) {
            return Token::Keyword(str.to_uppercase());
        }

        return Token::Identifier(str);
    }

    pub fn operator(&mut self) -> Token {
        let ops = ["+", "-", "*", "/", "%", "=", "<", "<=", ">", ">="];
        for _ in ops {
            let single = self.current().expect("ERROR tokenizing operator").to_string();
            let mut double = single.clone();

            if let Some(next) = self.source.chars().nth(self.current + 1) {
                double.push(next);
            }

            if ops.contains(&double.as_str()) {
                self.current += 2;
                return Token::Op(double);
            }

            if ops.contains(&single.as_str()) {
                self.current += 1;
                return Token::Op(single);
            }

            panic!("Unexpected operator");
        }

        return Token::Op(String::new());
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(c) = self.source.chars().nth(self.current) {
            if c.is_whitespace() {
                self.advance();
            } else if c.is_ascii_digit() {
                tokens.push(self.numeric());
            } else if c == '\"' {
                tokens.push(self.string());
            } else if c.is_alphabetic() {
                tokens.push(self.identifier());
            } else if "+-*/=<>^%".contains(c) {
                tokens.push(self.operator());
            } else { 
                self. advance();
            }
        }

        return tokens
    }
}