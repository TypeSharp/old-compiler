pub mod op;
pub mod parser;

pub use self::{ op::*, parser::* };
use crate::{ compiler::typesharp_lexer::Token };


pub fn parse(tokens: Vec<Token>) {

}