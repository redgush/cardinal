//! Information about possible Cardinal instructions.

use crate::entities::{Block, Value, ValueInfo};
use crate::instbuilder::InstBuilder;

pub enum Opcode {

    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitAnd,
    BitOr,
    BitXor,
    BitLeft,
    BitRight,
    BitNot,
    TestEq,
    TestNeq,
    TestGt,
    TestGtEq,
    TestLt,
    TestLtEq,
    Not,
    Or,
    And,
    Jmp,
    Set,
    Call,
    Ret,

}

/// Information about an instruction or operation.
pub struct InstructionInfo {

    /// The opcode of the instruction.
    pub opcode: Opcode,

    /// The arguments provided with the instruction.
    pub arguments: Vec<Value>,

}

/// A block type for creating different kinds of blocks.
pub enum BlockType {

    /// A basic IF type that uses a value as an expression.
    If(Value),

    /// A basic block with no conditions.
    Basic,

}

/// A block for instruction building.
pub struct InstBlock {

    /// The type of the block.
    pub block_type: BlockType,

    /// A list of elseif statements for the block, if any.
    pub elses: Vec<InstBlock>,

    /// An else_block for If blocks.
    pub else_block: Option<Box<InstBlock>>,

    /// A list of values defined in the block.
    pub values: Vec<ValueInfo>,

    /// A list of instructions in the block.
    pub insts: Vec<InstructionInfo>,

    /// A list of imports in the block.
    pub imports: Vec<String>,

    /// A list of nested blocks in the block.
    pub blocks: Vec<InstBlock>,

}

impl InstBuilder for InstBlock {

    fn require_import(&mut self, name: String) {
        if !self.imports.contains(&name) {
            self.imports.push(name);
        }
    }

    fn create_value(&mut self, value: ValueInfo) -> Value {
        let val = Value(self.values.len() as u32);
        self.values.push(value);

        val
    }

    fn create_block(&mut self, block: InstBlock) -> Block {
        let val = Block(self.blocks.len() as u32);
        self.blocks.push(block);

        val
    }

    fn create_inst(&mut self, inst: InstructionInfo) {
        self.insts.push(inst);
    }

    fn use_block(&mut self, block: Block) -> &mut InstBlock {
        self.blocks.get_mut(block.0 as usize).unwrap()
    }

}