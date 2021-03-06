pub mod compiler;

fn main() {
	let code = "const: string hi = 10;";
	let lexed = compiler::typesharp_lexer::tokenize(&code);

	//println!("There are: {} tokens.", lexed.len());

	for token in lexed {
		println!("Found: {:?}", token);
	}
}