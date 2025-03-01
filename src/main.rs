use rcc::lexer::lex::Lexer;
use rcc::parser::pars::Parser;
use std::fs;

fn main() {
    let file = fs::read_to_string("test.c").unwrap();
    let lexer = Lexer::new(&file);
    let tokens = lexer.tokenize();

    let parser = Parser::new();
    parser.process_tokens(tokens);
}
