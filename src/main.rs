use monkey::lexer::Lexer;
use monkey::token::{Token, TokenType};


fn main() {
    let mut testlexer = Lexer::new(String::from("let== 400; ,djflad;"));
    let testtoken  = Token::new(TokenType::ASSIGN,String::from("="));
    println!("{:?}", testtoken.tokentype);
    println!("{:?}", testtoken.literal);
    println!("{:?}", testlexer.ch);
    println!("{:?}", testlexer.next_token());
    println!("{:?}", testlexer.next_token());
    println!("{:?}", testlexer.next_token());
    println!("{:?}", testlexer.next_token());
    println!("{:?}", testlexer.next_token());
    println!("{:?}", testlexer.next_token());
    println!("{:?}", testlexer.input);


}
