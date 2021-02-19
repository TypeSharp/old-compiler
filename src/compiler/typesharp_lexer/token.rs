use crate::{ compiler::typesharp_ast::{ Span, Position } };
use crate::{ compiler::typesharp_ast::Cursor };

pub struct Token {
     kind: TokenKind,
     span: Span,
     position: Position
}

// Binary Operators
pub enum BinOp {
     // +
     Plus,

     // -
     Minus,

     // *
     Star,

     // /
     Slash,

     // %
     Percent,

     // ^
     Caret,

     // &
     And,

     // |
     Or,

     // <<
     Sh1,

     // >>
     Shr,

     // >>>
     UShr,
}

pub enum UnaryOp {
     // ++x
     IncP,

     // x++
     Inc,

     // --x
     DecP,

     // x--
     Dec,

     // -x
     Neg,

     // +x
     Pos,

     // !x
     Not,

     // experimental delete x
     Delete,

     // A syntax sugar for x = {}
     Object,
}

pub enum LogicalOp {
     // x && y
     And,

     // x || y
     Or,

     // x ?? y
     Coalasce,
}

pub enum ComparisonOp {
     Eq,

     NotEq,

     GreaterThan,

     GreaterThanOrEqual,

     LessThan,

     LessThanOrEqual,

     Contains,

     In,

     InstanceOf,
}

// todo: AssignmentOps?
pub enum AssignmentOp {
     // x += y
     Add,

     // x -= y
     Sub,

     // x *= y
     Mul,

     // x /= y
     Div,

     // x %= y
     Rem,

     And,

     Or,

     Xor,

     Sh1,

     Shr,

     Ushr,

     // [EXPERIMENT] x &&= y
     BoolAnd,

     // [EXPERIMENT] x ||= y
     BoolOr,

     // [EXPERIMENT] x ??= y : Support may not be in future versions
     Coalesce,
}

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
     Binary,

     /// Any number that starts with: "0o".
     Octal,

     /// Any number that starts with: "0x".
     Hexadecimal,
}

pub enum Comment {
     /// Line of the comment
     Line(Span),

     /// Block
     Block(String),
}

pub enum Delimiter {
     /// Parenthesis, either "(" or ")"
     Paren,

     /// Bracket, either "[" or "]"
     Bracket,

     /// Brace, either "{" or "}"
     Brace,

     /// No delimiter
     NoDelim,
}

pub enum TokenKind {
     // A boolean, true or false
     BoolLiteral(bool),

     // End of file
     EOF,

     // A identifier (const x = 0) where x is the identifier
     Identifier(Box<str>),

     //Keyword(Keyword),

     // A string, literal. EG: Constant
     StringLiteral,

     // No idea on how I want to do this yet
     ErrorLiteral,

     NumberLiteral(Numeric),

     //RegularExpressionLiteral,

     // A template string, wrapped in ``
     TemplateLiteral(Box<str>),

     // Comment literal
     CommentLiteral(Comment),

	// Using this until i get comments situated.
	Comment,

     DelimiterLiteral(Delimiter),

     BinaryOpLiteral(BinOp),

     UnaryOpLiteral(UnaryOp),

     GenericType(Box<str>),

     AssignmentLiteral(AssignmentOp),

	// Unknown token not expected by our lexer
	Unknown
}

impl Token {
     pub fn new(kind: TokenKind, span: Span) -> Self {
          return Token { kind: kind, span: span, position: span.into_position() }
     }
}

// I don't want to do this lallalalaala
// impl From<&'static str> for Token {
//
// }

impl Cursor<'_> {
	/**
	fn consume_token(&mut self) -> Token {
		let nchar = self.peek().unwrap();
		let kind = match nchar {
			'/' => match self.first() {
				'/' => panic!("You're supposed to do something?"),
				_ => BinOp::Slash // we should change this, this is bad.
			}
		}
	}*/

	pub fn consume_comment(&mut self, inline: bool) -> Token {
		if inline == true {
			// consume while
			let initpos: Position = self.pos; // maniuplation of this really doesn't affect anything
			self.consume_while(|c| c != '\n');
			return Token {
				kind: TokenKind::Comment,
				span: Span::new(initpos, self.pos),
				position: initpos
			};
		} else {
			let initpos: Position = self.pos; // maniuplation of this really doesn't affect anything
			self.consume_while(|c| c != '*');
			return Token {
				kind: TokenKind::Comment,
				span: Span::new(initpos, self.pos),
				position: initpos
			};
		}
	}
}

/// Tokenize an input into a iterator of tokens.
pub fn tokenize(input: &str) -> Vec<Token> {
     let mut cursor = Cursor::new(input);
	let mut tokens: Vec<Token> = Vec::new();

	while !cursor.is_eof() {
		let kind = cursor.peek().unwrap_or_default();
		let token: Token = match kind {
			// comments
			'/' => match cursor.first() {
				'/' => cursor.consume_comment(true),
				'*' => cursor.consume_comment(false),
				_ => Token::new(TokenKind::BinaryOpLiteral(BinOp::Slash), Span::from(cursor.pos))// probably an op
			},
			_ => Token::new(TokenKind::Unknown, Span::from(cursor.pos))
		};

		tokens.push(token);
	}

	return tokens;
}