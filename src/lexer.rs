use regex::Regex;
use crate::lexer::Token::*;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number,
    Plus,
    Minus,
    Multiply,
    Divide,
    Lparen,
    Rparen,
}

pub struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer { input: input.to_string(), position: 0 }
    }

    fn regexes() -> Vec<(Regex, Token)> {
        vec![
            (Regex::new(r"^(\d+)").unwrap(), Number),
            (Regex::new(r"^\+").unwrap(), Plus),
            (Regex::new(r"^-").unwrap(), Minus),
            (Regex::new(r"^\*").unwrap(), Multiply),
            (Regex::new(r"^/").unwrap(), Divide),
            (Regex::new(r"^\(").unwrap(), Lparen),
            (Regex::new(r"^\)").unwrap(), Rparen),
        ]
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        let slice = &self.input[self.position..self.input.len()];
        for (regex, token_kind) in Lexer::regexes() {
            if regex.is_match(slice) {
                self.position = slice.len();
                return Some(token_kind)
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::{Lexer, Token};

    #[test]
    fn test_arithmetic() {
        let mut lexer = Lexer::new("1+2*3-4/5");
        let tokens = lexer.tokenize();
        println!("{}", lexer.position);
        assert_eq!(
            tokens,
            vec![
                Token::Number,
                Token::Plus,
                Token::Number,
                Token::Multiply,
                Token::Number,
                Token::Minus,
                Token::Number,
                Token::Divide,
                Token::Number,
            ]
        );
    }
}