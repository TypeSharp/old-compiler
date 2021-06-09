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
pub enum ErrCompiler {
	/// An error. (general error)
	Error,
	
	/// A fatal error (for possible memory related issues, or maybe even compiler issues that
     /// the user wasn't supposed to reach).
	Unreachable,

	/// A warning.
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
	/// All went well, success.
	Success,

	/// Invalid handle that was trying to be accessed.
	InvalidHandle,

	/// Not enough memory allocated.
	NotEnoughMemory,

	/// Unable to Write to a certain destination.
	WriteFault,

	/// Unable to Read from certain destination.
	ReadFault,
}

/// Trait that all compiler-level errors adhere to.
trait ErrImpl {
    fn to_str(&self) -> &'static str;
}


impl ErrImpl for ErrCompiler {
	fn to_str(&self) -> &'static str {
		match *self {
			ErrCompiler::Error => "Compiler Error",
			ErrCompiler::Unreachable => "Unreachable point accessed in the compiler, this is an internal compiler issue.",
			ErrCompiler::Note => "Note:",
			ErrCompiler::Suggestion => "Suggestion:",
			ErrCompiler::Warning => "Warning:",
		}
	}
}

impl ErrImpl for ErrFeature {
	fn to_str(&self) -> &'static str {
		match *self {
		     ErrFeature::Success => "Sucess",
			ErrFeature::InvalidHandle => "Invalid Handle being accessed",
			ErrFeature::NotEnoughMemory => "Not enough memory available",
			ErrFeature::WriteFault => "Unable to write to certain destination.",
			ErrFeature::ReadFault => "Unable to read from certain desintation.",
		}
	}
}
