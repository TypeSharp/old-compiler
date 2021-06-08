// Typesharp ast
// use crate::{ typesharp_parser::Module };
use super::position::{ Position };
use super::op::*;
use super::types;
use crate::{ compiler::typesharp_lexer::token::Token };

pub struct AST {
	name: String,
	typ: ProgramType,
	body: Vec<ASTStatement>
}

#[derive(Clone, PartialEq, Debug)]
pub struct IContextResource {
	pub name: String,
	pub contents: Vec<char>
}

pub trait IContext {
	/// Initializes a new context
	fn new(&self) -> Self;

	// Gets the current resource to be parsed.
	fn getCurrentResource(&self)  -> IContextResource;

	/// Gets the current AST scope
	fn getCurrentScope(&self) -> Option<&ASTStatement>;

	/// Gets all AST scopes
	fn getScopes(&self) -> Vec<ASTStatement>;

	fn nextResource(&self) -> bool;
}

#[derive(Clone, PartialEq, Debug)]
pub struct AnyContext;

impl AnyContext {
	fn new(&self) -> AnyContext {
		return Self;
	}

	fn getLine() -> u8 {
		0
	}
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

#[derive(Clone, PartialEq, Debug)]
pub struct ASTStatement<Context = IContextResource> {
	pub body: ASTStateBody,
	pub context: Context,
	pub pos: Position
}

#[derive(Clone, PartialEq, Debug)]
pub enum ASTStateBody {
	// expressions, function calls, returns etc should be here.
	FuncCall(Signature),
	Expression(Expression),
	StackVar(Var),
	Constant(HeapVar),
	AnyConstant(AnyVar),
	If(Conditional)
}

// Context and definitions
/// A variable, const, class, etc.
#[derive(Clone, PartialEq, Debug)]
pub struct Var {
	pub op: Option<AnyOp>,
	pub typ: Option<types::Type>,
	pub val: Token,
	pub pos: Position,
	pub dies: bool,
	// pub typ: Type
}

/// Dynamic variable, extends var, which is static.
#[derive(Clone, PartialEq, Debug)]
pub struct HeapVar {
	pub var: Var,
	pub mangled: bool
}

#[derive(Clone, PartialEq, Debug)]
pub enum AnyVar {
	Static(Var),
	Heap(HeapVar)
}

/// Functions
#[derive(Clone, PartialEq, Debug)]
pub struct Signature {
	pub name: Var,
	pub dynamic: bool
}

#[derive(Clone, PartialEq, Debug)]
pub struct Expression {
	pub ops: Vec<AnyOp>,
	pub v: Vec<AnyVar>
}

#[derive(Clone, PartialEq, Debug)]
pub struct Conditional {
	pub condition: Expression,
	pub body: Vec<ASTStatement>,
	pub fin: Option<Vec<ASTStatement>>
}