//! Entities that the code generator use.

use crate::instruction::InstructionInfo;

/// An opaque reference to a Cardinal SSA value.  These can be used as instruction parameters,
/// if a value is not used, it will not be included in the generated code.
#[derive(Clone, PartialEq)]
pub struct Value(pub u32);

/// An opaque reference to a Cardinal IR block.
#[derive(Clone, PartialEq)]
pub struct Block(pub u32);

/// An opaque reference to a Cardinal variable.
#[derive(Clone, PartialEq)]
pub struct Variable(pub String);

impl Variable {

    pub fn named(&self) -> Named {
        Named::new(self.0.to_string())
    }

}

/// An opaque reference to a Cardinal global variable.
#[derive(Clone, PartialEq)]
pub struct GlobalVariable(pub String);

impl GlobalVariable {

    pub fn named(&self) -> Named {
        Named::new(self.0.to_string())
    }

}

/// Different types of types that can be declared.
#[derive(Clone, PartialEq)]
pub enum Type {

    /// A plain type, such as `int` or `double`.
    Plain,

    /// An array type, such as `char[]` or `int[]`. `-1` should be used as the size argument if
    /// a size should be declared implicitly.
    Array(isize),

    /// A pointer type, such as `char*` or `int*`.
    Pointer,

}

/// An ABI type.
#[derive(Clone, PartialEq)]
pub struct AbiType(pub Named, pub Type);

/// Properties of a `Named` struct that may be basic properties, static properties, pointer
/// properties or index properties.
#[derive(Clone, PartialEq)]
pub enum NamedProperty {

    /// A basic property, for example, `Named.Basic`.
    Basic(String),

    /// A reference to a static property, which is not supported by the C backend. For example,
    /// `Named::Basic` in C++ or Rust.
    Static(String),

    /// A pointer property, for example, `Named->Pointer`.
    Pointer(String),

    /// An index in a named value, for example, `Named[Index]`.  It uses a value that is
    /// determined at compile time.
    Index(Value),

}

/// Used as a named reference to an object.
#[derive(Clone, PartialEq)]
pub struct Named {

    /// The name of the first object in the reference.
    pub name: String,

    /// A list of indexes performed on the Named reference.
    /// 
    /// For example,
    /// ```c
    /// Named.Index1->Index2
    /// ```
    pub properties: Vec<NamedProperty>

}

impl Named {

    /// Creates a Named object with no properties.
    pub fn new(name: String) -> Self {
        Self {
            name,
            properties: vec![]
        }
    }

    /// Creates a Named object with a list of properties.
    pub fn new_props(name: String, properties: Vec<NamedProperty>) -> Self {
        Self {
            name,
            properties
        }
    }

}


/// Information about a value.
pub enum ValueInfo {

    /// An integer constant.
    IntegerConstant(u64),

    /// A floating point number constant.
    FloatConstant(f64),

    /// A double constant.
    DoubleConstant(f64),

    /// A boolean constant.
    BooleanConstant(bool),

    /// A string constant.
    StringConstant(String),

    /// A named reference.
    Named(Named),

    /// A value reference to a block.
    Block(Block),

    /// A pointer to an instruction.
    Instruction(InstructionInfo),

}