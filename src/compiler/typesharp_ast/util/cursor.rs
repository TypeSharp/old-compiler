use super::position::Position;
use std::str::Chars;

pub struct Cursor<'a> {
	init_length: usize,
	chars: Chars<'a>,
	pub previous: char,
	pub pos: Position,
}

pub const EOF: char = '\0';

impl<'a> Cursor<'a> {
	/// Creates a new cursor.
	/// Very useful for creating.
	pub fn new(input: &'a str) -> Cursor<'a> {
		Cursor {
			init_length: input.len(),
			chars: input.chars(),
			previous: EOF,
			pos: Position::new(0, 0),
		}
	}

	/// Increases the cursor offset, consumes the next character
	/// Do not use if you are trying to "check" a offset in advance.
	pub fn peek(&mut self) -> Option<char> {
		let c: char = self.chars.next()?;

		self.previous = c;
		self.pos.increment(c);

		return Some(c);
	}

	/// Decreases the cursor offset.
	/// Keep in mind, if the character has already been consumed
	/// EOF is returned
	pub fn unpeek(&mut self) -> char {
		return self.chars.nth(self.length_consumed() - 1).unwrap_or(EOF);
	}

	// Grabs the next char without consuming it.
	pub fn first(&self) -> char {
		return self.nth_char(0);
	}

	// Grabs the second char without consuming it.
	pub fn second(&self) -> char {
		return self.nth_char(1);
	}

	/// Returns the `nth_char` releative to the current cursor pos
	/// If the position given doesn't exist, `EOF` is returned.
	pub fn nth_char(&self, amt: usize) -> char {
		return self.chars().nth(amt).unwrap_or(EOF);
	}

	/// Copies the current chars in the cursor.
	pub fn chars(&self) -> Chars<'a> {
		return self.chars.clone();
	}

	/// Checks the length that has been consumed by the cursor
	/// Consumed symbols are not kept.
	pub fn length_consumed(&self) -> usize {
		return self.init_length - self.chars.as_str().len();
	}

	/// Checks whether or not if theres more chars to consume
	/// Returns true, if all chars have been consumed.
	pub fn is_eof(&self) -> bool {
		return self.chars.as_str().is_empty();
	}

	/// Consumes chars until the predicate returns false or the end of file is met.
	pub fn consume_while(&mut self, mut pred: impl FnMut(char) -> bool) {
		while !self.is_eof() && pred(self.first()) {
			self.peek();
		}
	}

	/// Consumes chars until the predicate returns false, or until the end of file is met, and returns the offspring.
	pub fn consume_segment(&mut self, mut pred: impl FnMut(char) -> bool) -> String {
		let mut segment = String::new();
		while !self.is_eof() && pred(self.first()) == true {
			segment.push(self.peek().unwrap());
		}
		return segment;
	}
}
