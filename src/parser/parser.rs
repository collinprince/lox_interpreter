use crate::lexer::token::{Token, TokenKind};
use crate::syntax::expr::{BinaryExpr, Expr, GroupingExpr, Literal, LiteralExpr, UnaryExpr};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

pub enum ParserError {
    UnexpectedToken(String, Token),
    Test,
}

impl std::fmt::Debug for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedToken(expected_token_msg, actual_token) => {
                write!(
                    f,
                    "Expected {} at line={}, got: {:?}",
                    expected_token_msg, actual_token.line, actual_token
                )
            }
            _ => {
                write!(f, "Test")
            }
        }
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// token parsing
impl Parser {
    pub fn parse(&mut self) -> Result<Expr, ParserError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.comparison()?;

        while self.match_any(&[TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let op: Token = self.prev().clone();
            let right: Expr = self.comparison()?;
            expr = Expr::Binary(BinaryExpr::new(Box::new(expr), op, Box::new(right)));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        // place holder return value to quiet parser
        let mut expr = self.term()?;
        while self.match_any(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let op: Token = self.prev().clone();
            let right: Expr = self.term()?;
            expr = Expr::Binary(BinaryExpr::new(Box::new(expr), op, Box::new(right)));
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.factor()?;
        while self.match_any(&[TokenKind::Plus, TokenKind::Minus]) {
            let op: Token = self.prev().clone();
            let right: Expr = self.factor()?;
            expr = Expr::Binary(BinaryExpr::new(Box::new(expr), op, Box::new(right)));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;
        while self.match_any(&[TokenKind::Slash, TokenKind::Star]) {
            let op: Token = self.prev().clone();
            let right: Expr = self.unary()?;
            expr = Expr::Binary(BinaryExpr::new(Box::new(expr), op, Box::new(right)));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        if self.match_any(&[TokenKind::Bang, TokenKind::Minus]) {
            let op: Token = self.prev().clone();
            let right: Expr = self.unary()?;
            Ok(Expr::Unary(UnaryExpr::new(op, Box::new(right))))
        } else {
            let ret = self.primary();
            ret
        }
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        match self.peek().kind {
            TokenKind::False => {
                self.advance();
                Ok(Expr::Literal(LiteralExpr::new(Literal::Str {
                    val: "false".to_string(),
                    terminated: true,
                })))
            }
            TokenKind::True => {
                self.advance();
                Ok(Expr::Literal(LiteralExpr::new(Literal::Str {
                    val: "true".to_string(),
                    terminated: true,
                })))
            }
            TokenKind::Nil => {
                self.advance();
                Ok(Expr::Literal(LiteralExpr::new(Literal::Str {
                    val: "null".to_string(),
                    terminated: true,
                })))
            }
            TokenKind::Number => {
                self.advance();
                Ok(Expr::Literal(LiteralExpr::new(
                    self.prev().literal.clone().unwrap(),
                )))
            }
            TokenKind::String => {
                self.advance();
                Ok(Expr::Literal(LiteralExpr::new(
                    self.prev().literal.clone().unwrap(),
                )))
            }
            TokenKind::LeftParen => {
                self.advance();
                let expr: Expr = self.expression()?;
                self.consume(self.peek().clone(), TokenKind::RightParen)?;

                Ok(Expr::Grouping(GroupingExpr::new(Box::new(expr))))
                // Ok(Expr::Literal(LiteralExpr::new(self.prev().literal.unwrap())))
            }
            _ => Err(self.unexpected_token_with_expected_types(
                self.peek().clone(),
                "`false`, `true`, `NUMBER`, `STRING`, or `)`".to_string(),
            )),
        }
        // if self.match_any(&[TokenKind::False]) {
        //     Expr::Literal(LiteralExpr::new(Literal::Str {
        //         val: "false".to_string(),
        //         terminated: true,
        //     }))
        // } else if self.match_any(&[TokenKind::True]) {
        //     Expr::Literal(LiteralExpr::new(Literal::Str {
        //         val: "true".to_string(),
        //         terminated: true,
        //     }))
        // } else if self.match_any(&[TokenKind::Nil]) {
        //     Expr::Literal(LiteralExpr::new(Literal::Str {
        //         val: "null".to_string(),
        //         terminated: true,
        //     }))
        // } else if self.match_any(&[TokenKind::Number]) {
        //     Expr::Literal(LiteralExpr::new(self.prev().clone()))
        // } else if self.match_any(&[TokenKind::String]) {
        //     Expr::Literal(LiteralExpr::new(self.prev().clone()))
        // }
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

    pub fn consume(&mut self, found_token: Token, ttype: TokenKind) -> Result<(), ParserError> {
        if self.check(ttype) {
            self.advance();
            Ok(())
        } else {
            Err(ParserError::UnexpectedToken(
                format!("{}", ttype),
                found_token,
            ))
        }
    }

    pub fn unexpected_token_with_expected_types(
        &mut self,
        found_token: Token,
        expected_types: String,
    ) -> ParserError {
        ParserError::UnexpectedToken(expected_types, found_token)
    }
}
