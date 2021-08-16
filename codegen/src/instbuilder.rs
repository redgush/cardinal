//! Provides a trait for building instructions.

use crate::entities::{AbiType, Block, Named, NamedProperty, Type, Value, ValueInfo};
use crate::instruction::{InstBlock, InstructionInfo, Opcode};

/// A trait for building instructions.
pub trait InstBuilder {

    /// Defines a value in the InstBuilder's values table, then returning the value.
    fn create_value(&mut self, value: ValueInfo) -> Value;

    /// Pushes an instruction to the InstBuilder's instruction list.
    fn create_inst(&mut self, inst: InstructionInfo);

    /// Creates a block and registers it in the InstBuilder's block table, returning a reference
    /// to said block.
    fn create_block(&mut self, block: InstBlock) -> Block;

    /// Returns a pointer to the given block.
    fn use_block(&mut self, block: Block) -> &mut InstBlock;

    /// Adds a required import.
    fn require_import(&mut self, name: String);

    /// Creates an unsigned 64-bit integer constant.
    fn iconst_int(&mut self, value: u64) -> Value {
        self.create_value(ValueInfo::IntegerConstant(value))
    }

    /// Creates an unsigned 64-bit float constant.
    fn iconst_float(&mut self, value: f64) -> Value {
        self.create_value(ValueInfo::FloatConstant(value))
    }

    /// Creates an unsigned 64-bit double constant.
    fn iconst_double(&mut self, value: f64) -> Value {
        self.create_value(ValueInfo::DoubleConstant(value))
    }

    /// Creates a boolean constant.
    fn iconst_bool(&mut self, value: bool) -> Value {
        self.create_value(ValueInfo::BooleanConstant(value))
    }

    /// Creates a string constant.
    fn iconst_str(&mut self, value: String) -> Value {
        self.create_value(ValueInfo::StringConstant(value))
    }

    /// Creates a named reference constant.
    fn iconst_named(&mut self, name: String) -> Value {
        self.create_value(ValueInfo::Named(Named::new(name)))
    }

    /// Creates a named reference constant including the provided properties.
    fn iconst_named_props(&mut self, name: String, props: Vec<NamedProperty>) -> Value {
        self.create_value(ValueInfo::Named(Named::new_props(name, props)))
    }

    /// Returns a new basic NamedProperty.
    fn iconst_named_property(&self, name: String) -> NamedProperty {
        NamedProperty::Basic(name)
    }

    /// Returns a new static NamedProperty.
    fn iconst_named_static(&self, name: String) -> NamedProperty {
        NamedProperty::Static(name)
    }

    /// Returns a new pointer NamedProperty.
    fn iconst_named_pointer(&self, name: String) -> NamedProperty {
        NamedProperty::Pointer(name)
    }

    /// Returns a new index NamedProperty.
    fn iconst_named_index(&self, name: Value) -> NamedProperty {
        NamedProperty::Index(name)
    }

    /// Creates a C target specific boolean type.  Requires the `stdbool.h` standard library
    /// to be provided by your C compiler.
    fn ctype_bool(&mut self) -> AbiType {
        // The C bool type requires `stdbool.h` to be imported.
        self.require_import("stdbool.h".into());
        AbiType("bool".into(), Type::Plain)
    }

    /// Creates a C target specific 8 bit unsigned integer type.  Requires the `stdint.h`
    /// standard library to be provided by your C compiler.
    fn ctype_uint8(&mut self) -> AbiType {
        self.require_import("stdint.h".into());
        AbiType("uint8_t".into(), Type::Plain)
    }

    /// Creates a C target specific 16 bit unsigned integer type.  Requires the `stdint.h`
    /// standard library to be provided by your C compiler.
    fn ctype_uint16(&mut self) -> AbiType {
        self.require_import("stdint.h".into());
        AbiType("uint16_t".into(), Type::Plain)
    }

    /// Creates a C target specific 32 bit unsigned integer type.  Requires the `stdint.h`
    /// standard library to be provided by your C compiler.
    fn ctype_uint32(&mut self) -> AbiType {
        self.require_import("stdint.h".into());
        AbiType("uint32_t".into(), Type::Plain)
    }

    /// Creates a C target specific 64 bit unsigned integer type.  Requires the `stdint.h`
    /// standard library to be provided by your C compiler.
    fn ctype_uint64(&mut self) -> AbiType {
        self.require_import("stdint.h".into());
        AbiType("uint64_t".into(), Type::Plain)
    }

    /// Creates a C target specific size that scales to the target's architecture.  For 32-bit
    /// processors, this is the same as a `uint32` and for 64-bit architectures this is the
    /// same as a `uint64`.
    fn ctype_usize(&mut self) -> AbiType {
        self.require_import("stdint.h".into());
        AbiType("uintptr".into(), Type::Plain)
    }

