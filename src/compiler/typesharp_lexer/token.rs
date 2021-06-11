use crate::{
	compiler::typesharp_ast::{Cursor, KeyWord, Position, Span},
	compiler::typesharp_parser::op::*,
};

pub type TokenType = (String, Box<str>);

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
	pub kind: TokenKind,
	pub span: Span,
	pub position: Position,
}

impl Token {
	pub fn new(kind: TokenKind, span: Span, pos: Option<Position>) -> Self {
		return Token {
			kind: kind,
			span: span,
			position: pos.unwrap_or(Position::new(0, 0)),
		};
	}

	pub fn build(kind: TokenKind, pos: Position) -> Self {
		return Token::new(kind, Span::from(pos), Some(pos));
	}
}

/// Public trait
/// This should be used to extract a value from a TokenKind (or related)
/// By default, typesharp **relies** on this implementation during parsing.
pub trait TokenValue<T> {
	fn get(&self) -> T;
}

/// Numerics!
/// Any numeric type can be referred to as below
#[derive(Clone, PartialEq, Debug)]
pub enum Numeric {
	/// @reference https://doc.rust-lang.org/beta/reference/types/numeric.html
	/// Floating number
	FloatLiteral(f32),

	/// Floating number that is larger than a float
	DoubleLiteral(f64),

	/// Integer: const blah: int = 13910;
	IntegerLiteral(i32),

	/// probably going to allow types for: i64 numerically. Compiler will detect?
	IntegerLiteralBig(i64),

	/// idk wtf you would need this for but, its there lmfao
	ItegerLiteralSigned128(i128),

	/// Any number that starts with: "0b".
	Binary(usize),

	/// Any number that starts with: "0o".
	Octal(usize),

	/// Any number that starts with: "0x".
	Hexadecimal(usize),
}

impl Numeric {
	fn new(s: String) -> Self {
		match s {
			_ => Numeric::Hexadecimal(0),
		}
	}
}

impl TokenValue<usize> for Numeric {
	fn get(&self) -> usize {
		match *self {
			Numeric::Binary(n) => n,
			Numeric::FloatLiteral(n) => n as usize,
			Numeric::Hexadecimal(n) => n,
			Numeric::IntegerLiteral(n) => n as usize,
			Numeric::IntegerLiteralBig(n) => n as usize,
			Numeric::ItegerLiteralSigned128(n) => n as usize,
			Numeric::Octal(n) => n,
			_ => panic!("Unknown Numeric Type provided."),
		}
	}
}

impl std::fmt::Display for Numeric {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "0") // lazy impl, todo: properly impl
	}
}

/// Comment Implementation
///
/// Block Comments:
///  /* test */
///
/// Line Comments:
///  // test
#[derive(Clone, PartialEq, Debug)]
pub enum Comment {
	/// Line of the comment
	Line(String),

	/// Block
	Block(String),
}

impl TokenValue<String> for Comment {
	fn get(&self) -> String {
		match self {
			Comment::Line(c) => c.to_string(),
			Comment::Block(c) => c.to_string()
		}
	}
}

impl std::fmt::Display for Comment {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Comment::Line(c) => write!(f, "Comment<Line> {{ {} }}", c),
			Comment::Block(c) => write!(f, "Comment<Block> {{ {} }}", c)
		}
	}
}

/// Delimiters are defined below, each delimiter is a type of scope delarator
#[derive(Clone, PartialEq, Debug)]
pub enum Delimiter {
	/// Parenthesis, either "(" or ")"
	Paren(String),

	/// Bracket, either "[" or "]"
	Bracket(String),

	/// Brace, either "{" or "}"
	Brace(String),

	/// No delimiter
	NoDelim,
}

impl TokenValue<String> for Delimiter {
	fn get(&self) -> String {
		match self {
			Delimiter::Paren(t) => t.to_string(),
			Delimiter::Bracket(t) => t.to_string(),
			Delimiter::NoDelim => String::from("None"),
			_ => panic!("Unknown Delimiter"),
		}
	}
}

impl std::fmt::Display for Delimiter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		return write!(f, "Delimiter {{ {} }}", self.get());
	}
}

#[derive(Clone, PartialEq, Debug)]
pub enum TokenKind {
	// Quite literally an accessor,
	// EG: "."
	Accessor,

	// A boolean, true or false
	BoolLiteral(String),

