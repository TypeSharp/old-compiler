use crate::{ typesharp::* };
use crate::{ typesharp::compiler::typesharp_ast_old::Features };
use std::fs::write;
fn main() {
	let code = "use std.math;
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

	let bin = compile(code, compiler::typesharp_ast_two::ASTOpts {
		assembler: compiler::typesharp_ast_two::Assembler::LLVM,
		mangle: false,
		library: false,
		entry: "main",
		features: Features::Defaults,
		imports: vec!()
		debug: true
	});

	write("test.exe", bin.to_bytes());
}