    /// Creates a C target specific 8 bit integer type.  Requires the `stdint.h`
    /// standard library to be provided by your C compiler.
    fn ctype_int8(&mut self) -> AbiType {
        self.require_import("stdint.h".into());
        AbiType("int8_t".into(), Type::Plain)
    }

    /// Creates a C target specific 16 bit integer type.  Requires the `stdint.h`
    /// standard library to be provided by your C compiler.
    fn ctype_int16(&mut self) -> AbiType {
        self.require_import("stdint.h".into());
        AbiType("uint16_t".into(), Type::Plain)
    }

    /// Creates a C target specific 32 bit integer type.  Requires the `stdint.h`
    /// standard library to be provided by your C compiler.
    fn ctype_int32(&mut self) -> AbiType {
        self.require_import("stdint.h".into());
        AbiType("int32_t".into(), Type::Plain)
    }

    /// Creates a C target specific 64 bit integer type.  Requires the `stdint.h`
    /// standard library to be provided by your C compiler.
    fn ctype_int64(&mut self) -> AbiType {
        self.require_import("stdint.h".into());
        AbiType("int64_t".into(), Type::Plain)
    }

    /// Creates a C target specific size that scales to the target's architecture.  For 32-bit
    /// processors, this is the same as an `int32` and for 64-bit architectures this is the
    /// same as an `int64`.
    fn ctype_isize(&mut self) -> AbiType {
        self.require_import("stdint.h".into());
        AbiType("intptr".into(), Type::Plain)
    }

    /// A C-specific character type.
    fn ctype_char(&mut self) -> AbiType {
        AbiType("char".into(), Type::Plain)
    }

    /// Adds two values together and returns the sum of the expression.
    fn iadd(&mut self, l: Value, r: Value) -> Value {
        self.create_value(ValueInfo::Instruction(InstructionInfo {
            opcode: Opcode::Add,
            arguments: vec![l, r]
        }))
    }

    /// Subtracts two values together and returns the sum of the expression.
    fn isub(&mut self, l: Value, r: Value) -> Value {
        self.create_value(ValueInfo::Instruction(InstructionInfo {
            opcode: Opcode::Sub,
            arguments: vec![l, r]
        }))
    }

    /// Multiplies two values together and returns the sum of the expression.
    fn imul(&mut self, l: Value, r: Value) -> Value {
        self.create_value(ValueInfo::Instruction(InstructionInfo {
            opcode: Opcode::Mul,
            arguments: vec![l, r]
        }))
    }

    /// Divides two values together and returns the sum of the expression.
    fn idiv(&mut self, l: Value, r: Value) -> Value {
        self.create_value(ValueInfo::Instruction(InstructionInfo {
            opcode: Opcode::Div,
            arguments: vec![l, r]
        }))
    }

    /// Divides two values together and returns the remainder of the expression.  Equivalent to
    /// the `%` (modulus) operator.
    fn imod(&mut self, l: Value, r: Value) -> Value {
        self.create_value(ValueInfo::Instruction(InstructionInfo {
            opcode: Opcode::Mod,
            arguments: vec![l, r]
        }))
    }

    /// Returns the results of the Bitwise AND operation.
    fn ibit_and(&mut self, l: Value, r: Value) -> Value {
        self.create_value(ValueInfo::Instruction(InstructionInfo {
            opcode: Opcode::BitAnd,
            arguments: vec![l, r]
        }))
    }

    /// Returns the results of the Bitwise OR operation.
    fn ibit_or(&mut self, l: Value, r: Value) -> Value {
        self.create_value(ValueInfo::Instruction(InstructionInfo {
            opcode: Opcode::BitOr,
            arguments: vec![l, r]
        }))
    }

    /// Returns the results of the Bitwise XOR operation.
    fn ibit_xor(&mut self, l: Value, r: Value) -> Value {
        self.create_value(ValueInfo::Instruction(InstructionInfo {
            opcode: Opcode::BitXor,
            arguments: vec![l, r]
        }))
    }

    /// Returns the results of the Bitwise NOT operation.
    fn ibit_not(&mut self, l: Value) -> Value {
        self.create_value(ValueInfo::Instruction(InstructionInfo {
            opcode: Opcode::BitNot,
            arguments: vec![l]
        }))
    }

    /// Returns the results of the Bitwise left shift operation.
    fn ibit_left(&mut self, l: Value, r: Value) -> Value {
        self.create_value(ValueInfo::Instruction(InstructionInfo {
            opcode: Opcode::BitLeft,
            arguments: vec![l, r]
        }))
    }

    /// Returns the results of the Bitwise right shift operation.
    fn ibit_right(&mut self, l: Value, r: Value) -> Value {
        self.create_value(ValueInfo::Instruction(InstructionInfo {
            opcode: Opcode::BitRight,
            arguments: vec![l, r]
        }))
    }

