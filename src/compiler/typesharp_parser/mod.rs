pub mod op;
pub mod parser;

pub use self::{ op::*, parser::* };
use crate::{ compiler::typesharp_lexer::Token };

pub struct Parser;

impl Parser {
	// fn singleton_parse(stream: Vec<Token>) {}
}