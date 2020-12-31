use std::{ fmt, error };

// These keywords are reserved.
// If you use these as identifiers you will be yelled at.
#[derive(Debug)]
pub enum KeyWord {
     // Keywords available in all versions of typesharp.

     // Await, used for synchronizing load.
     Await,

     // Async, allows for asynchronous programming
     Async,

     // As, used for type casting
     As,

     // Used to break out of statements like "if" etc.
     Break,

     //
     Case,

     // Used in try...catch blocks, catches errors.
     Catch,

     // OOP, reserved word for class
     Class,

     // Continue, used in loops to continue the loop
     Continue,

     // Constants, non-mutable constant vars.
     Const,

     // Defaults, used in switch statements.
     Default,

     // Delete (may be removed), removes a var. (this may be removed due to memory safety)
     // See issue https://github.com/TypeSharp/Typesharp/issues/1
     Delete,

     // Used in if...else blocks.
     Else,

     // Enums, mainly just syntax sugar
     Enum,

     // Exports module
     // THIS DOES NOT HAVE THE SAME USE CASE AS JS
     // Exporting a module makes it global.
     Export,

     // Creates an external function (used for FFI or dynlibs)
     Extern,

     // Class inheritance for extending
     Extends,

     // False
     False,

     // For loops, eg: for let i in blah {}
     For,

     // Function keyword, makes a function
     Function,

     // Reserved macro for function
     Fn,

     // If keyword
     If,

     // In, used in for..in loops
     In,

     // Checks an instance of a given object.
     InstanceOf,

     // Imports a public module
     // Use case is SIMILAR to JS
     Import,

     // Creates a mutable identifier.
     // Exactly equal to: let mut something: u8 = 0; in rust.
     Let,

     // Creates an instance of a class
     New,

     // Used in for...of loops
     Of,

     // Creates a package. Similar to `mod` keyword in rust.
     Package,

     // Return
     Return,

     // Self, used in classes, and functions for getting "it's self"
     SelfKeyword,

     // Static, used to create a static reference
     Static,

     // Super, accesses the parent class in typesharp
     // Eg:
     // class Alphabet {}
     // class A extends Alphabet, Numeric {
     //     public constructor() {
     //        super(); // alphabet
     //        super as Numeric(0); // numeric class.
     //     }
     // }
     Super,

     // Switch, used to catch cases, maybe implement match?
     Switch,

     // True
     True,

     // Traits, traits in typesharp are very similar to php
     //   trait Living {
     //        private name: string = "";
     //        public function getName(): string {
     //             return this.name;
     //        }
     //        public function talk(message: string) {
     //             print!(`${this.name} says: "${message}"`);
     //        }
     //   }
     //   class Animal {
     //        use Living;
     //        public constructor(name: string) {
     //             this.name = name;
     //        }
     //   }
     //   let dog: Animal = new Animal("Dog");
     //   dog.talk("Woof!");
     //   dog.getName(); // Dog
     Trait,

     // An accessor for a class, access as "self" but not statically.
     This,

     // Throws an error.
     Throw,

     // Type, declares a type
     Type,

     // Used in Try...catch loops
     Try,

     // Where, (Might implement)
     Where,

     // While loops.
     While,

     // Abstractions
     Abstract,
     Become,
     Do,
     Finally,
     Override,
     Typeof,
     Yield,
     Public,
     Pub,
     Private,
     Protected,

     // Types
     Union,
     Implements,
}

impl KeyWord {
     pub fn as_str(&self) -> &'static str {
          return "Not implemented";
          // To-Do
          // match self {
          // }
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
          //return fmt::Display::fmt(self.as_str(), f);
          return write!(f, "Not Implemented");
     }
}