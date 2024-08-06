pub mod lexer;
pub mod parser;
pub mod constants;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    // Keyword
    B1(usize, usize), B2(usize, usize), B4(usize, usize), B8(usize, usize),
    B16(usize, usize), B32(usize, usize), B64(usize, usize), B128(usize, usize),
    Function(usize, usize), If(usize, usize), Else(usize, usize), While(usize, usize),
    Return(usize, usize), Break(usize, usize), Continue(usize, usize), Print(usize, usize),
    True(usize, usize), False(usize, usize),

    // Symbols
    Assing(usize, usize), Equal(usize, usize), NotEqual(usize, usize), Greater(usize, usize),
    GreaterEqual(usize, usize), Less(usize, usize), LessEqual(usize, usize),
    Plus(usize, usize), Minus(usize, usize), Star(usize, usize), Slash(usize, usize),
    LeftParenthesis(usize, usize), RightParenthesis(usize, usize), LeftBraces(usize, usize),
    RightBraces(usize, usize), Comma(usize, usize), Semicolon(usize, usize),

    // Literals
    Identifier(String, usize, usize), Number(i64, usize, usize), StringLiteral(String, usize, usize),

    EOF(usize, usize),
}

impl Token {
    fn kind(&self) -> TokenType {
        match self {
            Token::B1(_, _) => TokenType::B1,
            Token::B2(_, _) => TokenType::B2,
            Token::B4(_, _) => TokenType::B4,
            Token::B8(_, _) => TokenType::B8,
            Token::B16(_, _) => TokenType::B16,
            Token::B32(_, _) => TokenType::B32,
            Token::B64(_, _) => TokenType::B64,
            Token::B128(_, _) => TokenType::B128,
            Token::Function(_, _) => TokenType::Function,
            Token::If(_, _) => TokenType::If,
            Token::Else(_, _) => TokenType::Else,
            Token::While(_, _) => TokenType::While,
            Token::Return(_, _) => TokenType::Return,
            Token::Break(_, _) => TokenType::Break,
            Token::Continue(_, _) => TokenType::Continue,
            Token::Print(_, _) => TokenType::Print,
            Token::True(_, _) => TokenType::True,
            Token::False(_, _) => TokenType::False,
            Token::Assing(_, _) => TokenType::Assing,
            Token::Equal(_, _) => TokenType::Equal,
            Token::NotEqual(_, _) => TokenType::NotEqual,
            Token::Greater(_, _) => TokenType::Greater,
            Token::GreaterEqual(_, _) => TokenType::GreaterEqual,
            Token::Less(_, _) => TokenType::Less,
            Token::LessEqual(_, _) => TokenType::LessEqual,
            Token::Plus(_, _) => TokenType::Plus,
            Token::Minus(_, _) => TokenType::Minus,
            Token::Star(_, _) => TokenType::Star,
            Token::Slash(_, _) => TokenType::Slash,
            Token::LeftParenthesis(_, _) => TokenType::LeftParenthesis,
            Token::RightParenthesis(_, _) => TokenType::RightParenthesis,
            Token::LeftBraces(_, _) => TokenType::LeftBraces,
            Token::RightBraces(_, _) => TokenType::RightBraces,
            Token::Comma(_, _) => TokenType::Comma,
            Token::Semicolon(_, _) => TokenType::Semicolon,
            Token::Identifier(_, _, _) => TokenType::Identifier,
            Token::Number(_, _, _) => TokenType::Number,
            Token::StringLiteral(_, _, _) => TokenType::StringLiteral,
            Token::EOF(_, _) => TokenType::EOF,
        }
    }

    fn position(&self) -> (usize, usize) {
        match self {
            Token::B1(line, col) | Token::B2(line, col) | Token::B4(line, col) | Token::B8(line, col) |
            Token::B16(line, col) | Token::B32(line, col) | Token::B64(line, col) | Token::B128(line, col) |
            Token::Function(line, col) | Token::If(line, col) | Token::Else(line, col) | Token::While(line, col) |
            Token::Return(line, col) | Token::Break(line, col) | Token::Continue(line, col) | Token::Print(line, col) |
            Token::True(line, col) | Token::False(line, col) | Token::Assing(line, col) | Token::Equal(line, col) |
            Token::NotEqual(line, col) | Token::Greater(line, col) | Token::GreaterEqual(line, col) | Token::Less(line, col) |
            Token::LessEqual(line, col) | Token::Plus(line, col) | Token::Minus(line, col) | Token::Star(line, col) |
            Token::Slash(line, col) | Token::LeftParenthesis(line, col) | Token::RightParenthesis(line, col) |
            Token::LeftBraces(line, col) | Token::RightBraces(line, col) | Token::Comma(line, col) | Token::Semicolon(line, col) |
            Token::Identifier(_, line, col) | Token::Number(_, line, col) | Token::StringLiteral(_, line, col) | 
            Token::EOF(line, col) => (*line, *col),
        }
    }

    fn text_value(&self) -> String {
        match self {
            Token::Identifier(value, _, _) => value.clone(),
            Token::StringLiteral(value, _, _) => value.clone(),
            _ => panic!("LMAO"),
        }
    }

    fn number_value(&self) -> i64 {
        match self {
            Token::Number(value, _, _) => value.clone(),
            _ => panic!("LMAO"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    B1,
    B2,
    B4,
    B8,
    B16,
    B32,
    B64,
    B128,
    Function,
    If,
    Else,
    While,
    Return,
    Break,
    Continue,
    Print,
    True,
    False,
    Assing,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Plus,
    Minus,
    Star,
    Slash,
    LeftParenthesis,
    RightParenthesis,
    LeftBraces,
    RightBraces,
    Comma,
    Semicolon,
    Identifier,
    Number,
    StringLiteral,
    EOF,
}

pub use self::lexer::Lexer;
pub use self::parser::Parser;
