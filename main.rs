mod compiler;

use std::fs;
use std::env;

use crate::compiler::{Lexer, Parser, Token};

fn main() -> std::io::Result<()> {
    let file_path = env::args().nth(1).expect("File path missing from command line arguments.");
    let file_content = fs::read_to_string(file_path).expect("Should read file");
    let mut lexer = Lexer::new(&file_content);
    let mut tokens = Vec::new();
    loop{
        let token = lexer.next_token();
        tokens.push(token.clone());
        if token == Token::EOF {
            println!("LEXER ENDED");
            println!("tokens: {:?}", tokens);
            //let mut file = File::create("tokens.lxr")?;
            //file.write_all(tokens);
            break;
        }

        println!("{:?}", token);
    }
    let mut parser = Parser::new(&tokens);
    loop {
        //println!("tokens: {:?}", tokens);
        parser.parse();
        println!("PARSE CONCLUDED");
        return Ok(())
    }
}
