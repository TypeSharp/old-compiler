use typesharp;
// use crate::{ typesharp::compiler::typesharp_ast_old::Features };
use std::fs::write;
fn main() {
	let code = "\"this is a test string\\\" with a qoutation escape sequence\"\"and another string next to that\"";
	let lexed = typesharp::compiler::typesharp_lexer::tokenize(&code);
	let val = lexed.len();

	for token in lexed {
		println!("Found: {:?}", token);
	}

	println!("There are: {} tokens.", val);

}
