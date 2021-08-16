//! The top-level `lib.rs` for the Cardinal code generator.

pub mod entities;
pub mod function;
pub mod instbuilder;
pub mod instruction;
pub mod ir;
pub mod module;

pub use entities::{AbiType, Block, GlobalVariable, Named, NamedProperty, Type, Value, Variable};
pub use function::{Function, FunctionSignature};
pub use module::Module;