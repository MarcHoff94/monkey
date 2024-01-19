use crate::token::Token;
use crate::token::TokenType;
#[derive(Debug)]
pub struct Lexer {
    pub input: String,
    pub position: usize,
    pub read_position: usize,
    pub ch: char,
}
impl Lexer {
    pub fn new(input: String) -> Lexer {
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
        if !self.reached_eof() {
            match self.input.chars().nth(self.read_position) {
                Some(ch) => self.ch = ch,
                None => ()
            }
        }
        
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> Option<char> {
        if self.reached_eof() {
            None
        } else {
            Some(self.input.chars().nth(self.read_position).unwrap())
        }
    }

    fn read_number(&mut self) -> String  {
        let start_pos: usize = self.position;
        while is_digit(self.ch) && !self.reached_eof() {
            self.read_char()
        }
        let result = match self.input.get(start_pos..self.position) {
            Some(x) => String::from(x),
            None => panic!("tried to read a number and failed")
        };
        result
    }

    fn read_identifier(&mut self) -> String  {
        let start_pos: usize = self.position;
        while is_letter(self.ch) && !self.reached_eof() {
            self.read_char();
        }
        let result = match self.input.get(start_pos..self.position) {
            Some(x) => String::from(x),
            None => panic!("tried to read a identifier and failed")
        };
        result
    }

    fn make_two_char_token(&mut self, one_char_tokentype: TokenType, two_char_tokentype: TokenType, second_char: char) -> Token {

        if self.peek_char().unwrap_or(' ') == second_char {
            let mut literal = String::from(self.ch);
            self.read_char();
            literal.push(self.ch);
            return Token::new(two_char_tokentype, literal)
        } else {
            return Token::new(one_char_tokentype, self.ch.to_string())
        }
    }

    fn eat_whitespaces(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char()
        }
    }

    fn reached_eof(&self) -> bool {
        if self.read_position > self.input.len() {
            true
        } else {
            false
        }
    }

    pub fn next_token(&mut self) -> Token {

        self.eat_whitespaces();

        let tok = match self.ch {
            //hm book uses strings in general here but maybe chars are better for performance, vec<chars> instead of string 
            //maybe not, the vector of chars would definitely use more memory. One char uses four bytes and a element of a string is just 1 byte
            //alternative: vec<u8> ???
            '=' => self.make_two_char_token(TokenType::ASSIGN, TokenType::EQ, '='),
            ';' => Token::new(TokenType::SEMICOLON, self.ch.to_string()),
            '(' => Token::new(TokenType::LPAREN, self.ch.to_string()),
            ')' => Token::new(TokenType::RPAREN, self.ch.to_string()),
            ',' => Token::new(TokenType::COMMA, self.ch.to_string()),
            '+' => self.make_two_char_token(TokenType::PLUS, TokenType::PLUSSELF, '='),
            '-' => self.make_two_char_token(TokenType::MINUS, TokenType::MINUSSELF, '='),
            '!' => self.make_two_char_token(TokenType::BANG, TokenType::NOTEQ, '='),
            '/' => Token::new(TokenType::SLASH, self.ch.to_string()),
            '*' => Token::new(TokenType::ASTERISK, self.ch.to_string()),
            '^' => Token::new(TokenType::POWER, self.ch.to_string()),
            '<' => self.make_two_char_token(TokenType::LT, TokenType::LTEQ, '='),
            '>' => self.make_two_char_token(TokenType::GT, TokenType::GTEQ, '='),
            '{' => Token::new(TokenType::LBRACE, self.ch.to_string()),
            '}' => Token::new(TokenType::RBRACE, self.ch.to_string()),
            _ => {
                if is_letter(self.ch) && !self.reached_eof() {
                    let literal: String = self.read_identifier();
                    return Token::new(TokenType::lookup_keyword(&literal), literal)
                    
                } else if is_digit(self.ch) && !self.reached_eof() {
                    let literal: String = self.read_number();
                    return Token::new(TokenType::INT, literal)
                } else if self.reached_eof() {
                    return Token::new(TokenType::EOF, String::from("eof"))
                } else {
                    Token::new(TokenType::ILLEGAL, self.ch.to_string())
                }
            }
        };
        if self.reached_eof() {
            return Token::new(TokenType::EOF, self.ch.to_string())
        }
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