    /// Tests if two values are equal to eachother.  Returns a boolean value with the result of
    /// the operation.
    fn itest_eq(&mut self, l: Value, r: Value) -> Value {
        self.create_value(ValueInfo::Instruction(InstructionInfo {
            opcode: Opcode::TestEq,
            arguments: vec![l, r]
        }))
    }

    /// Tests if two values are not equal to eachother.  Returns a boolean value with the result
    /// of the operation.
    fn itest_neq(&mut self, l: Value, r: Value) -> Value {
        self.create_value(ValueInfo::Instruction(InstructionInfo {
            opcode: Opcode::TestNeq,
            arguments: vec![l, r]
        }))
    }

    /// Tests if the first value is greater than the second.  Returns a boolean value with the
    /// result of the operation.
    fn itest_gt(&mut self, l: Value, r: Value) -> Value {
        self.create_value(ValueInfo::Instruction(InstructionInfo {
            opcode: Opcode::TestGt,
            arguments: vec![l, r]
        }))
    }

    /// Tests if the first value is greater than or equal to the second.  Returns a boolean
    /// value with the result of the operation.
    fn itest_gt_eq(&mut self, l: Value, r: Value) -> Value {
        self.create_value(ValueInfo::Instruction(InstructionInfo {
            opcode: Opcode::TestGtEq,
            arguments: vec![l, r]
        }))
    }

    /// Tests if the first value is less than the second.  Returns a boolean value with the
    /// result of the operation.
    fn itest_lt(&mut self, l: Value, r: Value) -> Value {
        self.create_value(ValueInfo::Instruction(InstructionInfo {
            opcode: Opcode::TestLt,
            arguments: vec![l, r]
        }))
    }

    /// Tests if the first value is less than or equal to the second.  Returns a boolean
    /// value with the result of the operation.
    fn itest_lt_eq(&mut self, l: Value, r: Value) -> Value {
        self.create_value(ValueInfo::Instruction(InstructionInfo {
            opcode: Opcode::TestLtEq,
            arguments: vec![l, r]
        }))
    }

    /// Negates a boolean value.
    fn inot(&mut self, l: Value) -> Value {
        self.create_value(ValueInfo::Instruction(InstructionInfo {
            opcode: Opcode::Not,
            arguments: vec![l]
        }))
    }

    /// Returns if either value is equal to true.
    fn ior(&mut self, l: Value, r: Value) -> Value {
        self.create_value(ValueInfo::Instruction(InstructionInfo {
            opcode: Opcode::Or,
            arguments: vec![l, r]
        }))
    }

    /// Returns if both values are equal to true.
    fn iand(&mut self, l: Value, r: Value) -> Value {
        self.create_value(ValueInfo::Instruction(InstructionInfo {
            opcode: Opcode::And,
            arguments: vec![l, r]
        }))
    }

    /// Unconditionally jumps to a certain block.
    fn jmp(&mut self, block: Block) {
        let b = self.create_value(ValueInfo::Block(block));
        self.create_inst(InstructionInfo {
            opcode: Opcode::Jmp,
            arguments: vec![b]
        });
    }

    /// Uses a named reference as a value.
    fn iuse(&mut self, named: Named) -> Value {
        self.create_value(ValueInfo::Named(named))
    }

    /// Sets a value, equivalent to the `=` assignment operator in most programming languages.
    fn set(&mut self, k: Value, v: Value) {
        self.create_inst(InstructionInfo {
            opcode: Opcode::Set,
            arguments: vec![k, v]
        });
    }

    /// Makes a function call with the specified function name and arguments.
    fn call(&mut self, k: Value, args: Vec<Value>) {
        let mut v = vec![k];
        v.append(&mut args.clone());
        self.create_inst(InstructionInfo {
            opcode: Opcode::Call,
            arguments: v
        });
    }

    /// Makes a function call and returns the value that the function call returns.
    fn icall(&mut self, k: Value, args: Vec<Value>) -> Value {
        let mut v = vec![k];
        v.append(&mut args.clone());
        self.create_value(ValueInfo::Instruction(InstructionInfo {
            opcode: Opcode::Call,
            arguments: v
        }))
    }

    /// Returns a value from the function that this InstBuilder resides in.
    fn return_(&mut self, v: Value) {
        self.create_inst(InstructionInfo {
            opcode: Opcode::Ret,
            arguments: vec![v]
        });
    }

    /// Returns and exits the function, without returning a value.  The function should have a
    /// return type of `void`.
    fn return_none(&mut self) {
        self.create_inst(InstructionInfo {
            opcode: Opcode::Ret,
            arguments: vec![]
        });
    }

}