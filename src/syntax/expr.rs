pub use crate::lexer::{Literal, Token};

pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct GroupingExpr {
    pub expr: Box<Expr>,
}

pub struct LiteralExpr {
    pub value: Literal,
}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub trait ExprVisitor<T> {
    fn visit_expr(&self, e: &Expr) -> T;
    fn visit_binary_expr(&self, b: &BinaryExpr) -> T;
    fn visit_grouping_expr(&self, g: &GroupingExpr) -> T;
    fn visit_literal_expr(&self, l: &LiteralExpr) -> T;
    fn visit_unary_expr(&self, u: &UnaryExpr) -> T;
}

pub trait ExprWalker<T> {
    fn walk_expr(&self, v: &dyn ExprVisitor<T>) -> T;
}

impl<T> ExprWalker<T> for Expr {
    fn walk_expr(&self, v: &dyn ExprVisitor<T>) -> T {
        v.visit_expr(self)
    }
}

impl<T> ExprWalker<T> for BinaryExpr {
    fn walk_expr(&self, v: &dyn ExprVisitor<T>) -> T {
        v.visit_binary_expr(self)
    }
}

impl<T> ExprWalker<T> for GroupingExpr {
    fn walk_expr(&self, v: &dyn ExprVisitor<T>) -> T {
        v.visit_grouping_expr(self)
    }
}

impl<T> ExprWalker<T> for UnaryExpr {
    fn walk_expr(&self, v: &dyn ExprVisitor<T>) -> T {
        v.visit_unary_expr(self)
    }
}

impl<T> ExprWalker<T> for LiteralExpr {
    fn walk_expr(&self, v: &dyn ExprVisitor<T>) -> T {
        v.visit_literal_expr(self)
    }
}

pub struct AstPrinter;
impl ExprVisitor<String> for AstPrinter {
    fn visit_expr(&self, e: &Expr) -> String {
        match e {
            Expr::Grouping(a) => a.walk_expr(self),
            Expr::Binary(b) => b.walk_expr(self),
            Expr::Literal(c) => c.walk_expr(self),
            Expr::Unary(d) => d.walk_expr(self),
        }
    }
    fn visit_binary_expr(&self, b: &BinaryExpr) -> String {
        self.parenthesize(&b.operator.lexeme, &[&b.left, &b.right])
    }
    fn visit_grouping_expr(&self, g: &GroupingExpr) -> String {
        self.parenthesize(&"group".to_string(), &[&g.expr])
    }
    fn visit_literal_expr(&self, l: &LiteralExpr) -> String {
        match &l.value {
            Literal::Num { val } => val.to_string(),
            Literal::Str { val, .. } => val.clone(),
        }
    }
    fn visit_unary_expr(&self, u: &UnaryExpr) -> String {
        self.parenthesize(&u.operator.lexeme, &[&u.right])
    }
}

impl AstPrinter {
    pub fn parenthesize(&self, name: &String, exprs: &[&Box<Expr>]) -> String {
        let mut ret: String = String::from(format!("({name}"));
        for x in exprs {
            let some = format!(" {}", x.walk_expr(self));
            ret.push_str(some.as_str());
        }
        ret.push(')');
        ret
    }
}

// TOOD: replace these Err(String)s with correct LoxError types
pub fn match_op(operator: Token) -> Result<Box<dyn std::ops::FnOnce(f64, f64) -> f64>, String> {
    match operator.literal {
        None => Err("No operator found".to_string()),
        Some(l) => match l {
            Literal::Str { val, .. } => match val.as_str() {
                "+" => Ok(Box::new(std::ops::Add::add)),
                "-" => Ok(Box::new(std::ops::Sub::sub)),
                "*" => Ok(Box::new(std::ops::Mul::mul)),
                "/" => Ok(Box::new(std::ops::Div::div)),
                _ => Err("Invalid operator".to_string()),
            },
            _ => Err("Invalid operator".to_string()),
        },
    }
}
