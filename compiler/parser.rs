use crate::compiler::Token;
use std::iter::Peekable;
use std::slice::Iter;

pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
    on_while: bool,
    on_function: bool,
    symbol_table: Vec<Symbol>, 
}

struct Symbol {
    id: String,
    tp: Token,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser {
            tokens: tokens.iter().peekable(),
            on_while: false,
            on_function: false,
            symbol_table: Vec::<Symbol>::new(), 
        }
    }
    
    pub fn parse(&mut self) {
        while self.tokens.peek() != Some(&&Token::EOF) {
            self.parse_statement();
        }
    }

    fn parse_statement(&mut self) {
        println!("Token to be read: {:?}", self.tokens.peek());
        match self.tokens.peek() {
            Some(Token::B1) | Some(Token::B2) | Some(Token::B4) | Some(Token::B8)
            | Some(Token::B16) | Some(Token::B32) | Some(Token::B64)
            | Some(Token::B128) => {
                self.parse_var_decl();
            }
            Some(Token::Function) => self.parse_func_decl(),
            Some(Token::If) => self.parse_if_stmt(),
            Some(Token::While) => self.parse_while_stmt(),
            Some(Token::Print) => self.parse_print_stmt(),
            Some(Token::Break) => {
                if self.on_while {
                    self.parse_break_stmt();
                } else {
                    panic!("Unexpected break token on statement: {:?}", self.tokens.peek());
                }
            }
            Some(Token::Continue) => {
                if self.on_while {
                    self.parse_continue_stmt();
                } else {
                    panic!("Unexpected continue token on statement: {:?}", self.tokens.peek());
                }
            }
            Some(Token::Return) => {
                if self.on_function {
                    self.parse_return_stmt();
                } else {
                    panic!("Unexpected return token on statement: {:?}", self.tokens.peek());
                }
            }
            Some(Token::Identifier(_)) => self.parse_assign_or_func_call(),
            _ => panic!("Unexpected token on statement: {:?}", self.tokens.peek()),
        }
    }

    fn parse_var_decl(&mut self) {
        //println!("we are here VAR");
        //println!("token: {:?}", self.tokens.peek());
        let var_type = self.tokens.next().unwrap();
        let var_name = self.tokens.next();
        println!("new variable to be declared: {:?}", var_name);
        if let Some(Token::Identifier(name)) = var_name {
            self.symbol_table.push(Symbol{id: name.clone(), tp: var_type.clone()});
            if let Some(Token::Semicolon) = self.tokens.next() {
                return
            } else {
                panic!("Unexpected token in variable declaration: {:?}", self.tokens.peek());
            }
        } else {
            panic!("Unexpected token in variable declaration: {:?}", self.tokens.peek());
        }
    }
    
    fn parse_func_decl(&mut self) {
        self.tokens.next(); // consume 'function'
        if let Some(Token::Identifier(name)) = self.tokens.next() {
            self.symbol_table.push(Symbol{id: name.clone(), tp: Token::Function});
            self.expect(Token::LeftParenthesis);
            self.parse_param_list();
            self.expect(Token::RightParenthesis);
            self.expect(Token::LeftBraces);
            while self.tokens.peek() != Some(&&Token::RightBraces) {
                self.on_function = true;
                self.parse_statement();
                self.on_function = false;
            }
            self.expect(Token::RightBraces);
        } else {
            panic!("Expected identifier after 'function'");
        }
    }

    fn parse_param_list(&mut self) {
        while let Some(token) = self.tokens.peek() {
            match token {
                Token::B1 | Token::B2 | Token::B4 | Token::B8 | Token::B16
                | Token::B32 | Token::B64 | Token::B128 => {
                    self.tokens.next(); // consume type
                    if let Some(Token::Identifier(_)) = self.tokens.next() {
                        if self.tokens.peek() == Some(&&Token::Comma) {
                            self.tokens.next(); // consume ','
                        }
                    } else {
                        panic!("Expected identifier in parameter list");
                    }
                }
                _ => break,
            }
        }
    }
    fn symbol_table_contains(&self, identifier: &str) -> bool {
        self.symbol_table.iter().any(|symbol| symbol.id == identifier)
    }

    fn parse_assign_or_func_call(&mut self) {
        self.check_symbol_table_for_identifier();
        match self.tokens.peek() {
            Some(Token::Assing) => {
                self.tokens.next(); // consume '='
                if let Some(Token::Identifier(_)) = self.tokens.peek(){
                    if self.check_symbol_table_for_type(Token::Function){
                        //println!("beenhere: {:?}", self.tokens.peek());
                        self.parse_func_call();
                    }
                } else {
                    self.parse_expression();
                    self.expect(Token::Semicolon);
                }
            }
            Some(Token::LeftParenthesis) => {
                self.tokens.next(); // consume '('
                self.parse_param_list();
                self.expect(Token::RightParenthesis);
                self.expect(Token::Semicolon);
            }
            _ => panic!("Unexpected token after identifier: {:?}", self.tokens.peek()),
        }
    }

    fn parse_func_call(&mut self) {
            self.tokens.next(); // consume '('
            self.parse_func_call_param_list();
            self.expect(Token::RightParenthesis);
            self.expect(Token::Semicolon);
    }
   
    fn parse_func_call_param_list(&mut self) {
        self.expect(Token::LeftParenthesis);
        while let Some(token) = self.tokens.peek() {
            println!("beenhere: {:?}", self.tokens.peek());
            if let Some(Token::RightParenthesis) = self.tokens.peek() {
                break;
            } else if let Some(Token::Identifier(_)) = self.tokens.peek() {
                self.check_symbol_table_for_identifier();
                if self.tokens.peek() == Some(&&Token::Comma) {
                    self.tokens.next(); // consume ','
                }
            } else {
                panic!("Expected identifier in parameter list");
            }
        }
    }

    fn parse_if_stmt(&mut self) {
        self.tokens.next(); // consume if
        self.expect(Token::LeftParenthesis);
        self.parse_expression();
        self.expect(Token::RightParenthesis);
        self.expect(Token::LeftBraces);
        while self.tokens.peek() != Some(&&Token::RightBraces) {
            self.parse_statement();
        }
        self.expect(Token::RightBraces);
        if self.tokens.peek() == Some(&&Token::Else) {
            self.tokens.next();
            self.expect(Token::LeftBraces);
            while self.tokens.peek() != Some(&&Token::RightBraces) {
                self.parse_statement();
            }
            self.expect(Token::RightBraces);

        }
    }

    fn parse_while_stmt(&mut self) {
        self.tokens.next(); // consume while
        self.expect(Token::LeftParenthesis);
        self.parse_expression();
        self.expect(Token::RightParenthesis);
        self.expect(Token::LeftBraces);
        while self.tokens.peek() != Some(&&Token::RightBraces) {
            self.on_while = true;
            self.parse_statement();
            self.on_while = false;
        }
        self.expect(Token::RightBraces);
    }

    fn parse_print_stmt(&mut self) {
        self.tokens.next(); // consume print
        self.expect(Token::LeftParenthesis);
        self.parse_expression();
        self.expect(Token::RightParenthesis);
        self.expect(Token::Semicolon);
    }

    fn parse_break_stmt(&mut self) {
        self.tokens.next(); // consume break
        self.expect(Token::Semicolon);
    }

    fn parse_continue_stmt(&mut self) {
        self.tokens.next(); // consume continue
        self.expect(Token::Semicolon);
    }

    fn parse_return_stmt(&mut self) {
        self.tokens.next(); // consume return
        if self.tokens.peek() != Some(&&Token::Semicolon) {
            self.parse_expression();
        }
        self.expect(Token::Semicolon);
    }

    fn parse_expression(&mut self) {
        self.parse_arith_expr();
    }

    fn parse_arith_expr(&mut self) {
        self.parse_term();
        while let Some(&token) = self.tokens.peek() {
            match token {
                Token::Plus | Token::Minus => {
                    self.tokens.next(); // consume operator
                    self.parse_term();
                }
                Token::True | Token::False => break,
                _ => break,
            }
        }
    }

    fn parse_term(&mut self) {
        self.parse_factor();
        while let Some(&token) = self.tokens.peek() {
            match token {
                Token::Star | Token::Slash => {
                    self.tokens.next(); // consume operator
                    self.parse_factor();
                }
                _ => break,
            }
        }
    }

    fn parse_factor(&mut self) {
        match self.tokens.next() {
            Some(Token::Number(_)) => {}
            Some(Token::Identifier(_)) => {}
            Some(Token::LeftParenthesis) => {
                self.parse_arith_expr();
                self.expect(Token::RightParenthesis);
            }
            token => panic!("Unexpected token in factor: {:?}", token),
        }
    }

    fn check_symbol_table_for_type(&mut self, tp: Token) -> bool {
        self.symbol_table.iter().any(|symbol| symbol.tp == tp)
    }

    fn check_symbol_table_for_identifier(&mut self) {
        if let Some(Token::Identifier(identifier)) = self.tokens.next(){
            if !self.symbol_table_contains(&identifier){
                panic!("Identifier not declared previously: {:?}", identifier);
            }
        }
    }

    fn expect(&mut self, expected: Token) {
        match self.tokens.next() {
            Some(token) if *token == expected => {
                println!("Token expected and found: {:?}", token);
            }
            token => panic!("Expected {:?}, found {:?}", expected, token),
        }
    }
}
