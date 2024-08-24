mod lexer;
mod parser;
mod interpreter;

fn main() {
    let source: String = std::fs::read_to_string("example/code.txt").expect("error reading file");
    
    let mut lexer = lexer::Lexer::new(source);
    let tokens = lexer.tokenize();
    // for token in &tokens {
    //     println!("{:?}", token);
    // }

    let mut parser = parser::Parser::new(&tokens);
    let exprs = parser.parse();
    for expr in &exprs {
        println!("{:?}", expr)
    }

    let mut interpreter = interpreter::Interpreter::new(&exprs);
    interpreter.interpret();
}