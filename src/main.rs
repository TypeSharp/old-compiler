pub mod compiler;
//pub mod parser_h::{ parser, compile };

fn main() {
	let code = "import std;
	/*const mychild: string = 10;
	const newval = (int) mychild;
	debug!(newval); // Number<u8, 10>
	*/";
	let lexed = compiler::typesharp_lexer::tokenize(&code);
	let val = lexed.len();

	for token in lexed {
		println!("Found: {:?}", token);
	}

	println!("There are: {} tokens.", val);
	//compile(parser::parse(lexed));
}