// This is a module that handles internal classes and types.
#[derive(Clone, PartialEq, Debug)]
pub struct Type {
	pub name: String,
	pub statement: TypeDefinition
}

#[derive(Clone, PartialEq, Debug)]
pub struct TypeDefinition {
	pub func: TypeSignature
}

#[derive(Clone, PartialEq, Debug)]
pub struct TypeSignature {
	
}