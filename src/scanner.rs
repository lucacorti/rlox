pub mod token;

use std::str::Chars;
use token::*;

pub struct Scanner<'a> {
    peeked: Option<char>,
    source: &'a String,
    chars: Chars<'a>,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a String) -> Self {
        Scanner {
            source,
            chars: source.chars(),
            peeked: None,
            tokens: Vec::new() as Vec<Token>,
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn scan_tokens(&mut self) {
        self.start = self.current;
        while self.current < self.source.len() as u32 {
            self.scan_token()
        }
        self.add_token(TokenType::EOF, None);
    }

    fn scan_token(&mut self) {
        match self.next_char() {
            Some('(') => self.add_token(TokenType::LeftParen, None),
            Some(')') => self.add_token(TokenType::RightParen, None),
            Some('{') => self.add_token(TokenType::LeftBrace, None),
            Some('}') => self.add_token(TokenType::RightBrace, None),
            Some(',') => self.add_token(TokenType::Comma, None),
            Some('.') => self.add_token(TokenType::Dot, None),
            Some('-') => self.add_token(TokenType::Minus, None),
            Some('+') => self.add_token(TokenType::Plus, None),
            Some(';') => self.add_token(TokenType::Semicolon, None),
            Some('*') => self.add_token(TokenType::Star, None),
            Some('!') => match self.peek_char() {
                Some('=') => self.add_token(TokenType::BangEqual, None),
                Some(_) => self.add_token(TokenType::Bang, None),
                None => (),
            },
            Some('=') => match self.peek_char() {
                Some('=') => self.add_token(TokenType::EqualEqual, None),
                Some(_) => self.add_token(TokenType::Equal, None),
                None => (),
            },
            Some('<') => match self.peek_char() {
                Some('=') => self.add_token(TokenType::LessEqual, None),
                Some(_) => self.add_token(TokenType::Less, None),
                None => (),
            },
            Some('>') => match self.peek_char() {
                Some('=') => self.add_token(TokenType::GreaterEqual, None),
                Some(_) => self.add_token(TokenType::Greater, None),
                None => (),
            },
            Some('/') => match self.peek_char() {
                Some('/') => loop {
                    match self.next_char() {
                        Some('\n') => break,
                        _ => continue,
                    }
                },
                Some(_) => self.add_token(TokenType::Slash, None),
                None => (),
            },
            Some('"') => self.scan_string(),
            Some(' ') => self.scan_token(),
            Some('\r') => self.scan_token(),
            Some('\t') => self.scan_token(),
            Some('\n') => self.scan_token(),
            Some('0'..='9') => self.scan_number(),
            Some('A'..='Z') => self.scan_identifier(),
            Some('a'..='z') => self.scan_identifier(),
            Some('_') => self.scan_identifier(),
            Some(char) => {
                let line = self.line;
                println!("[line {line}] error: unexpected char {char}")
            }
            None => (),
        };
    }

    fn scan_identifier(&mut self) {
        let start = (self.current - 1) as usize;
        let mut end: usize = 1;
        loop {
            match self.next_char() {
                Some('A'..='Z') => end += 1,
                Some('a'..='z') => end += 1,
                Some('_') => end += 1,
                Some(_char) => break,
                None => break,
            }
        }

        match self.source[start..start + end].as_ref() {
            "and" => self.add_token(TokenType::And, None),
            "class" => self.add_token(TokenType::Class, None),
            "else" => self.add_token(TokenType::Else, None),
            "false" => self.add_token(TokenType::False, None),
            "for" => self.add_token(TokenType::For, None),
            "fun" => self.add_token(TokenType::Fun, None),
            "if" => self.add_token(TokenType::If, None),
            "nil" => self.add_token(TokenType::Nil, None),
            "or" => self.add_token(TokenType::Or, None),
            "print" => self.add_token(TokenType::Print, None),
            "return" => self.add_token(TokenType::Return, None),
            "super" => self.add_token(TokenType::Super, None),
            "this" => self.add_token(TokenType::This, None),
            "true" => self.add_token(TokenType::True, None),
            "var" => self.add_token(TokenType::Var, None),
            "while" => self.add_token(TokenType::While, None),
            identifier => self.add_token(TokenType::Identifier, Some(identifier.to_string())),
        }
    }

    fn scan_number(&mut self) {
        let start = self.current as usize;
        let mut end: usize = 1;
        loop {
            match self.next_char() {
                Some('0'..='9') => end += 1,
                Some('.') => end += 1,
                Some(_char) => break,
                None => break,
            }
        }

        self.add_token(
            TokenType::Number,
            Some(self.source[start..start + end].to_string()),
        );
    }

    fn scan_string(&mut self) {
        let start = self.current as usize;
        let mut end: usize = 0;
        loop {
            match self.next_char() {
                Some('"') => break,
                Some(_char) => end += 1,
                None => continue,
            }
        }
        self.add_token(
            TokenType::String,
            Some(self.source[start..start + end].to_string()),
        );
    }

    fn add_token(&mut self, token_type: TokenType, lexeme: Option<String>) {
        let token = Token::new(token_type, lexeme, self.line);
        self.tokens.push(token);
    }

    fn next_char(&mut self) -> Option<char> {
        match self.peeked_or_next_char() {
            Some('\n') => {
                self.line += 1;
                self.current += 1;
                return Some('\n');
            }
            Some(character) => {
                self.current += 1;
                return Some(character);
            }

            None => return None,
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        match self.next_char() {
            None => None,
            next_char => next_char,
        }
    }

    fn peeked_or_next_char(&mut self) -> Option<char> {
        match self.peeked {
            None => return self.chars.next(),
            peeked => {
                self.peeked = None;
                return peeked;
            }
        };
    }
}
