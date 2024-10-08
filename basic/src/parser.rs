use core::panic;

use crate::lexer::Token;

#[derive(Debug)]
pub enum Expr {
    Bin(Box<Expr>, Token, Box<Expr>),
    Rel(Box<Expr>, Token, Box<Expr>),
    Num(i64),
    Identifier(String),
    Str(String),
    VarDec(String, Box<Expr>),
    Print(Box<Expr>),
    Input(String, Box<Expr>),
    If(Box<Expr>, Box<Expr>),
    Assign(String, Box<Expr>),
    For {
        variable: String,
        start: Box<Expr>,
        end: Box<Expr>,
        step: Option<Box<Expr>>,
        body: Vec<Expr>,
    },
    Call(String, Vec<Expr>),
}

pub struct Parser<'a> {
    current: usize,
    tokens: &'a[Token]
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Parser {
        Parser {
            current: 0,
            tokens
        }
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn expect(&mut self, expected: Token) {
        let curr = &self.tokens[self.current];
        
        if curr == &expected {
            self.advance();
        } else {
            panic!("Expected {:?}, got {:?}", expected, curr)
        }
    }

    fn parse_primary(&mut self) -> Expr {
        let curr: &Token = &self.tokens[self.current];

        match curr {
            Token::Number(num) => {
                self.advance();
                return Expr::Num(num.parse().unwrap());
            },
            Token::Identifier(id) => {
                self.advance();
                return Expr::Identifier(id.clone());
            }
            Token::Str(str) => {
                self.advance();
                return Expr::Str(str.clone());
            }
            Token::BuiltIn(_) => {
                return self.parse_call();
            } 
            _ => panic!("Unexpected token in primary expression {:?}", curr)
        }
    }

    fn parse_factor(&mut self) -> Expr {
        let mut left = self.parse_primary();

        while self.current < self.tokens.len() {
            let curr = &self.tokens[self.current];

            let op = match curr {
                Token::BinOp(op) if op == "*" || op == "/" || op == "%" => op.clone(),
                _ => break,
            };

            self.advance();
            let right = self.parse_primary();

            left = Expr::Bin(Box::new(left), Token::BinOp(op), Box::new(right))
        }

        return left;
    }

    fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_factor();

        while self.current < self.tokens.len() {
            let curr = &self.tokens[self.current];

            let op = match curr {
                Token::BinOp(op) if op == "+" || op == "-" => op.clone(),
                _ => break,
            };

            self.advance();
            let right = self.parse_factor();

            left = Expr::Bin(Box::new(left), Token::BinOp(op), Box::new(right))
        }

        return left;
    }
    
    fn parse_relational(&mut self) -> Expr {
        let mut left = self.parse_term();

        while self.current < self.tokens.len() {
            let curr = &self.tokens[self.current];

            let op = match curr {
                Token::RelOp(op) if op == ">" || op == ">=" || op == "<" || op == "<=" 
                    || op == "=" || op == "<>" || op == "!=" => op.clone(),

                _ => break,
            };

            self.advance();
            let right = self.parse_term();

            left = Expr::Rel(Box::new(left), Token::RelOp(op), Box::new(right))
        }

        return left;
    }

    fn parse_assign(&mut self) -> Expr {
        let identifier = match &self.tokens[self.current] {
            Token::Identifier(id) => id.clone(),
            _ => panic!("Expected an identifier for variable declaration"),
        };
        self.advance();

        self.expect(Token::RelOp("=".to_string()));
        
        let expr = self.parse_expr();

        return Expr::Assign(identifier, Box::new(expr));
    }

    fn parse_expr(&mut self) -> Expr {
        return self.parse_relational();
    }

    fn parse_for_loop(&mut self) -> Expr {
        self.expect(Token::Keyword("FOR".to_string()));

        let variable = match &self.tokens[self.current] {
            Token::Identifier(id) => id.clone(),
            _ => panic!("Expected an identifier for variable declaration"),
        };
        self.advance();

        self.expect(Token::RelOp("=".to_string()));
        
        let start = self.parse_expr();

        self.expect(Token::Keyword("TO".to_string()));
        let end = self.parse_expr();

        let step = if self.current < self.tokens.len() {
            if let Token::Keyword(keyword) = &self.tokens[self.current] {
                if keyword == "STEP" {
                    self.advance();
                    let step = self.parse_expr();
                    Some(Box::new(step))
                } else { None }
            } else { None }
        } else { None };

        let mut body = Vec::new();
        while self.current < self.tokens.len() {
            match &self.tokens[self.current] {
                Token::Keyword(word) if word == "NEXT" => {
                    self.advance();
                    self.expect(Token::Identifier(variable.clone()));
                },
                _ => { body.push(self.parse_stmt()); }
            }
        }

        Expr::For {
            variable, start: Box::new(start), end: Box::new(end),
            step, body
        }
    }

    fn parse_if_stmt(&mut self) -> Expr {
        self.expect(Token::Keyword("IF".to_string()));

        let expr = self.parse_expr();
        self.expect(Token::Keyword("THEN".to_string()));

        let consequent = self.parse_stmt();

        return Expr::If(Box::new(expr), Box::new(consequent));
    }

    fn parse_var_dec(&mut self) -> Expr {
        self.expect(Token::Keyword("LET".to_string()));
    
        let identifier = match &self.tokens[self.current] {
            Token::Identifier(id) => id.clone(),
            _ => panic!("Expected an identifier for variable declaration"),
        };
        self.advance();
    
        self.expect(Token::RelOp("=".to_string()));
        let expr = self.parse_expr();
        
        Expr::VarDec(identifier, Box::new(expr))
    }

    fn parse_print(&mut self) -> Expr {
        self.expect(Token::Keyword("PRINT".to_string()));
    
        let expr = self.parse_expr();
        Expr::Print(Box::new(expr))
    }
    
    fn parse_input(&mut self) -> Expr  {
        self.expect(Token::Keyword("INPUT".to_string()));

        if let Token::Str(s) = &self.tokens[self.current] {
            self.advance();
            let identifier = self.parse_primary();

            return Expr::Input(s.clone(), Box::new(identifier));
        }

        panic!("Unexpected expression for input");
    }

    fn parse_call(&mut self) -> Expr {
        let func = match &self.tokens[self.current] {
            Token::BuiltIn(id) => id.clone(),
            _ => panic!("Expected function name, got {:?}", &self.tokens[self.current]),
        };
        self.advance();
    
        self.expect(Token::Punc("(".to_string()));
        
        let mut args = Vec::new();
        while self.current < self.tokens.len() {
            let arg = self.parse_expr();
            args.push(arg);
            
            if matches!(self.tokens[self.current], Token::Punc(ref punc) if punc == &",".to_string()) {
                self.advance();
            } else if matches!(self.tokens[self.current], Token::Punc(ref punc) if punc == &")".to_string()) {
                self.advance();
                break;
            } else {
                panic!("Expected comma or closing parenthesis in function call");
            }
        }
        
        Expr::Call(func, args)
    }
    

    fn parse_stmt(&mut self) -> Expr {
        match &self.tokens[self.current] {
            Token::Keyword(word) => match word.as_str() {
                "LET" => self.parse_var_dec(),
                "PRINT" => self.parse_print(),
                "INPUT" => self.parse_input(),
                "IF" => self.parse_if_stmt(),
                "FOR" => self.parse_for_loop(),
                _ => panic!("Unknown keyword"),
            },
            Token::BuiltIn(_) => self.parse_call(),
            Token::Identifier(_) => {
                if self.tokens.len() > self.current + 1 {
                    if let Token::RelOp(op) = &self.tokens[self.current + 1] {
                        if op == "=" { return self.parse_assign(); }
                    }
                }
                self.parse_expr()
            },
            _ => self.parse_expr(),
        }
    }

    pub fn parse(&mut self) -> Vec<Expr> {
        let mut exprs = Vec::new();

        while self.current < self.tokens.len() {
            exprs.push(self.parse_stmt());
        }

        return exprs;
    }
}