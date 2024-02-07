use crate::parser::Parser;
use crate::token::TokenType;
use crate::lexer::Lexer;
use crate::evaluator::*;
use std::fs;
use std::io;

pub fn start_interactive() {

    
    loop {
        println!(">> ");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        user_input = user_input.trim().to_string();
        let mut monkey_lexer = Lexer::new(user_input);
        // create_tokens(&mut monkey_lexer);
        let mut monkey_parser = Parser::new(&mut monkey_lexer);
        let program = monkey_parser.parse_programm();
    }
    
}

pub fn start(mut args: impl Iterator<Item = String>) {

    args.next();
    let filepath = "test.txt";
    // let filepath = match args.next() {
    //     Some(x) => x,
    //     None => panic!("Error: could not open monkeyfile"),
    // };
    let mut monkey_lexer = match fs::read_to_string(filepath) {
        Ok(l) => Lexer::new(l),
        Err(err) => panic!("Could not create lexer. Error: {}", err),
    };
    println!("{}", monkey_lexer.input);
    //create_tokens(&mut monkey_lexer);
    let mut monkey_parser = Parser::new(&mut monkey_lexer);
    let program = monkey_parser.parse_programm();
    let eval_program = eval(program.unwrap());
    println!("{:#?}", eval_program);

}

fn create_tokens(monkey_lexer: &mut Lexer) {
    loop {
        let tok = monkey_lexer.next_token();
        match tok.tokentype {
            TokenType::EOF => {println!("{:?}", tok); break;},
            _ => println!("{:?}", tok)
        }
    }
}