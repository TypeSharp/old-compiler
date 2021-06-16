// Typesharp ast
// use crate::{ typesharp_parser::Module };
use super::types;
use super::util::position;
use super::node::NodeId;
use crate::{ compiler::typesharp_parser as parser };
// use crate::compiler::typesharp_lexer::token::Token;

pub enum StatementKind {
	Constant(Constant),
	Item,
	Expression,
	Label,
	TypeDeclaration(types::Type),
	Class(Constant)
}

pub enum ExpressionKind {
	/// Private context
	Scope,
	/// A call to a function or reference to a signature
	Signature,
	Operation(Option<parser::AnyOp>)
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