	// End of file
	EOF,

	Keyword(KeyWord),

	// A identifier (const x = 0) where x is the identifier
	Identifier(String),

	//Keyword(Keyword),

	// A string, literal. EG: Constant
	StringLiteral(String),

	// No idea on how I want to do this yet
	ErrorLiteral,

	NumberLiteral(Numeric),

	//RegularExpressionLiteral,

	// A template string, wrapped in ``
	TemplateLiteral(String),

	// Comment literal
	CommentLiteral(Comment),

	// Using this until i get comments situated.
	// Comment,
	DelimiterLiteral(Delimiter),

	BinaryOpLiteral(BinOp),

	UnaryOpLiteral(UnaryOp),

	GenericType(String),

	AssignmentLiteral(AssignmentOp),

	ExpressionTerminator,

	Indent,

	WhiteSpace,

	// Unknown token not expected by our lexer
	Unknown(String),
}

impl TokenKind {
	/// Gets the token as a string value
	fn as_str(&self) -> String {
		match self {
			TokenKind::Accessor => String::from("."),
			TokenKind::BoolLiteral(v) => String::from(v),
			TokenKind::EOF => String::from("EOF"),
			TokenKind::Keyword(v) => v.get(),
			TokenKind::Identifier(v) => v.to_string(),
			TokenKind::StringLiteral(v) => v.to_string(),
			TokenKind::ErrorLiteral => String::from("Error"),
			TokenKind::NumberLiteral(n) => format!("{}", n.get()),
			TokenKind::TemplateLiteral(v) => v.to_string(),
			TokenKind::CommentLiteral(c) => c.get(),
			TokenKind::DelimiterLiteral(v) => v.get(),
			TokenKind::BinaryOpLiteral(v) => String::from("Op unknown"),
			TokenKind::UnaryOpLiteral(v) => String::from("Op unknown"),
			TokenKind::GenericType(v) => String::from("Op unknown"),
			TokenKind::AssignmentLiteral(v) => String::from("Op unknown"),
			TokenKind::ExpressionTerminator => String::from("Expression Terminated"),
			TokenKind::Indent => String::from(""),
			TokenKind::WhiteSpace => String::from(" "),
			TokenKind::Unknown(v) => v.to_string()
		}
	}
}

impl std::fmt::Display for TokenKind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match *self {
			TokenKind::CommentLiteral(_) => write!(f, "CommentLiteral"),
			TokenKind::NumberLiteral(_) => write!(f, "Number"),
			TokenKind::Indent | TokenKind::WhiteSpace => write!(f, "Space or Whitespace"),
			_ => write!(f, "Unknown token."),
		}
	}
}

// A macro utility for ease of use with token.
macro_rules! token {
	// used for building without chaos
	($kind: expr, $span: expr) => {
		Token::new($kind, $span, None);
	};
	() => {
		Token::build(TokenKind::Unknown(String::from("")), Position::new(0, 0));
	};
}

// I don't want to do this lallalalaala
// impl From<&'static str> for Token {
//
// }

