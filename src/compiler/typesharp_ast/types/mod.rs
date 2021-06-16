// This is a module that handles internal classes and types.
use crate::{ compiler::typesharp_lexer::Token };

pub struct Type {
	pub kind: TypeKinds,
	pub tokens: Option<Vec<Token>>
}

pub enum TypeKinds {
	RawPtr,
	AClass,
	ARef
}