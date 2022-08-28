pub use crate::lexer::token::{Literal, Token};

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

pub struct AstPrinter;
impl AstPrinter {
	pub fn print(&self, e: &Expr) -> String {
		e.walk_expr(self)
	}
}
impl ExprVisitor<String> for AstPrinter {
	fn visit_binary_expr(&self, b: &BinaryExpr) -> String {
		self.parenthesize(&b.operator.lexeme, &[&b.left, &b.right])
	}
	fn visit_grouping_expr(&self, g: &GroupingExpr) -> String {
		self.parenthesize(&"group".to_string(), &[&g.expression])
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
