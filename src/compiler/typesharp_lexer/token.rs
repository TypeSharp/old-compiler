use crate::{ compiler::typesharp_ast::* };

pub struct Token {
     kind: TokenKind,
     position: usize,
     file: Box<str>
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

     NumberLiteral(Numeric),

     //RegularExpressionLiteral,

     TemplateLiteral(Box<str>)
}

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

     ItegerLiteralSigned128(i128)
}