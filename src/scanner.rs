use std::char;
use crate::token::{Literal, Token, TokenType, KEYWORDS};

pub struct Scanner {
    pub source: Vec<char>,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: u32,
}

impl Scanner {
    pub fn new(source_code: &str) -> Scanner {
        return Scanner {
            source: source_code.chars().collect(),
            tokens: Vec::<Token>::new(),
            start: 0,
            current: 0,
            line: 1,
        };
    }
}

impl Scanner {
    pub fn scan_tokens(&mut self) {
        while self.current < self.source.len() {
            self.scan_token();
            self.start = self.current;
        }
        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), None, self.line));
    }

    fn scan_token(&mut self) {
        let c = self.source[self.current];
        self.current += 1;
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '+' => self.add_token(TokenType::Plus, None),
            '-' => self.add_token(TokenType::Minus, None),
            '*' => self.add_token(TokenType::Star, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '!' => {
                if self.is_next('=') {
                    self.add_token(TokenType::BangEqual, None);
                } else {
                    self.add_token(TokenType::Bang, None);
                }
            }
            '=' => {
                if self.is_next('=') {
                    self.add_token(TokenType::EqualEqual, None);
                } else {
                    self.add_token(TokenType::Equal, None);
                }
            }
            '<' => {
                if self.is_next('=') {
                    self.add_token(TokenType::LessEqual, None);
                } else {
                    self.add_token(TokenType::Less, None);
                }
            }
            '>' => {
                if self.is_next('=') {
                    self.add_token(TokenType::GreaterEqual, None);
                } else {
                    self.add_token(TokenType::Greater, None);
                }
            }
            '/' => {
                if self.is_next('/') {
                    while self.source[self.current] != '\n' && self.current < self.source.len() {
                        self.current += 1;
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            '"' => {
                println!("is \"");
                if self.source[self.current].is_alphanumeric() || self.source[self.current] == ' ' {
                    let text = Literal::Str(self.string());
                    self.add_token(TokenType::StringLiteral, Some(text));
                }
            }
            _ => {
                if self.current < self.source.len() {
                    if c.is_alphabetic() {
                        println!("is iden");
                        let identifier: String = self.identifier();
                        if let Some(&keyword_type) = KEYWORDS.get(&identifier) {
                            self.add_token(keyword_type, None);
                        } else {
                            self.add_token(
                                TokenType::Identifier,
                                Some(Literal::Identifier(identifier)),
                            );
                        }
                    } else if c.is_digit(10) {
                        println!("is number");
                        let number = Literal::Number(self.number());
                        self.add_token(TokenType::Number, Some(number));
                    }
                }
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    fn is_next(&mut self, expected: char) -> bool {
        if self.current >= self.source.len() {
            return false;
        }

        if self.source[self.current] == expected {
            self.current += 1;
            return true;
        }
        return false;
    }
}

impl Scanner {
    fn string(&mut self) -> String {
        while !self.is_next('"') {
            if self.current >= self.source.len() {
                panic!("inside scanner.string()");
            }
            self.current += 1;
        }
        return self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
    }

    fn number(&mut self) -> f64 {
        while self.current < self.source.len() && self.source[self.current].is_digit(10) {
            self.current += 1;
        }

        if self.source[self.current] == '.' {
            self.current += 1;
            while self.current < self.source.len() && self.source[self.current].is_digit(10) {
                self.current += 1;
            }
        }
        return self.source[self.start..self.current]
            .iter()
            .collect::<String>() // compiler does not know to store this as String.
            .to_string()
            .trim()
            .parse::<f64>()
            .unwrap();
    }

    fn identifier(&mut self) -> String {
        while self.current < self.source.len() && self.source[self.current].is_alphanumeric() {
            self.current += 1;
        }
        return self.source[self.start..self.current].iter().collect();
    }
}
