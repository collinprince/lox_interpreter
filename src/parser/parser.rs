use crate::lexer::token::{Token, TokenKind};
use crate::syntax::expr::{BinaryExpr, Expr, Literal, LiteralExpr};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

// token parsing
impl Parser {
    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_any(&[TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let op: Token = self.prev().clone();
            let right: Expr = self.comparison();
            expr = Expr::Binary(BinaryExpr::new(Box::new(expr), op, Box::new(right)));
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        // place holder return value to quiet parser
        Expr::Literal(LiteralExpr {
            value: Literal::Num { val: 1.0 },
        })
    }
}

// helpers
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn prev(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    pub fn match_any(&mut self, token_types: &[TokenKind]) -> bool {
        token_types.iter().any(|ttype| {
            if self.check(*ttype) {
                self.advance();
                true
            } else {
                false
            }
        })
    }

    pub fn check(&mut self, token_type: TokenKind) -> bool {
        !self.is_at_end() && self.peek().kind == token_type
    }

    pub fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.prev()
    }

    pub fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::EOF
    }
}
