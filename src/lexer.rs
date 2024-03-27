use std::str::Chars;

use crate::lexer::Token::*;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    Lparen,
    Rparen,
    Id(String),
    Assign,
    If,
}

pub struct Lexer<'a> {
    input: Chars<'a>,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer { input: input.chars(), position: 0 }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        if let Some(char) = self.input.next() {
            self.position += 1;
            return match char {
                '0'..='9' => {
                    let mut total = char.to_digit(10)? as i32;
                    // Look ahead in a clone to see if the next character is also a digit
                    while let Some(ch) = self.input.clone().next() {
                        if let Some(num) = ch.to_digit(10) {
                            total = total * 10 + num as i32;
                            // Consume the character on the real iterator
                            self.input.next();
                        } else {
                            break
                        }
                    }
                    Some(Number(total))
                },
                'i' => {
                    match self.lookahead() {
                        None => None,
                        Some('f') => {
                            // consume the 'f'
                            self.input.next();
                            if self.lookahead_alphabetic() {
                                // read and consume the rest of the string
                                let mut str = self.read_string();
                                str.insert_str(0, "if");
                                Some(Id(str))
                            } else {
                                Some(If)
                            }
                        },
                        _ => {
                            let mut str = self.read_string();
                            str.insert(0, 'i');
                            Some(Id(str))
                        }
                    }
                },
                'a'..='z' => {
                    let mut id = String::new();
                    id.push(char);
                    while let Some(ch) = self.lookahead() {
                        if ch.is_alphabetic() {
                            id.push(ch);
                            self.input.next();
                        } else {
                            break
                        }
                    }
                    Some(Id(id))
                },
                '+' => Some(Plus),
                '-' => Some(Minus),
                '*' => Some(Multiply),
                '/' => Some(Divide),
                '(' => Some(Lparen),
                ')' => Some(Rparen),
                ' ' => self.next_token(),
                '=' => Some(Assign),
                _ => None,
            }
        }
        None
    }

    fn lookahead(&self) -> Option<char> {
        self.input.clone().next()
    }

    fn lookahead_alphabetic(&self) -> bool {
        if let Some(ch) = self.lookahead() {
            ch.is_alphabetic()
        } else {
            false
        }
    }

    fn read_string(&mut self) -> String {
        let mut str = String::new();
        while let Some(char) = self.lookahead() {
            if char.is_alphabetic() {
                str.push(char);
                self.input.next();
            } else {
                break;
            }
        }
        str
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::{Lexer, Token};

    #[test]
    fn test_arithmetic() {
        let mut lexer = Lexer::new("10 +2*(3-4)/5");
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Number(10),
                Token::Plus,
                Token::Number(2),
                Token::Multiply,
                Token::Lparen,
                Token::Number(3),
                Token::Minus,
                Token::Number(4),
                Token::Rparen,
                Token::Divide,
                Token::Number(5),
            ]
        );
    }
    
    #[test]
    fn test_assignment() {
        let mut lexer = Lexer::new("myVar = 10");
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Id("myVar".to_string()),
                Token::Assign,
                Token::Number(10),
            ]
        );
    }

    #[test]
    fn test_if() {
        let mut lexer = Lexer::new("if x = 10");
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::If,
                Token::Id("x".to_string()),
                Token::Assign,
                Token::Number(10),
            ]
        );

        let mut lexer = Lexer::new("ifx = 10");
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Id("ifx".to_string()),
                Token::Assign,
                Token::Number(10),
            ]
        );
    }
}