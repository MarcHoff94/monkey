fn main() {
    let mut testlexer = Lexer::new(String::from("let 4; ,djflad;"));
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

#[derive(Debug)]
pub struct Token{
    tokentype: TokenType,
    literal: String,
}
impl Token {
    fn new(tok_type: TokenType, literal: String) -> Token {
        Token {
            tokentype: tok_type,
            literal: literal,
        }
    }
}
#[derive(Debug)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    //Identifiers + literals
    IDENT,
    INT,

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
impl TokenType {
    fn lookup_keyword(keyword: &str) -> TokenType {
        let result = match keyword {
            "function" => TokenType::FUNCTION,
            "let" => TokenType::LET,
            "true" => TokenType::TRUE,
            "false" => TokenType::FALSE,
            "if" => TokenType::IF,
            "else" => TokenType::ELSE,
            "return" => TokenType::RETURN,
            _ => TokenType::IDENT,
        };
        return result
    }
}
#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}
impl Lexer {
    fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: '0',
        };
        l.read_char();
        return l
        
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '0'
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap_or('0');
        }
        
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String  {
        let start_pos: usize = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        let result = match self.input.get(start_pos..self.position) {
            Some(x) => String::from(x),
            None => panic!("Error empty String")
        };
        result
    }

    fn eat_whitespaces(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char()
        }
    }

    fn next_token(&mut self) -> Token {

        self.eat_whitespaces();

        let tok = match self.ch {
            //hm book uses strings in general here but maybe chars are better for performance, vec<chars> instead of string 
            //maybe not, the vector of chars would definitely use more memory. One char uses four bytes and a element of a string is just 1 byte
            //alternative: vec<u8> ???
            '=' => Token::new(TokenType::ASSIGN, self.ch.to_string()), 
            ';' => Token::new(TokenType::SEMICOLON, self.ch.to_string()),
            '(' => Token::new(TokenType::LPAREN, self.ch.to_string()),
            ')' => Token::new(TokenType::RPAREN, self.ch.to_string()),
            ',' => Token::new(TokenType::COMMA, self.ch.to_string()),
            '+' => Token::new(TokenType::PLUS, self.ch.to_string()),
            '-' => Token::new(TokenType::MINUS, self.ch.to_string()),
            '{' => Token::new(TokenType::LBRACE, self.ch.to_string()),
            '}' => Token::new(TokenType::RBRACE, self.ch.to_string()),
            '0' => Token::new(TokenType::EOF, self.ch.to_string()),
            _ => {
                if is_letter(self.ch) {
                    let literal: String = self.read_identifier();
                    return Token::new(TokenType::lookup_keyword(&literal), literal)
                    
                }else if is_digit(self.ch) {
                    Token::new(TokenType::INT, String::from(self.ch))
                } else {
                    Token::new(TokenType::ILLEGAL, String::from("0"))
                }
            }
        };
        self.read_char();
        tok
    }
}
pub fn is_letter(byte: char) -> bool {
    'a' <= byte && byte <= 'z' || 'A' <= byte && byte <= 'Z' || byte == '_'
}

pub fn is_digit(byte:char) -> bool {
    '0' <= byte && byte <= '9'
}