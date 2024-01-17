
pub struct token<'a> {
    tokentype: TokenType,
    literal: &'a str,
}
pub enum TokenType {
    ILLEGAL,
    EOF,

    //Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    LT,
    GT,

    EQ,
    NOTEQ,

    //Delimiters

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    //Keywords

    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

pub struct lexer {
    input: String,
    position: i32,
    read_Position: i32,
    ch: char,
}
impl lexer {
    fn new(&mut self, input: String){
        self.input = input;
    }

    fn read_char(&mut self) {
        self.ch = char::from(self.input.bytes().next().unwrap_or_else(|| 0))
    }
}