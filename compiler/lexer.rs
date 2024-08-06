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
                    self.current_column += 1;
                    if self.input.peek() == Some(&'=') {
                        self.input.next();
                        self.current_column += 1;
                        Token::Equal(self.current_line, self.current_column)
                    } else {
                        Token::Assing(self.current_line, self.current_column)
                    }
                }
                '!' => {
                    self.input.next();
                    self.current_column += 1;
                    if self.input.peek() == Some(&'=') {
                        self.input.next();
                        self.current_column += 1;
                        Token::NotEqual(self.current_line, self.current_column)
                    } else {
                        panic!("Unexpected character after !, line {}", self.current_line);
                    }
                }
                '>' => {
                    self.input.next();
                    self.current_column += 1;
                    if self.input.peek() == Some(&'=') {
                        self.input.next();
                        self.current_column += 1;
                        Token::GreaterEqual(self.current_line, self.current_column)
                    } else {
                        Token::Greater(self.current_line, self.current_column)
                    }
                }
                '<' => {
                    self.input.next();
                    self.current_column += 1;
                    if self.input.peek() == Some(&'=') {
                        self.input.next();
                        self.current_column += 1;
                        Token::LessEqual(self.current_line, self.current_column)
                    } else {
                        Token::Less(self.current_line, self.current_column)
                    }
                }
                '+' => {
                    self.input.next();
                    self.current_column += 1;
                    Token::Plus(self.current_line, self.current_column)
                }
                '-' => {
                    self.input.next();
                    self.current_column += 1;
                    Token::Minus(self.current_line, self.current_column)
                }
                '*' => {
                    self.input.next();
                    self.current_column += 1;
                    Token::Star(self.current_line, self.current_column)
                }
                '/' => {
                    if self.input.peek() == Some(&'/'){
                        self.consume_until_newline();
                        continue;
                    }
                    self.input.next();
                    self.current_column += 1;
                    Token::Slash(self.current_line, self.current_column)
                }
                '(' => {
                    self.input.next();
                    self.current_column += 1;
                    Token::LeftParenthesis(self.current_line, self.current_column)
                }
                ')' => {
                    self.input.next();
                    self.current_column += 1;
                    Token::RightParenthesis(self.current_line, self.current_column)
                }
                '{' => {
                    self.input.next();
                    self.current_column += 1;
                    Token::LeftBraces(self.current_line, self.current_column)
                    }
                '}' => {
                    self.input.next();
                    self.current_column += 1;
                    Token::RightBraces(self.current_line, self.current_column)
                }
                ';' => {
                    self.input.next();
                    self.current_column += 1;
                    Token::Semicolon(self.current_line, self.current_column)
                }
                ',' => {
                    self.input.next();
                    self.current_column += 1;
                    Token::Comma(self.current_line, self.current_column)
                }
                _ => panic!("Unexpected character: {}, line {}", ch, self.current_line),
            };
        }
        Token::EOF(self.current_line, self.current_column)
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
        self.current_column = 0;
    }

    fn lex_identifier_or_keyword(&mut self) -> Token {
        let mut word = String::new();
        while let Some(&ch) = self.input.peek() {
            if ch.is_alphanumeric() || permitted_id_characters().contains(&ch) {
                self.input.next();
                self.current_column += 1;
                word.push(ch);
            } else {
                break;
            }
        }

        match word.as_str() {
            "B1" => Token::B1(self.current_line, self.current_column),
            "B2" => Token::B2(self.current_line, self.current_column),
            "B4" => Token::B4(self.current_line, self.current_column),
            "B8" => Token::B8(self.current_line, self.current_column),
            "B16" => Token::B16(self.current_line, self.current_column),
            "B32" => Token::B32(self.current_line, self.current_column),
            "B64" => Token::B64(self.current_line, self.current_column),
            "B128" => Token::B128(self.current_line, self.current_column),
            "b1" => Token::B1(self.current_line, self.current_column),
            "b2" => Token::B2(self.current_line, self.current_column),
            "b4" => Token::B4(self.current_line, self.current_column),
            "b8" => Token::B8(self.current_line, self.current_column),
            "b16" => Token::B16(self.current_line, self.current_column),
            "b32" => Token::B32(self.current_line, self.current_column),
            "b64" => Token::B64(self.current_line, self.current_column),
            "b128" => Token::B128(self.current_line, self.current_column),
            "function" => Token::Function(self.current_line, self.current_column),
            "if" => Token::If(self.current_line, self.current_column),
            "else" => Token::Else(self.current_line, self.current_column),
            "while" => Token::While(self.current_line, self.current_column),
            "return" => Token::Return(self.current_line, self.current_column),
            "break" => Token::Break(self.current_line, self.current_column),
            "continue" => Token::Continue(self.current_line, self.current_column),
            "print" => Token::Print(self.current_line, self.current_column),
            "true" => Token::True(self.current_line, self.current_column),
            "false" => Token::False(self.current_line, self.current_column),
            _ => Token::Identifier(word, self.current_line, self.current_column),
        }
    }
    
    fn lex_string(&mut self) -> Token {
        self.input.next();
        self.current_column += 1;
        let mut string = String::new();
        while let Some(&ch) = self.input.peek() {
            match ch {
                '"' => {
                    self.input.next();
                    self.current_column += 1;
                    break;
                }
                _ => {
                    string.push(ch);
                    self.input.next();
                    self.current_column += 1;
                }
            }
        }
        Token::StringLiteral(string, self.current_line, self.current_column)
    }

    fn lex_number(&mut self) -> Token {
        let mut word = String::new();
        while let Some(&ch) = self.input.peek() {
            if ch.is_digit(10) {
                self.input.next();
                self.current_column += 1;
                word.push(ch);
            } else {
                break;
            }
        }
        Token::Number(word.parse().unwrap(), self.current_line, self.current_column)
    }
}
