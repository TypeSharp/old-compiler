use crate::{
	compiler::{
		typesharp_ast::ast,
		typesharp_ast::Position,
		typesharp_ast::Span
	}
};

/// A enum that is a representation of error types.
/// The errors represented here are errors that are related to 
/// The compilers syntax-checking.
/// 
/// Todo: When more features get added, create a different enum for features
/// that are related to that feature.
/// 
/// An example would be: IO::ErrUnsupported, IO::
pub enum ErrGeneral {
	// An error.
	Error,
	
	// A fatal error (for possible memory related issues).
	Fatal,

	// A warning.
	Warning,

	/// A suggestion.
	Suggestion,

	/// A note.
	Note, 
}

/// This are typesharp errors that aren't related to 
/// typesharps syntax checking.
/// 
/// Note: These are not related to IO, or any other feature in the stdlib.
pub enum ErrFeature {
	// All went well, success.
	Success,

	// Invalid handle that was trying to be accessed.
	InvalidHandle,

	// Not enough memory allocated.
	NotEnoughMemory,

	// Unable to Write to a certain destination.
	WriteFault,

	// Unable to Read from certain destination.
	ReadFault,
}

pub struct ErrCustom {}


impl ErrGeneral {
	pub fn to_str(&self) -> &'static str {
		match *self {
			ErrGeneral::Error => "Syntax Error",
			ErrGeneral::Fatal => "Fatal Error",
			ErrGeneral::Note => "Note",
			ErrGeneral::Suggestion => "Suggestion:",
			ErrGeneral::Warning => "Warning",
		}
	}
}

impl ErrFeature {
	pub fn to_str(&self) -> &'static str {
		match *self {
		    ErrFeature::Success => "Sucess",
			ErrFeature::InvalidHandle => "Invalid Handle being accessed",
			ErrFeature::NotEnoughMemory => "Note enough memory available",
			ErrFeature::WriteFault => "Unable to write to certain destination.",
			ErrFeature::ReadFault => "Unable to read from certain desintation.",
		}
	}
}
