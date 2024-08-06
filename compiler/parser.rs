use crate::compiler::{Token, TokenType};
use std::iter::Peekable;
use std::slice::Iter;

pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
    on_while: bool,
    on_function: bool,
    symbol_table: Vec<Symbol>, 
    last_expect_line: usize,
    last_expect_column: usize,
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
            last_expect_line: 1,
            last_expect_column: 0,
        }
    }
    
    pub fn parse(&mut self) {
        while self.tokens.peek().unwrap().kind() != TokenType::EOF {
            self.parse_statement();
        }
    }

    fn parse_statement(&mut self) {
        println!("Token to be read: {:?}", self.tokens.peek());
        match self.tokens.peek() {
            Some(Token::B1(_,_)) | Some(Token::B2(_,_)) | Some(Token::B4(_,_))
            | Some(Token::B8(_,_)) | Some(Token::B16(_,_))
            | Some(Token::B32(_,_)) | Some(Token::B64(_,_))
            | Some(Token::B128(_,_)) => {
                self.parse_var_decl();
            }
            Some(Token::Function(_,_)) => self.parse_func_decl(),
            Some(Token::If(_,_)) => self.parse_if_stmt(),
            Some(Token::While(_,_)) => self.parse_while_stmt(),
            Some(Token::Print(_,_)) => self.parse_print_stmt(),
            Some(Token::Break(_,_)) => {
                if self.on_while {
                    self.parse_break_stmt();
                } else {
                    panic!("Unexpected break token on statement: {:?}", self.tokens.peek());
                }
            }
            Some(Token::Continue(_,_)) => {
                if self.on_while {
                    self.parse_continue_stmt();
                } else {
                    panic!("Unexpected continue token on statement: {:?}", self.tokens.peek());
                }
            }
            Some(Token::Return(_,_)) => {
                if self.on_function {
                    self.parse_return_stmt();
                } else {
                    panic!("Unexpected return token on statement: {:?}", self.tokens.peek());
                }
            }
            Some(Token::Identifier(_,_,_)) => self.parse_assign_or_func_call(),
            _ => panic!("Unexpected token on statement: {:?}", self.tokens.peek()),
        }
    }

    fn parse_var_decl(&mut self) {
        //println!("we are here VAR");
        //println!("token: {:?}", self.tokens.peek());
        let var_type = self.tokens.next().unwrap();
        let var_name = self.tokens.next();
        println!("new variable to be declared: {:?}", var_name);
        if let Some(Token::Identifier(name,_,_)) = var_name {
            self.symbol_table.push(Symbol{id: name.clone(), tp: var_type.clone()});
            if let Some(Token::Semicolon(_,_)) = self.tokens.peek() {
                self.tokens.next();
                return
            } else {
                self.report_error("var_decl");
                panic!("Unexpected token in variable declaration: {:?}", self.tokens.peek());
            }
        } else {
            panic!("Unexpected token in variable declaration: {:?}", self.tokens.peek());
        }
    }
    
    fn parse_func_decl(&mut self) {
        self.tokens.next(); // consume 'function'
        if let Some(Token::Identifier(name,line,column)) = self.tokens.next() {
            self.symbol_table.push(Symbol{id: name.clone(), tp: Token::Function(*line,*column)});
            self.expect(Token::LeftParenthesis(self.last_expect_line,self.last_expect_column));
            self.parse_param_list();
            self.expect(Token::RightParenthesis(self.last_expect_line,self.last_expect_column));
            self.expect(Token::LeftBraces(self.last_expect_line,self.last_expect_column));
            while self.tokens.peek().unwrap().kind() != TokenType::RightBraces {
                self.on_function = true;
                self.parse_statement();
                self.on_function = false;
            }
            self.expect(Token::RightBraces(self.last_expect_line,self.last_expect_column));
        } else {
            panic!("Expected identifier after 'function'");
        }
    }

    fn parse_param_list(&mut self) {
        println!("PARSE");
        while let Some(token) = self.tokens.peek() {
            match token {
                Token::B1(_,_) | Token::B2(_,_) | Token::B4(_,_) | Token::B8(_,_) 
                | Token::B16(_,_) | Token::B32(_,_) | Token::B64(_,_) 
                | Token::B128(_,_) => {
                    self.tokens.next(); // consume type
                    if let Some(Token::Identifier(_,_,_)) = self.tokens.next() {
                        if self.tokens.peek().expect("LMAO").kind() == TokenType::Comma {
                            println!("COMMA CONSUMED");
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
            Some(Token::Assing(_,_)) => {
                self.tokens.next(); // consume '='
                if let Some(Token::Identifier(_,_,_)) = self.tokens.peek(){
                    if self.check_symbol_table_for_type(Token::Function(self.last_expect_line,self.last_expect_column)){
                        //println!("beenhere: {:?}", self.tokens.peek());
                        self.parse_func_call();
                    }
                } else {
                    self.parse_expression();
                    self.expect(Token::Semicolon(self.last_expect_line,self.last_expect_column));
                }
            }
            Some(Token::LeftParenthesis(_,_)) => {
                self.tokens.next(); // consume '('
                self.parse_func_call_param_list();
                self.expect(Token::RightParenthesis(self.last_expect_line,self.last_expect_column));
                self.expect(Token::Semicolon(self.last_expect_line,self.last_expect_column));
            }
            _ => panic!("Unexpected token after identifier: {:?}", self.tokens.peek()),
        }
    }

    fn parse_func_call(&mut self) {
            self.tokens.next(); // consume '('
            self.parse_func_call_param_list();
            self.expect(Token::RightParenthesis(self.last_expect_line,self.last_expect_column));
            self.expect(Token::Semicolon(self.last_expect_line,self.last_expect_column));
    }
   
    fn parse_func_call_param_list(&mut self) {
        //self.expect(Token::LeftParenthesis(self.last_expect_line,self.last_expect_column));
        while let Some(token) = self.tokens.peek() {
            println!("beenhere: {:?}", self.tokens.peek());
            if let Some(Token::RightParenthesis(_,_)) = self.tokens.peek() {
                break;
            } else if let Some(Token::Identifier(_,_,_)) = self.tokens.peek() {
                self.check_symbol_table_for_identifier();
                if self.tokens.peek().unwrap().kind() == TokenType::Comma {
                    self.tokens.next(); // consume ','
                }
            } else {
                panic!("Expected identifier in parameter list");
            }
        }
    }

    fn parse_if_stmt(&mut self) {
        self.tokens.next(); // consume if
        self.expect(Token::LeftParenthesis(self.last_expect_line,self.last_expect_column));
        self.parse_expression();
        self.expect(Token::RightParenthesis(self.last_expect_line,self.last_expect_column));
        self.expect(Token::LeftBraces(self.last_expect_line,self.last_expect_column));
        while self.tokens.peek() != Some(&&Token::RightBraces(self.last_expect_line,self.last_expect_column)) {
            self.parse_statement();
        }
        self.expect(Token::RightBraces(self.last_expect_line,self.last_expect_column));
        if self.tokens.peek() == Some(&&Token::Else(self.last_expect_line,self.last_expect_column)) {
            self.tokens.next();
            self.expect(Token::LeftBraces(self.last_expect_line,self.last_expect_column));
            while self.tokens.peek() != Some(&&Token::RightBraces(self.last_expect_line,self.last_expect_column)) {
                self.parse_statement();
            }
            self.expect(Token::RightBraces(self.last_expect_line,self.last_expect_column));

        }
    }

    fn parse_while_stmt(&mut self) {
        self.tokens.next(); // consume while
        self.expect(Token::LeftParenthesis(self.last_expect_line,self.last_expect_column));
        self.parse_expression();
        self.expect(Token::RightParenthesis(self.last_expect_line,self.last_expect_column));
        self.expect(Token::LeftBraces(self.last_expect_line,self.last_expect_column));
        while self.tokens.peek() != Some(&&Token::RightBraces(self.last_expect_line,self.last_expect_column)) {
            self.on_while = true;
            self.parse_statement();
            self.on_while = false;
        }
        self.expect(Token::RightBraces(self.last_expect_line,self.last_expect_column));
    }

    fn parse_print_stmt(&mut self) {
        self.tokens.next(); // consume print
        self.expect(Token::LeftParenthesis(self.last_expect_line,self.last_expect_column));
        self.parse_expression();
        self.expect(Token::RightParenthesis(self.last_expect_line,self.last_expect_column));
        self.expect(Token::Semicolon(self.last_expect_line,self.last_expect_column));
    }

    fn parse_break_stmt(&mut self) {
        self.tokens.next(); // consume break
        self.expect(Token::Semicolon(self.last_expect_line,self.last_expect_column));
    }

    fn parse_continue_stmt(&mut self) {
        self.tokens.next(); // consume continue
        self.expect(Token::Semicolon(self.last_expect_line,self.last_expect_column));
    }

    fn parse_return_stmt(&mut self) {
        self.tokens.next(); // consume return
        if self.tokens.peek() != Some(&&Token::Semicolon(self.last_expect_line,self.last_expect_column)) {
            self.parse_expression();
        }
        self.expect(Token::Semicolon(self.last_expect_line,self.last_expect_column));
    }

    fn parse_expression(&mut self) {
        self.parse_arith_expr();
    }

    fn parse_arith_expr(&mut self) {
        self.parse_term();
        while let Some(&token) = self.tokens.peek() {
            match token {
                Token::Plus(_,_) | Token::Minus(_,_) => {
                    self.tokens.next(); // consume operator
                    self.parse_term();
                }
                Token::True(_,_) | Token::False(_,_) => break,
                _ => break,
            }
        }
    }

    fn parse_term(&mut self) {
        self.parse_factor();
        while let Some(&token) = self.tokens.peek() {
            match token {
                Token::Star(_,_) | Token::Slash(_,_) => {
                    self.tokens.next(); // consume operator
                    self.parse_factor();
                }
                _ => break,
            }
        }
    }

    fn parse_factor(&mut self) {
        match self.tokens.next() {
            Some(Token::Number(_,_,_)) => {}
            Some(Token::Identifier(_,_,_)) => {}
            Some(Token::LeftParenthesis(_,_)) => {
                self.parse_arith_expr();
                self.expect(Token::RightParenthesis(self.last_expect_line,self.last_expect_column));
            }
            token => panic!("Unexpected token in factor: {:?}", token),
        }
    }

    fn check_symbol_table_for_type(&mut self, tp: Token) -> bool {
        self.symbol_table.iter().any(|symbol| symbol.tp == tp)
    }

    fn check_symbol_table_for_identifier(&mut self) {
        if let Some(Token::Identifier(ref identifier, _, _)) = self.tokens.peek() {
            if !self.symbol_table_contains(identifier) {
                panic!("Identifier '{}' not declared", identifier);
            }
            self.tokens.next();
        } else {
            panic!("Expected identifier, but found {:?}", self.tokens.peek());
        }
    }

    fn expect(&mut self, expected: Token) {
        match self.tokens.next() {
            Some(token) if token.kind() == expected.kind() => {
                println!("Token expected and found: {:?}", token);
                self.last_expect_line = token.position().0;
                self.last_expect_column = token.position().1;
            }
            Some(token) => panic!("Expected {:?}, found {:?}", expected.kind(), token),
            None => panic!("Expected {:?}, but no more tokens", expected),
        }
    }
    fn report_error(&mut self, token_type: &str) {
        let token = self.tokens.peek().unwrap();
        match token_type {
            "var_decl" => {
                panic!("Unexpected {:?} token in variable declaration: {:?} at position {:?}"
                    , token.kind(), token.text_value(), token.position());
            }
            _ => {
                panic!("Unexpected token: {:?}", token)
            }
        }
    }
}
