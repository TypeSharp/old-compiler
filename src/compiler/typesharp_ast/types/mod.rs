// This is a module that handles internal classes and types.
pub mod builtin;
pub mod compiler;
use super::ast;

#[derive(Clone, PartialEq, Debug)]
pub struct Type {
	pub name: String,
	pub statement: TypeDefinition,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TypeDefinition {
	//// Used for linting and more descriptive errors
// pub calculated_defs: Option<Vec<ast::Conditional>>,
// pub defs: Option<Vec<ast::AnyVar>>
}

pub use self::{builtin::*, compiler::*};
