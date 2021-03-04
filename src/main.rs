pub mod compiler;

fn main() {
	let code = "1209 // okay?  2390";
	let lexed = compiler::typesharp_lexer::tokenize(&code);

	println!("There are: {} tokens.", lexed.len());

	for token in lexed {
		println!("Found: {}", token.kind);
	}
}