use std::fs;

use lexer::{Lexer, Token};

mod lexer;

fn main() {
    let source = fs::read_to_string("example/code.txt").expect("could not open file");
    let mut lexer = Lexer::new(source);

    let tokens: Vec<Token> = lexer.tokenize();
    for token in tokens {
        println!("'{}' = {:?}", token.lexeme, token.token_type);
    }

    
}