// Typesharp ast
// use crate::{ typesharp_parser::Module };
use super::position::{ Position };

pub struct AST {
	// name
	n: String,
	// // program type
	// t: ProgramType,
	// // Stack constants
	// s: Vec<ASTExpr>,
	// // Body
	// b: Vec<ASTNode>
}

pub trait AnyContext {
	fn new(&self) -> Self;
	fn getLine() -> u8;
}

pub enum ProgramType {
	/// Production
	PRO,
	/// Library
	LIB,
	/// External Foreign Function Interface
	FFI,
	/// Systematic compile, (NOT COMPILED FOR ANY OS, REQUIRES AN OBJMP)
	SYS,
	/// A driver
	INTERNAL
}

pub struct Library;

pub struct Statement<Context> {
	pub body: StmtBody,
	pub context: Context,
	pub pos: Position
}

#[derive(Clone, PartialEq, Debug)]
pub enum StmtBody {
	// expressions, function calls, returns etc should be here.
}