use core::fmt;
use std::io::Error;
pub struct Lexer<'a>{
    whole: &'a str,
    rest: &'a str
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub origin: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    BangEqual,
    EqualEqual,
    LessEqual,
    GreaterEqual,
    Less,
    Greater,
    Slash,
    Bang,
    Equal,
    String,
    Ident,
    Number(f64),
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let origin = self.origin.clone();
        match self.kind {
            TokenKind::LeftParen => write!(f,"LEFT_PAREN {origin} null"),
            TokenKind::RightParen => write!(f,"RIGHT_PAREN {origin} null"),
            TokenKind::LeftBrace => write!(f,"LEFT_BRACE {origin} null"),
            TokenKind::RightBrace => write!(f,"RIGHT_BRACE {origin} null"),
            TokenKind::Comma =>write!(f," COMMA {origin} null"),
            TokenKind::Dot => write!(f," DOT {origin} null"),
            TokenKind::Minus => write!(f," MINUS {origin} null"),
            TokenKind::Plus => write!(f," PLUS {origin} null"),
            TokenKind::Semicolon => write!(f," SEMICOLON {origin} null"),
            TokenKind::Star => write!(f," STAR {origin} null"),
            TokenKind::BangEqual => write!(f," BANG_EQUAL {origin} null"),
            TokenKind::EqualEqual => write!(f," EQUAL_EQUAL {origin} null"),
            TokenKind::LessEqual => write!(f," LESS_EQUAL {origin} null"),
            TokenKind::GreaterEqual => write!(f," GREATER_EQUAL {origin} null"),
            TokenKind::Less => write!(f," LESS {origin} null"),
            TokenKind::Greater => write!(f," GREATER {origin} null"),
            TokenKind::Slash => write!(f," SLASH {origin} null"),
            TokenKind::Bang => write!(f," BANG {origin} null"),
            TokenKind::Equal =>  write!(f," Equal {origin} null"),
            TokenKind::String => todo!(),
            TokenKind::Ident => todo!(),
            TokenKind::Number(val) => todo!(),
            TokenKind::And => write!(f, "AND {origin} null"),
            TokenKind::Class => write!(f, "CLASS {origin} null"),
            TokenKind::Else => write!(f, "ELSE {origin} null"),
            TokenKind::False => write!(f, "FALSE {origin} null"),
            TokenKind::For => write!(f, "FOR {origin} null"),
            TokenKind::Fun => write!(f, "FUN {origin} null"),
            TokenKind::If => write!(f, "IF {origin} null"),
            TokenKind::Nil => write!(f, "NIL {origin} null"),
            TokenKind::Or => write!(f, "OR {origin} null"),
            TokenKind::Print => write!(f, "PRINT {origin} null"),
            TokenKind::Return => write!(f, "RETURN {origin} null"),
            TokenKind::Super => write!(f, "SUPER {origin} null"),
            TokenKind::This => write!(f, "THIS {origin} null"),
            TokenKind::True => write!(f, "TRUE {origin} null"),
            TokenKind::Var => write!(f, "VAR {origin} null"),
            TokenKind::While => write!(f, "WHILE {origin} null"),
        }
    }
}

// impl<'de> Iterator for Lexer<'de>{
//     type Item = Result<Token<'de>, Error>;

//     fn next(&mut self) -> Option<Self::Item> {
        
//     }
// }

enum Started {
    Slash,
    String,
    Number,
    Ident,
    Symbol(Token),
    IfEqualElse(TokenKind, TokenKind),
}

impl<'a> Lexer<'a> {
    pub fn lex(input: &str) -> i32 {
        let mut result = 0;
        let mut line_number = 1;
        let mut chars = input.chars().peekable(); // Create a peekable iterator
        
        let just = |kind: TokenKind,c:String| {
            let token = Token {
                kind,
                origin:c
            };
            Started::Symbol(token)
        };


        while let Some(char) = chars.next() {
            let token = match char {
                '(' => just(TokenKind::LeftParen,char.to_string()),       
                ')' => just(TokenKind::RightParen,char.to_string()),
                '{' => just(TokenKind::LeftBrace,char.to_string()),
                '}' => just(TokenKind::RightBrace,char.to_string()),
                ',' => just(TokenKind::Comma,char.to_string()),
                '.' => just(TokenKind::Dot,char.to_string()),
                '-' => just(TokenKind::Minus,char.to_string()),
                '+' => just(TokenKind::Plus,char.to_string()),
                ';' => just(TokenKind::Semicolon,char.to_string()),
                '*' => just(TokenKind::Star,char.to_string()),
                '!' => Started::IfEqualElse(TokenKind::BangEqual, TokenKind::Bang),
                '=' => Started::IfEqualElse(TokenKind::EqualEqual, TokenKind::Equal),
                '>' => Started::IfEqualElse(TokenKind::GreaterEqual, TokenKind::Greater),
                '<' => Started::IfEqualElse(TokenKind::LessEqual, TokenKind::Less),
                '/' => Started::Slash,
                '"' => Started::String,
                '0'..='9'=>{
                    //number
                    todo!()
                },
                'a'..='z' | '_' =>{
                    //Ident
                    todo!()
                }
                '\n' => {
                    line_number += 1;
                    todo!()
                },
                c => {
                    eprintln!("[line {}] Error: Unexpected character: {}", line_number, c);
                    result = 65;
                    todo!()
                }
            };
            
            let token = match token{
                Started::Slash => todo!(),
                Started::String => todo!(),
                Started::Number => todo!(),
                Started::Ident => todo!(),
                Started::Symbol(tk) => tk, 
                Started::IfEqualElse(yes, no) => todo!(),
            };
            
            println!("{token}");
           
        }

        println!("EOF  null");
        result
    }
}
