use std::{ fmt, error };

/// These keywords are reserved.
/// If you use these as identifiers you will be yelled at.
#[derive(Debug)]
pub enum KeyWord {
     /// Keywords available in all versions of typesharp.
	/// Asm, allows use for inline assembly
	Asm,

     /// Await, used for synchronizing load.
     Await,

     /// Async, allows for asynchronous programming
     Async,

     /// As, used for type casting
     As,

     /// Used to break out of statements like "if" etc.
     Break,

     ///
     Case,

     /// Used in try...catch blocks, catches errors.
     Catch,

     /// OOP, reserved word for class
     Class,

     /// Continue, used in loops to continue the loop
     Continue,

     /// Constants, non-mutable constant vars.
     Const,

     /// Defaults, used in switch statements.
     Default,

     /// Delete (may be removed), removes a var. (this may be removed due to memory safety)
     /// See issue https:///github.com/TypeSharp/Typesharp/issues/1
     Delete,

     /// Used in if...else blocks.
     Else,

     /// Enums, mainly just syntax sugar
     Enum,

     /// Exports module
     /// THIS DOES NOT HAVE THE SAME USE CASE AS JS
     /// Exporting a module makes it global.
     Export,

     /// Creates an external function (used for FFI or dynlibs)
     Extern,

     /// Class inheritance for extending
     Extends,

     /// False
     False,

     /// For loops, eg: for let i in blah {}
     For,

     /// Function keyword, makes a function
     Function,

     /// Reserved macro for function
     Fn,

     /// If keyword
     If,

     /// In, used in for..in loops
     In,

     /// Checks an instance of a given object.
     InstanceOf,

     /// Imports a public module
     /// Use case is SIMILAR to JS
     Import,

     /// Creates a mutable identifier.
     /// Exactly equal to: let mut something: u8 = 0; in rust.
     Let,

     /// Creates an instance of a class
     New,

     /// Used in for...of loops
     Of,

     /// Creates a package. Similar to `mod` keyword in rust.
     Package,

     /// Return
     Return,

     /// Self, used in classes, and functions for getting "it's self"
     SelfKeyword,

     /// Static, used to create a static reference
     Static,

     /// Super, accesses the parent class in typesharp
     /// Eg:
     /// ```ts
     /// class Alphabet {}
     /// class A extends Alphabet, Numeric {
     ///     public constructor() {
     ///        super(); /// alphabet
     ///        super as Numeric(0); /// numeric class.
     ///     }
     /// }
     /// ```
     Super,

     /// Switch, used to catch cases, maybe implement match?
     Switch,

     /// True
     True,

     /// Traits, traits in typesharp are very similar to php
     /// ```ts
     ///   trait Living {
     ///        private name: string = "";
     ///        public function getName(): string {
     ///             return this.name;
     ///        }
     ///        public function talk(message: string) {
     ///             print!(`${this.name} says: "${message}"`);
     ///        }
     ///   }
     ///   class Animal {
     ///        use Living;
     ///        public constructor(name: string) {
     ///             this.name = name;
     ///        }
     ///   }
     ///   let dog: Animal = new Animal("Dog");
     ///   dog.talk("Woof!");
     ///   dog.getName(); // Dog
     /// ```
     Trait,

     /// An accessor for a class, access as "self" but not statically.
     This,

     /// Throws an error.
     Throw,

     /// Type, declares a type
     Type,

     /// Used in Try...catch loops
     Try,

     /// Where, (Might implement)
     Where,

     /// While loops.
     While,

     /// Abstractions
     Abstract,
     Become,
     Do,

     /// This is not for sure being implemented
     /// Final classes and vars seem redundant
     /// See issue: https://github.com/TypeSharp/Typesharp/issues/10
     Final,
     Finally,
     Override,
     Typeof,
     Yield,
     Public,
     Pub,
     Private,
     Protected,

     /// Types
     Union,
     Implements,
     Interface,
}

