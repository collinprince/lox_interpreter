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
	pub expression: Box<Expr>,
}

pub struct LiteralExpr {
	pub value: Literal,
}

pub struct UnaryExpr {
	pub operator: Token,
	pub right: Box<Expr>,
}

pub trait ExprVisitor<T> {
	fn visit_binary_expr(&self, e: &BinaryExpr) -> T;
	fn visit_grouping_expr(&self, e: &GroupingExpr) -> T;
	fn visit_literal_expr(&self, e: &LiteralExpr) -> T;
	fn visit_unary_expr(&self, e: &UnaryExpr) -> T;
}

impl Expr {
	pub fn walk_expr<T>(&self, v: &dyn ExprVisitor<T>) -> T {
		match self {
			Expr::Binary(e) => e.walk_binary_expr(v),
			Expr::Grouping(e) => e.walk_grouping_expr(v),
			Expr::Literal(e) => e.walk_literal_expr(v),
			Expr::Unary(e) => e.walk_unary_expr(v),
		}
	}
}

impl BinaryExpr {
	pub fn walk_binary_expr<T>(&self, v: &dyn ExprVisitor<T>) -> T {
		v.visit_binary_expr(self)
	}
}

impl GroupingExpr {
	pub fn walk_grouping_expr<T>(&self, v: &dyn ExprVisitor<T>) -> T {
		v.visit_grouping_expr(self)
	}
}

impl LiteralExpr {
	pub fn walk_literal_expr<T>(&self, v: &dyn ExprVisitor<T>) -> T {
		v.visit_literal_expr(self)
	}
}

impl UnaryExpr {
	pub fn walk_unary_expr<T>(&self, v: &dyn ExprVisitor<T>) -> T {
		v.visit_unary_expr(self)
	}
}

