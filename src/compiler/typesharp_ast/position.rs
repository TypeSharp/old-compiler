use std::{ fmt, cmp::Ordering };

/**
 * This is the position that is lexed.
 * Tokens will contain a specific position, as well as lines.
 */
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct Position {
     line: u32,
     column: u32
}

impl Position {
     #[inline]
     #[track_caller]
     pub fn new(line: u32, column: u32) -> Self {
          if line <= 0 || column <= 0 {
               panic!("Column or Line position can not be less than or equal to 0.");
          }

          return Self {
               line: line,
               column: column
          }
     }
}

impl fmt::Display for Position {
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          return write!(f, "{}:{}", self.line, self.column);
     }
}

/**
 * This is used for grabbing a "section" or "span"
 * of code in a file. EG: Comments.
 */
#[derive(Clone, Copy, PartialEq, Eq, Ord)]
pub struct Span {
     start: Position,
     end: Position
}

impl Span {
     pub fn new(start: Position, end: Position) -> Self {
          assert!(start <= end, "Start position can not be after end position.");

          return Self { start: start, end: end };
     }

     pub fn contains(self, other: Span) -> bool {
          return self.start <= other.start && self.end >= other.end;
     }

     // to-do: Shrink

     pub fn verify(self) -> bool {
          return self.start <= self.end;
     }
}

impl fmt::Display for Span {
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          return write!(f, "Span {{ {}, {} }}", self.start, self.end);
     }
}

impl From<Position> for Span {
     fn from(pos: Position) -> Self {
          Self {
               start: pos,
               end: pos
          }
     }
}

impl PartialOrd for Span {
     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
          if self == other {
               return Some(Ordering::Equal);
          } else if self.end < other.start {
               return Some(Ordering::Less);
          } else if self.start > other.end {
               return Some(Ordering::Greater);
          } else {
               return None;
          }
     }
}