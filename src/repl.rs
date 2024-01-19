use crate::token::TokenType;
use crate::lexer::Lexer;
use std::fs;

pub fn start_interactive() {
    println!(">> ");
    
}

pub fn start(mut args: impl Iterator<Item = String>) {

    args.next();

    let filepath = match args.next() {
        Some(x) => x,
        None => panic!("Error: could not open monkeyfile"),
    };

    let mut monkey_lexer = match fs::read_to_string(filepath) {
        Ok(l) => Lexer::new(l),
        Err(err) => panic!("Could not create lexer. Error: {}", err),
    };

    loop {
        let tok = monkey_lexer.next_token();
        match tok.tokentype {
            TokenType::EOF => break,
            _ => println!("{:?}", tok)
        }
    } 
}