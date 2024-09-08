use crate::{
    lex::{Token, TokenKind},
    Lexer,
};
use miette::{Error, LabeledSpan, WrapErr};
use std::{borrow::Cow, fmt::{self, Display}};

pub struct Parser<'a> {
    whole: &'a str,
    lexer: Lexer<'a>,
}

pub enum Expr<'a>{
    Bool(bool),
    Nil,
    Number(f64),
    String(&'a str),
    Unary{
        operator: Token<'a>,
        right: Box<Expr<'a>>
    },
    Binary {
        operator: Token<'a>,
        left: Box<Expr<'a>>,
        right: Box<Expr<'a>>,
    },
    //Grouping(Vec<Expr<'a>>),
}

impl<'a> Display for Expr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Bool(b) => write!(f,"{b}"),
            Expr::Nil => f.write_str("nil"),
            Expr::Number(n) => write!(f,"{n:?}"),
            Expr::String(s) => write!(f,"{s}"),
            Expr::Unary { operator, right } => {
                f.write_fmt(format_args!("{} {right}", operator.origin))
            }
            Expr::Binary {
                operator,
                left,
                right,
            } => f.write_fmt(format_args!("({} {left} {right})", operator.origin)),
        }
    }
}

impl<'a> Parser<'a>{
    pub fn new(input: &'a str)-> Self {
        Self{
            whole : input,
            lexer : Lexer::new(input)
        }
    }

    // fn parse_expression(&mut self) -> Expr<'a>{
    //     self.parse_binary_expression(0) 
    // }
    
    fn parse_binary_expression(&mut self, operator:Toke<'a>) -> Expr<'a>{
        let left = self.parse_primary_expression();
        let right = self.parse_primary_expression();
        Expr::Binary{
            operator,
            left: Box::new(left)
            right: Box::new(right)
        }
    }
    
    fn parse_primary_expression(&mut self) -> Expr<'a> {
        match self.lexer.next() {
            Some(Ok(token)) => match token.kind {
                TokenKind::Number(val) => Expr::Number(val),
                TokenKind::String => Expr::String(token.origin.trim_matches('"')),
                TokenKind::True => Expr::Bool(true),
                TokenKind::False => Expr::Bool(false),
                TokenKind::Nil => Expr::Nil,
                TokenKind::LeftParen => {
                    let expr = self.parse_binary_expression(0);
                    if let Some(Ok(Token { kind: TokenKind::RightParen, .. })) = self.lexer.next() {
                        expr
                    } else {
                        panic!("Expected closing parenthesis");
                    }
                }
                _ => panic!("Unexpected token: {:?}", token.kind),
            },
            Some(Err(_)) => panic!("Lexer error"),
            None => panic!("Unexpected end of input"),
        }
    }
    

    pub fn parse_expression_within(&mut self)  {
        let mut exprs = Vec::new();
        while let Some(t) = self.lexer.next(){
            match t {
                Ok(token) => {
                    let expr = match token.kind{
                        TokenKind::LeftParen => self.parse_binary_expression(0),
                        TokenKind::RightParen => todo!(),
                        TokenKind::String => Expr::String(token.origin.trim_matches('"')),
                        TokenKind::Number(val) => Expr::Number(val),
                        TokenKind::False => Expr::Bool(false),
                        TokenKind::True => Expr::Bool(true),
                        TokenKind::Nil => Expr::Nil,
                        _ => todo!(),
                    };
                    exprs.push(expr);
                },
                Err(_) => todo!(),
            }
        }

        for exp in exprs{
            println!("{exp}");
        }
        
    }
}

