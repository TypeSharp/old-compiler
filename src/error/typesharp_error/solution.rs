use crate::{
	compiler::{
		typesharp_ast::ast,
		typesharp_ast::Position,
		typesharp_ast::Span
	}
};

pub trait PossibleSolutionGenerator {
	/// This function should check whether or not the given AST can be corrected.
	/// Keep in mind that all AST given here is during Parsing.
	fn is_corrective(&self, tree: ast::ASTStatement) -> bool;

	/// Should check the given AST and correct it (if accessible) and return None otherwise.
	fn try_correct(&self, tree: ast::ASTStatement) -> Option<PossibleSolution>;

	/// Displays the correct line of code.
	fn correct_display(&self, tree: ast::ASTStatement) -> String;
}

pub enum PossibleSolutionType {
	Inline,
	Descriptive,
	Replacement
}

pub struct PossibleSolution {
	// pub resource: ast::IContextResource,
	pub loc: Span,
	pub solutions: Vec<ast::ASTStatement>,
	pub typ: PossibleSolutionType
}
