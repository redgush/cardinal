//! Exposes types for function declarations and definitions.

use crate::entities::{AbiParam, AbiType, Block, Type, Variable};
use crate::instruction::{InstBlock, BlockType};
use std::collections::HashMap;

// A function that allows Cardinal to create instructions, variables and SSA values.
pub struct Function {

    // A list of variables declared in the function.
    pub variables: HashMap<String, AbiType>,

    /// A list of that blocks that may store instructions.
    pub blocks: Vec<InstBlock>,

    /// The name of the Function, used at compile time to generate correct code.
    pub name: String,

    /// The signature that the function uses.
    pub signature: FunctionSignature,

}

/// A function signature that allows the code generator to verify function calls and references.
pub struct FunctionSignature {

    /// A list of arguments in the function signature, which are checked at compile time to
    /// verify their validity.
    pub arguments: Vec<AbiParam>,

    /// A return value of the function.  Defaults to `void`.
    pub returns: AbiType,

}

impl FunctionSignature {

    pub fn new() -> Self {
        Self {
            arguments: vec![],
            returns: AbiType("void".into(), Type::Plain)
        }
    }

}

impl Function {

    /// Creates a new function from the given name and signature.
    pub fn new(name: String, sig: FunctionSignature) -> Self {
        Self {
            name,
            signature: sig,
            variables: HashMap::new(),
            blocks: vec![],
        }
    }

    /// Declares a variable at the start of the function.
    pub fn declare_var(&mut self, name: String, var_type: AbiType) -> Variable {
        let val = Variable(name.to_string());
        self.variables.insert(name, var_type);

        val
    }

    /// Uses a block.
    pub fn use_block(&mut self, block: Block) -> &mut InstBlock {
        self.blocks.get_mut(block.0 as usize).unwrap()
    }

    /// Creates a new empty block.
    pub fn create_block(&mut self) -> Block {
        let block = InstBlock {
            block_type: BlockType::Basic,
            blocks: vec![],
            else_block: None,
            elses: vec![],
            imports: vec![],
            insts: vec![],
            values: vec![],
        };

        let val = Block(self.blocks.len() as u32);
        self.blocks.push(block);

        val
    }

}