#[derive(Clone, PartialEq, Debug)]
pub enum AnyOp {
	BinOp(BinOp),
	UnaryOp(UnaryOp),
	LogicalOp(LogicalOp),
	ComparisonOp(ComparisonOp),
	AssignmentOp(AssignmentOp)
}

// Binary Operators
#[derive(Clone, PartialEq, Debug)]
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

#[derive(Clone, PartialEq, Debug)]
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

#[derive(Clone, PartialEq, Debug)]
pub enum LogicalOp {
     // x && y
     And,

     // x || y
     Or,

     // x ?? y
     Coalasce,
}

#[derive(Clone, PartialEq, Debug)]
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

#[derive(Clone, PartialEq, Debug)]
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