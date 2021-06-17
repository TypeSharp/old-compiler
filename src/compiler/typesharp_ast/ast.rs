// Typesharp ast
// use crate::{ typesharp_parser::Module };
use super::types;
use super::util::position;
use super::node::NodeId;
use crate::{ compiler::typesharp_parser as parser };
use crate::compiler::typesharp_lexer::token::Token;

pub struct Identifier {
	pub loc: position::Span,
	pub tokens: Option<Vec<Token>>
}

pub enum StatementKind {
	Constant(Constant),
	Item,
	Expression,
	Label,
	Scope,
	TypeDeclaration(types::Type),
	Class(Constant)
}

pub enum ExpressionKind {
	/// An array of any expression
	Array(Vec<Expression>),
	/// Private context
	Scope,
	/// A call to a function or reference to a signature
	Function,
	/// A method call eg: `foo.bar(a, b)`
	///
	/// The `Identifier` here represents the Name of the method being called
	/// `Vec<Expression>` represents the arguments given to the expression
	Method(Identifier, Vec<Expression>, position::Span)
}

pub struct Constant {
	pub id: NodeId,
	pub typ: types::Type
}

pub struct Statement {
	pub id: NodeId,
	pub kind: StatementKind,
	pub loc: position::Span
}

pub struct Param {
	pub id: NodeId,
	// pub val: Node
}

pub struct Expression {
	pub id: NodeId,
	pub kind: ExpressionKind,
}