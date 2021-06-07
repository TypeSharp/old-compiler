// Typesharp ast
// use crate::{ typesharp_parser::Module };

pub struct AST {
	// name
	n: String,
	// program type
	t: ProgramType,
	// Stack constants
	s: Vec<ASTExpr>,
	// Body
	b: Vec<ASTNode>
}

pub struct ASTExpr {
	
}

pub struct ASTNode {

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

