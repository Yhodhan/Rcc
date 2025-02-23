use rcc::lexer::lex::Lexer;
use std::fs;

fn main() {
    let file = fs::read_to_string("test.c").unwrap();
    let lexer = Lexer::new(&file);
    let tokens = lexer.tokenize();

    for token in tokens {
        println!("{:?}", token);
    }
}
