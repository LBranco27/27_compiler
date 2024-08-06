pub mod lexer;
pub mod parser;
pub mod constants;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    // Keyword
    B1, B2, B4, B8, B16, B32, B64, B128,
    Function, If, Else, While, Return, Break, Continue, Print, True, False,

    // Symbols
    Assing, Equal, NotEqual, Greater, GreaterEqual, Less, LessEqual,
    Plus, Minus, Star, Slash, LeftParenthesis, RightParenthesis, LeftBraces,
    RightBraces, Comma, Semicolon,

    // Literals
    Identifier(String),
    Number(i64),
    StringLiteral(String),

    EOF,
}

pub use self::lexer::Lexer;
pub use self::parser::Parser;
