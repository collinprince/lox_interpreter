pub use crate::lexer::{Literal, Token};

pub enum Expr {
    P1(Literal),
    P2(Unary),
    P3(Binary),
    P4(Grouping),
}

pub enum Grouping {
    P1(Box<Expr>),
}

pub enum Unary {
    P1 { operator: Token, right: Box<Expr> },
}

pub enum Binary {
    P1 {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
}