impl KeyWord {
     pub fn as_str(&self) -> &'static str {
          // To-Do
          match self {
			Self::Asm => "asm",
               Self::Async => "async",
               Self::Await => "await",
               Self::As => "as",
               Self::Become => "become",
               Self::Break => "break",
               Self::Case => "case",
               Self::Catch => "catch",
               Self::Class => "class",
               Self::Continue => "continue",
               Self::Const => "const",
               Self::Default => "default",
               Self::Delete => "delete",
               Self::Else => "else",
               Self::Enum => "enum",
               Self::Extends => "extends",
               Self::Export => "export",
               Self::Extern => "extern",
               Self::False => "false",
               Self::For => "for",
               Self::Function => "function",
               Self::Fn => "fn",
               Self::If => "if",
               Self::In => "in",
               Self::InstanceOf => "instanceof",
               Self::Import => "import",
               Self::Let => "let",
               Self::New => "new",
               Self::Of => "of",
               Self::Package => "package",
               Self::Return => "return",
               Self::SelfKeyword => "self",
               Self::Static => "static",
               Self::Super => "super",
               Self::Switch => "switch",
               Self::Trait => "trait",
               Self::True => "true",
               Self::This => "this",
               Self::Throw => "throw",
               Self::Type => "type",
               Self::Try => "try",
               Self::Where => "where",
               Self::While => "while",
               Self::Abstract => "abstract",
               Self::Do => "do",
               Self::Final => "final",
               Self::Finally => "finally",
               Self::Override => "override",
               Self::Typeof => "typeof",
               Self::Yield => "yield",
               Self::Public => "public",
               Self::Pub => "pub",
               Self::Private => "private",
               Self::Protected => "protected",
               Self::Union => "union",
               Self::Implements => "implements",
               Self::Interface => "interface"
          }
     }

     pub fn from_str(name: &'static str) -> KeyWord {
          match name {
			"Asm" => Self::Asm,
               "async" => Self::Async,
               "await" => Self::Await,
               "as" => Self::As,
               "become" => Self::Become,
               "break" => Self::Break,
               "case" => Self::Case,
               "catch" => Self::Catch,
               "class" => Self::Class,
               "continue" => Self::Continue,
               "const" => Self::Const,
               "default" => Self::Default,
               "delete" => Self::Delete,
               "else" => Self::Else,
               "enum" => Self::Enum,
               "extends" => Self::Extends,
               "export" => Self::Export,
               "extern" => Self::Extern,
               "false" => Self::False,
               "for" => Self::For,
               "function" => Self::Function,
               "fn" => Self::Fn,
               "if" => Self::If,
               "in" => Self::In,
               "instanceof" => Self::InstanceOf,
               "import" => Self::Import,
               "let" => Self::Let,
               "new" => Self::New,
               "of" => Self::Of,
               "package" => Self::Package,
               "return" => Self::Return,
               "self" => Self::SelfKeyword,
               "static" => Self::Static,
               "super" => Self::Super,
               "switch" => Self::Switch,
               "trait" => Self::Trait,
               "true" => Self::True,
               "this" => Self::This,
               "throw" => Self::Throw,
               "type" => Self::Type,
               "try" => Self::Try,
               "where" => Self::Where,
               "while" => Self::While,
               "abstract" => Self::Abstract,
               "do" => Self::Do,
               "final" => Self::Final,
               "finally" => Self::Finally,
               "override" => Self::Override,
               "typeof" => Self::Typeof,
               "yield" => Self::Yield,
               "public" => Self::Public,
               "pub" => Self::Pub,
               "private" => Self::Private,
               "protected" => Self::Protected,
               "union" => Self::Union,
               "implements" => Self::Implements,
               "interface" => Self::Interface,
               _ => panic!("Unknown Keyword!")
          }
     }
}

impl From<&'static str> for KeyWord {
     fn from(m: &'static str) -> KeyWord {
          return KeyWord::from_str(m);
     }
}

pub struct KeyWordError;
impl fmt::Display for KeyWordError {
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          return write!(f, "");
     }
}

impl error::Error for KeyWord {
     fn description(&self) -> &str {
          return "Unknown Token";
     }

     fn cause(&self) -> Option<&dyn error::Error> {
          return None;
     }
}

impl fmt::Display for KeyWord {
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          return fmt::Display::fmt(self.as_str(), f);
     }
}