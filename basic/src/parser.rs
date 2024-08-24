use crate::lexer::Token;

#[derive(Debug)]
pub enum Expr {
    Bin(Box<Expr>, Token, Box<Expr>),
    Num(i64),
    Identifier(String)
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

    fn parse_primary(&mut self) -> Expr {
        let curr: &Token = &self.tokens[self.current];

        match curr {
            Token::Number(num) => {
                self.advance();
                return Expr::Num(num.parse().unwrap());
            },
            Token::Identifier(id) => {
                self.advance();
                return Expr::Identifier(id.parse().unwrap());
            }
            _ => panic!("Unexpected token in primary expression")
        }
    }

    fn parse_factor(&mut self) -> Expr {
        let mut left = self.parse_primary();

        while self.current < self.tokens.len() {
            let curr = &self.tokens[self.current];

            let op = match curr {
                Token::Op(op) if op == "*" || op == "/" => op.clone(),
                _ => break,
            };

            self.advance();
            let right = self.parse_primary();

            left = Expr::Bin(Box::new(left), Token::Op(op), Box::new(right))
        }

        return left;
    }

    fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_factor();

        while self.current < self.tokens.len() {
            let curr = &self.tokens[self.current];

            let op = match curr {
                Token::Op(op) if op == "+" || op == "-" => op.clone(),
                _ => break,
            };

            self.advance();
            let right = self.parse_factor();

            left = Expr::Bin(Box::new(left), Token::Op(op), Box::new(right))
        }

        return left;
    }

    fn parse_expr(&mut self) -> Expr {
        return self.parse_term();
    }

    pub fn parse(&mut self) -> Vec<Expr> {
        let mut exprs = Vec::new();

        while self.current < self.tokens.len() {
            exprs.push(self.parse_expr());
        }

        return exprs;
    }
}