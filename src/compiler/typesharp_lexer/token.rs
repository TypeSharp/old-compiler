use crate::{ compiler::typesharp_ast::{ Span, Position } };

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

pub enum Numeric {
     // @reference https://doc.rust-lang.org/beta/reference/types/numeric.html
     // Floating number
     FloatLiteral(f32),

     // Floating number that is larger than a float
     DoubleLiteral(f64),

     // Integer: const blah: int = 13910;
     IntegerLiteral(i32),

     // probably going to allow types for: i64 numerically. Compiler will detect?
     IntegerLiteralBig(i64),

     // idk wtf you would need this for but, its there lmfao
     ItegerLiteralSigned128(i128)
}

pub enum Comment {
     // Line of the comment
     Line,

     // Block
     Block,
}

pub enum Delimiter {
     // Parenthesis, either "(" or ")"
     Paren,

     // Bracket, either "[" or "]"
     Bracket,

     // Brace, either "{" or "}"
     Brace,

     // No delimiter
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

     DelimiterLiteral(Delimiter),

     BinaryOpLiteral(BinOp),

     UnaryOpLiteral(UnaryOp)
}

impl Token {
     pub fn new(kind: TokenKind, span: Span) -> Self {
          return Token { kind: kind, span: span, position: span.into_position() }
     }
}