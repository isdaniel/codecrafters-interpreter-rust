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
            Expr::String(s) => write!(f,"{s:?}"),
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

    pub fn parse_expression_within(&mut self)  {
        let mut exprs = Vec::new();
        loop {
            let Some((t)) = self.lexer.next() else { break; };
            match t {
                Ok(token) => {
                    let expr = match token.kind{
                        TokenKind::LeftParen => todo!(),
                        TokenKind::RightParen => todo!(),
                        TokenKind::LeftBrace => todo!(),
                        TokenKind::RightBrace => todo!(),
                        TokenKind::Comma => todo!(),
                        TokenKind::Dot => todo!(),
                        TokenKind::Minus => todo!(),
                        TokenKind::Plus =>  todo!(),
                        TokenKind::Semicolon => todo!(),
                        TokenKind::Star => todo!(),
                        TokenKind::BangEqual => todo!(),
                        TokenKind::EqualEqual => todo!(),
                        TokenKind::LessEqual => todo!(),
                        TokenKind::GreaterEqual => todo!(),
                        TokenKind::Less => todo!(),
                        TokenKind::Greater => todo!(),
                        TokenKind::Slash => todo!(),
                        TokenKind::Bang => todo!(),
                        TokenKind::Equal => todo!(),
                        TokenKind::String => todo!(),
                        TokenKind::Ident => todo!(),
                        TokenKind::Number(val) => Expr::Number(val),
                        TokenKind::And => todo!(),
                        TokenKind::Class => todo!(),
                        TokenKind::Else => todo!(),
                        TokenKind::False => Expr::Bool(false),
                        TokenKind::True => Expr::Bool(true),
                        TokenKind::For => todo!(),
                        TokenKind::Fun => todo!(),
                        TokenKind::If => todo!(),
                        TokenKind::Nil => Expr::Nil,
                        TokenKind::Or => todo!(),
                        TokenKind::Print => todo!(),
                        TokenKind::Return => todo!(),
                        TokenKind::Super => todo!(),
                        TokenKind::This => todo!(),
                        TokenKind::Var => todo!(),
                        TokenKind::While => todo!(),
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