impl Cursor<'_> {
	/// Definitely tries to consume a token.
	/// If it can't, we panic.
	pub fn consume_token(&mut self, init: &char) -> Token {
		return match init {
			// comments
			'/' => match self.first() {
				'/' => self.consume_comment(true),
				'*' => self.consume_comment(false),
				_ => Token::new(
					TokenKind::BinaryOpLiteral(BinOp::Slash),
					Span::from(self.pos),
					None,
				), // probably an op
			},

			// numbers (parser checks for numeric types later)
			'0'..='9' => self.consume_any_numeric(*init),

			// whitespace (eg: space)
			' ' | '\n' | '\r' | '\t' => {
				Token::new(TokenKind::WhiteSpace, Span::from(self.pos), None)
			},
			'"' | '\'' => self.consume_any_string(Some(init)),
			'A'..='z' => self.consume_keyword_or_identifier(Some(init)),
			'.' => token!(TokenKind::Accessor, Span::from(self.pos)),
			';' => token!(TokenKind::ExpressionTerminator, Span::from(self.pos)),
			_ => token!(TokenKind::Unknown(init.to_string()), Span::from(self.pos)),
		};
	}

	/// Indefinitely consumes any word encapsulated in a string char
	pub fn consume_any_string(&mut self, init: Option<&char>) -> Token {
		let init_pos: Position = self.pos;

		if init == None {
			// there was no initial char
			// we need to panic because it's impossible to know when we can terminate the string.
			// to-do: Implement errors.
			panic!("Unknown String");
		}

		let mut previous = self.previous;
		let string = self.consume_segment(|c| -> bool {
			if c == *init.unwrap() {
				if previous == '\\' {
					return true;
				}
				return false;
			} else {
				previous = c;
				return true;
			}
		});

		// consume the next char because it is a string terminator and has not been consumed, (possibly fix cursor?)
		self.peek();

		return token!(
			TokenKind::StringLiteral(string),
			Span::new(init_pos, self.pos)
		);
	}

	/// Indefinitely consume until we match whitespace.
	/// This will resolve keywords into tokens if found,
	/// if not found, will return a token in the form of a
	/// **VALID** identifier.
	pub fn consume_keyword_or_identifier(&mut self, init: Option<&char>) -> Token {
		let init_pos: Position = self.pos;
		// consume and preserve until next space
		let mut identifier: String = String::new();
		if init == None {
			identifier = self.consume_segment(|c| !c.is_whitespace() && c.is_alphanumeric());
		} else {
			identifier.push(*init.unwrap());
			identifier.push_str(
				self.consume_segment(|c| !c.is_whitespace() && c.is_alphanumeric())
					.chars()
					.as_str(),
			);
		}
		let span: Span = Span::new(init_pos, self.pos);

		match &identifier[..] {
			// refer to keywords for this
			"as" => token!(TokenKind::Keyword(KeyWord::As), span),
			"asm" => token!(TokenKind::Keyword(KeyWord::Asm), span),
			"async" => token!(TokenKind::Keyword(KeyWord::Async), span),
			"await" => token!(TokenKind::Keyword(KeyWord::Await), span),
			"break" => token!(TokenKind::Keyword(KeyWord::Break), span),
			"case" => token!(TokenKind::Keyword(KeyWord::Case), span),
			"catch" => token!(TokenKind::Keyword(KeyWord::Catch), span),
			"class" => token!(TokenKind::Keyword(KeyWord::Class), span),
			"continue" => token!(TokenKind::Keyword(KeyWord::Continue), span),
			"const" => token!(TokenKind::Keyword(KeyWord::Const), span),
			"default" => token!(TokenKind::Keyword(KeyWord::Default), span),
			// [EXPERIMENT] See: https://github.com/TypeSharp/Typesharp/issues/1
			"delete" => token!(TokenKind::Keyword(KeyWord::Delete), span),
			"else" => token!(TokenKind::Keyword(KeyWord::Else), span),
			"enum" => token!(TokenKind::Keyword(KeyWord::Enum), span),
			"export" => token!(TokenKind::Keyword(KeyWord::Export), span),
			"extern" => token!(TokenKind::Keyword(KeyWord::Extern), span),
			// May change in the future
			"external" => token!(TokenKind::Keyword(KeyWord::Extern), span),
			"extends" => token!(TokenKind::Keyword(KeyWord::Extends), span),
			"for" => token!(TokenKind::Keyword(KeyWord::For), span),
			"function" => token!(TokenKind::Keyword(KeyWord::Function), span),
			"fn" => token!(TokenKind::Keyword(KeyWord::Fn), span),
			"if" => token!(TokenKind::Keyword(KeyWord::If), span),
			"in" => token!(TokenKind::Keyword(KeyWord::In), span),
			"instanceof" => token!(TokenKind::Keyword(KeyWord::InstanceOf), span),
			"import" => token!(TokenKind::Keyword(KeyWord::Import), span),
			"let" => token!(TokenKind::Keyword(KeyWord::Let), span),
			"new" => token!(TokenKind::Keyword(KeyWord::New), span),
			"of" => token!(TokenKind::Keyword(KeyWord::Of), span),
			// [EXPERIMENT]
			// Todo: Add checks and feature gates
			"package" => token!(TokenKind::Keyword(KeyWord::Package), span),
			"return" => token!(TokenKind::Keyword(KeyWord::Return), span),
			"self" => token!(TokenKind::Keyword(KeyWord::SelfKeyword), span),
			"static" => token!(TokenKind::Keyword(KeyWord::Static), span),
			"super" => token!(TokenKind::Keyword(KeyWord::Super), span),
			"switch" => token!(TokenKind::Keyword(KeyWord::Switch), span),
			"trait" => token!(TokenKind::Keyword(KeyWord::Trait), span),
			"this" => token!(TokenKind::Keyword(KeyWord::This), span),
			"throw" => token!(TokenKind::Keyword(KeyWord::Throw), span),
			"type" => token!(TokenKind::Keyword(KeyWord::Type), span),
			"try" => token!(TokenKind::Keyword(KeyWord::Try), span),
			"where" => token!(TokenKind::Keyword(KeyWord::Where), span),
			"while" => token!(TokenKind::Keyword(KeyWord::While), span),

			// Abstractions are here as i dont plan on implementing them until all of the above is done.
			// I also believe this will change a lot so that is antoher reason I am leaving this here.

			// [EXPERIMENTAL] https://github.com/TypeSharp/Typesharp/issues/10
			"final" => token!(TokenKind::Keyword(KeyWord::Final), span),
			"finally" => token!(TokenKind::Keyword(KeyWord::Finally), span),
			"override" => token!(TokenKind::Keyword(KeyWord::Override), span),
			"typeof" => token!(TokenKind::Keyword(KeyWord::Typeof), span),
			"yield" => token!(TokenKind::Keyword(KeyWord::Yield), span),
			"public" => token!(TokenKind::Keyword(KeyWord::Public), span),
			// Pub may not be used, it is reserved under keyword incase it is
			// the pub keyword will be treated as "public" for now.
			"pub" => token!(TokenKind::Keyword(KeyWord::Public), span),
			"private" => token!(TokenKind::Keyword(KeyWord::Private), span),
			"protected" => token!(TokenKind::Keyword(KeyWord::Protected), span),

			// Types that aren't compiled are here as well for the same reason that abstractions
			// are below everything else.

			// Unions are complex types, and are currently reserved.
			"union" => token!(TokenKind::Keyword(KeyWord::Union), span),
			"implements" => token!(TokenKind::Keyword(KeyWord::Implements), span),
			"interface" => token!(TokenKind::Keyword(KeyWord::Interface), span),

			// Wasn't a keyword, it was an identifier
			_ => token!(TokenKind::Identifier(identifier), span),
		}
	}

	/// Consumes an inline or multiline comment.
	pub fn consume_comment(&mut self, inline: bool) -> Token {
		if inline == true {
			// consume while
			let initpos: Position = self.pos; // maniuplation of this really doesn't affect anything
			let block: String = self.consume_segment(|c| c != '\n');
			return token!(
				TokenKind::CommentLiteral(Comment::Block(block)),
				Span::new(initpos, self.pos)
			);
		} else {
			let initpos: Position = self.pos; // maniuplation of this really doesn't affect anything
			self.peek(); // we need this to consume this old char.
			let seg: String = self.consume_segment(|c| c != '*');
			self.peek();
			self.peek(); // this is hacky, pls find fix
			return token!(
				TokenKind::CommentLiteral(Comment::Line(seg)),
				Span::new(initpos, self.pos)
			);
		}
	}

	/// Consumes a numeric
	/// A numeric can be one of: int, double, float, or decimal
	/// Check: https://bavfalcon9.gitbook.io/typesharp/types/numeric-types for more information
	/// Please note that this documentation may be outdated
	pub fn consume_any_numeric(&mut self, initial: char) -> Token {
		let mut number: String = String::from(initial);
		let init_pos: Position = self.pos;
		// immediately check next char but don't consume
		number.push_str(
			self.consume_segment(|c| c.is_numeric() || c == '.')
				.chars()
				.as_str(),
		);
		return Token::new(
			TokenKind::NumberLiteral(Numeric::new(number)),
			Span::new(init_pos, self.pos),
			None,
		);
	}

	pub fn last_is_escape_char(&self) -> bool {
		return self.previous == '\\'
	}
}

/// Tokenize an input into a iterator of tokens.
pub fn tokenize(input: &str) -> Vec<Token> {
	let mut cursor = Cursor::new(input);
	let mut tokens: Vec<Token> = Vec::new();

	while !cursor.is_eof() {
		let kind = cursor.peek().unwrap();
		let token: Token = cursor.consume_token(&kind);
		tokens.push(token);
	}

	return tokens;
}
