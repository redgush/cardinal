//! A module that can contain Cardinal functions and global data.

//! Exposes types for function declarations and definitions.

use crate::entities::{AbiType, GlobalVariable};
use crate::function::{Function, FunctionSignature};
use std::collections::HashMap;

// A module that contains Cardinal functions and global data.
pub struct Module {

    /// A list of functions defined in the module.
    pub functions: HashMap<String, Function>,

    /// A list of global data variables declared in the module.
    pub data: HashMap<String, AbiType>,

}

impl Module {

    /// Creates a new empty module.
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            data: HashMap::new(),
        }
    }

    /// Declares a function with the specified name.
    pub fn declare_function(&mut self, name: String) {
        let func = Function::new(name.to_string(), FunctionSignature::new());
        self.functions.insert(name, func);
    }

    /// Defines a function with the specified name.
    pub fn define_function(&mut self, func: Function) {
        self.functions.insert(func.name.to_string(), func);
    }

    /// Declares a variable in the module.
    pub fn declare_variable(&mut self, name: String, val_type: AbiType) -> GlobalVariable {
        self.data.insert(name.to_string(), val_type);
        GlobalVariable(name)
    }

}