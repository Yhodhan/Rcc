use regex::Regex;

pub struct Lexer {
    pub src: Vec<char>,
}

#[derive(Debug)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    LeftParen(char),
    RightParen(char),
    LeftBracket(char),
    RightBracket(char),
    LeftBrace(char),
    RightBrace(char),
    Semicolon(char),
    Equal(char),
    Integer(i32),
    Operand(char),
    Obelisk(char),
    Unknown,
}

impl Lexer {
    pub fn new(src: &str) -> Self {
        let code: Vec<char> = src.chars().collect();
        Lexer { src: code }
    }

    pub fn split_symbols(&self) -> Vec<String> {
        let mut symbols = Vec::new();
        let mut symbol = String::new();

        for ch in &self.src {
            match ch {
                '\n' | ' ' => {
                    if !symbol.is_empty() {
                        symbols.push(symbol.clone());
                        symbol.clear();
                    }
                }
                '(' | ')' | '[' | ']' | '{' | '}' | '=' | ';' => {
                    if !symbol.is_empty() {
                        symbols.push(symbol.clone());
                        symbol.clear();
                    }
                    symbols.push(ch.to_string());
                }
                _ => symbol.push(*ch),
            }
        }
        symbols
    }

    fn obtain_char(&self, symbol: String) -> char {
        symbol.chars().next().unwrap()
    }

    pub fn tokenize(&self) -> Vec<Token> {
        let symbols = self.split_symbols();
        let mut tokens = Vec::with_capacity(symbols.len());
        // regexes
        let int_regex = Regex::new(r"[0-9]+").unwrap();
        let left_brace = Regex::new(r"\{").unwrap();
        let right_brace = Regex::new(r"\}").unwrap();
        let left_paren = Regex::new(r"\(").unwrap();
        let right_paren = Regex::new(r"\)").unwrap();
        let semicolon = Regex::new(r";").unwrap();
        let int_keyword = Regex::new(r"int").unwrap();
        let return_keyword = Regex::new(r"return").unwrap();
        let identifier = Regex::new(r"[a-zA-z]\w*").unwrap();
        let equal = Regex::new(r"=").unwrap();
        let operand = Regex::new(r"<|>|>=|<=").unwrap();
        let obelisk = Regex::new(r"\*").unwrap();

        for symbol in symbols {
            let token;
            if int_regex.is_match(&symbol) {
                let val = symbol.parse::<i32>().unwrap();
                token = Token::Integer(val);
            } else if left_brace.is_match(&symbol) {
                token = Token::LeftBrace(self.obtain_char(symbol));
            } else if right_brace.is_match(&symbol) {
                token = Token::RightBrace(self.obtain_char(symbol));
            } else if left_paren.is_match(&symbol) {
                token = Token::LeftParen(self.obtain_char(symbol));
            } else if right_paren.is_match(&symbol) {
                token = Token::RightParen(self.obtain_char(symbol));
            } else if semicolon.is_match(&symbol) {
                token = Token::Semicolon(self.obtain_char(symbol));
            } else if operand.is_match(&symbol) {
                token = Token::Operand(self.obtain_char(symbol));
            } else if obelisk.is_match(&symbol) {
                token = Token::Obelisk(self.obtain_char(symbol));
            } else if equal.is_match(&symbol) {
                token = Token::Equal(self.obtain_char(symbol));
            } else if int_keyword.is_match(&symbol) {
                token = Token::Keyword(symbol);
            } else if return_keyword.is_match(&symbol) {
                token = Token::Keyword(symbol);
            } else if identifier.is_match(&symbol) {
                token = Token::Identifier(symbol);
            } else {
                token = Token::Unknown;
            }
            tokens.push(token);
        }

        tokens
    }
}
