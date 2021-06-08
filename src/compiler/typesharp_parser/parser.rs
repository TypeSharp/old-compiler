use crate::{ compiler::{
	typesharp_ast::ast,
	typesharp_lexer::Token
}};

pub struct TypeSharpRCTX {
	pub resources: Vec<ast::IContextResource>,
	pub scopes: Vec<ast::ASTStatement>,
	pub current: Option<ast::IContextResource>
}

impl ast::IContext for TypeSharpRCTX {
	fn new(&self) -> Self {
		return Self {
			resources: vec!(),
			scopes: vec!(),
			current: None
		}
	}


	// Gets the current resource to be parsed.
	fn getCurrentResource(&self) -> ast::IContextResource {
		self.current.unwrap_or(ast::IContextResource {
			name: String::from("None"),
			contents: vec!()
		})
	}

	/// Gets the current AST scope
	fn getCurrentScope(&self) -> Option<&ast::ASTStatement> {
		if self.scopes.len() > 0 {
			self.scopes.get(self.scopes.len())
		} else {
			None
		}
	}

	/// Gets all AST scopes
	fn getScopes(&self) -> Vec<ast::ASTStatement> {
		self.scopes
	}

	fn nextResource(&self) -> bool {
		true
	}
}

// Typesharp parser
pub fn parse(mut curr: &ast::AST, tokens: &Vec<Token>) {
}