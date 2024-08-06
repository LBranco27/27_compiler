use crate::compiler::Token;
use crate::compiler::constants::permitted_id_characters;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    current_line: usize,
    current_column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
            current_line: 1,
            current_column: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(&ch) = self.input.peek() {
            return match ch {
                ' ' | '\t' => {
                    self.consume_whitespace();
                    continue;
                }
                '\n' => {
                    self.consume_newline();
                    continue;
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    return self.lex_identifier_or_keyword();
                }
                '0'..='9' => {
                    return self.lex_number();
                }
                '"' => return self.lex_string(),
                '=' => {
                    self.input.next();
                    if self.input.peek() == Some(&'=') {
                        self.input.next();
                        Token::Equal
                    } else {
                        Token::Assing
                    }
                }
                '!' => {
                    self.input.next();
                    if self.input.peek() == Some(&'=') {
                        self.input.next();
                        Token::NotEqual
                    } else {
                        panic!("Unexpected character after !, line {}", self.current_line);
                    }
                }
                '>' => {
                    self.input.next();
                    if self.input.peek() == Some(&'=') {
                        self.input.next();
                        Token::GreaterEqual
                    } else {
                        Token::Greater
                    }
                }
                '<' => {
                    self.input.next();
                    if self.input.peek() == Some(&'=') {
                        self.input.next();
                        Token::LessEqual
                    } else {
                        Token::Less
                    }
                }
                '+' => {
                    self.input.next();
                    Token::Plus
                }
                '-' => {
                    self.input.next();
                    Token::Minus
                }
                '*' => {
                    self.input.next();
                    Token::Star
                }
                '/' => {
                    if self.input.peek() == Some(&'/'){
                        self.consume_until_newline();
                        continue;
                    }
                    self.input.next();
                    Token::Slash
                }
                '(' => {
                    self.input.next();
                    Token::LeftParenthesis
                }
                ')' => {
                    self.input.next();
                    Token::RightParenthesis
                }
                '{' => {
                    self.input.next();
                    Token::LeftBraces
                    }
                '}' => {
                    self.input.next();
                    Token::RightBraces
                }
                ';' => {
                    self.input.next();
                    Token::Semicolon
                }
                ',' => {
                    self.input.next();
                    Token::Comma
                }
                _ => panic!("Unexpected character: {}, line {}", ch, self.current_line),
            };
        }
        Token::EOF
    }

    fn consume_until_newline(&mut self) {
        while let Some(ch) = self.input.peek() {
            if ch != &'\n' {
                self.input.next();
                self.current_column += 1;
            } else {
                self.consume_newline();
                return
            }
        }
    }

    fn consume_whitespace(&mut self) {
        while let Some(ch) = self.input.peek() {
            if ch.is_whitespace() {
                self.input.next();
                self.current_column += 1;
            } else {
                break;
            }
        }
    }

    fn consume_newline(&mut self) {
        self.input.next();
        self.current_line += 1;
    }

    fn lex_identifier_or_keyword(&mut self) -> Token {
        let mut word = String::new();
        while let Some(&ch) = self.input.peek() {
            if ch.is_alphanumeric() || permitted_id_characters().contains(&ch) {
                self.input.next();
                word.push(ch);
            } else {
                break;
            }
        }

        match word.as_str() {
            "B1" => Token::B1,
            "B2" => Token::B2,
            "B4" => Token::B4,
            "B8" => Token::B8,
            "B16" => Token::B16,
            "B32" => Token::B32,
            "B64" => Token::B64,
            "B128" => Token::B128,
            "b1" => Token::B1,
            "b2" => Token::B2,
            "b4" => Token::B4,
            "b8" => Token::B8,
            "b16" => Token::B16,
            "b32" => Token::B32,
            "b64" => Token::B64,
            "b128" => Token::B128,
            "function" => Token::Function,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "return" => Token::Return,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "print" => Token::Print,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Identifier(word),
        }
    }
    
    fn lex_string(&mut self) -> Token {
        self.input.next();
        let mut string = String::new();
        while let Some(&ch) = self.input.peek() {
            match ch {
                '"' => {
                    self.input.next();
                    break;
                }
                _ => {
                    string.push(ch);
                    self.input.next();
                }
            }
        }
        Token::StringLiteral(string)
    }

    fn lex_number(&mut self) -> Token {
        let mut word = String::new();
        while let Some(&ch) = self.input.peek() {
            if ch.is_digit(10) {
                self.input.next();
                word.push(ch);
            } else {
                break;
            }
        }
        Token::Number(word.parse().unwrap())
    }
}
