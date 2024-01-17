fn main() {
    let mut testlexer = lexer::new(String::from("let ; ,djflad;"));
    println!("{:?}", testlexer.ch);
    println!("{:?}", testlexer.nextToken());
    println!("{:?}", testlexer.nextToken());
    println!("{:?}", testlexer.nextToken());
    println!("{:?}", testlexer.nextToken());
    println!("{:?}", testlexer.nextToken());
    println!("{:?}", testlexer.input);

}

#[derive(Debug)]
pub struct token{
    tokentype: TokenType,
    literal: String,
}
impl token {
    fn new(Type: TokenType, literal: String) -> token {
        token {
            tokentype: Type,
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
    fn lookupKeyword(keyword: &str) -> TokenType {
        let result = match keyword {
            "function" => TokenType::FUNCTION,
            "let" => TokenType::LET,
            _ => TokenType::IDENT,
        };
        return result
    }
}
#[derive(Debug)]
pub struct lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}
impl lexer {
    fn new(input: String) -> lexer {
        let mut l = lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: char::from(0),
        };
        l.read_char();
        return l
        
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = char::from(0)
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap_or(char::from(0));
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

    fn nextToken(&mut self) -> token {
        
        self.eat_whitespaces();

        let tok = match self.ch {
            '=' => token::new(TokenType::ASSIGN, self.ch.to_string()),
            ';' => token::new(TokenType::SEMICOLON, self.ch.to_string()),
            '(' => token::new(TokenType::LPAREN, self.ch.to_string()),
            ')' => token::new(TokenType::RPAREN, self.ch.to_string()),
            ',' => token::new(TokenType::COMMA, self.ch.to_string()),
            '+' => token::new(TokenType::PLUS, self.ch.to_string()),
            '{' => token::new(TokenType::LBRACE, self.ch.to_string()),
            '}' => token::new(TokenType::RBRACE, self.ch.to_string()),
            '0' => token::new(TokenType::EOF, self.ch.to_string()),
            _ => {
                if is_letter(self.ch) {
                    let literal: String = self.read_identifier();
                    return token::new(TokenType::lookupKeyword(&literal), literal)
                    
                } else {
                    token::new(TokenType::ILLEGAL, String::from("0"))
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