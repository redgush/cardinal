//! A module for compiling Cardinal IR to functioning C code.

use cardinal_codegen::entities::{AbiType, Named, NamedProperty, Type, Value, ValueInfo};
use cardinal_codegen::function::{Function};
use cardinal_codegen::instruction::{InstructionInfo, InstBlock, Opcode};
use cardinal_codegen::module::Module;

/// Cardinal's C backend for the code generator.
pub struct CBackend {

    /// The module to emit C code from.
    module: Module,

    /// A list of C header files to include at compile time.
    imports: Vec<String>,

}

impl CBackend {

    /// Creates a new CBackend that will generate C code from the provided Cardinal IR module.
    pub fn new(module: Module) -> Self {
        Self {
            module,
            imports: vec![],
        }
    }
    
    /// Displays an instruction.
    fn display_instruction(&self, inst: &InstructionInfo, block: &InstBlock) -> String {
        match inst.opcode {
            Opcode::Add => {
                self.display_value(inst.arguments[0], block) + " + " + &self.display_value(inst.arguments[1], block)
            },
            Opcode::Sub => {
                self.display_value(inst.arguments[0], block) + " - " + &self.display_value(inst.arguments[1], block)
            },
            Opcode::Mul => {
                self.display_value(inst.arguments[0], block) + " * " + &self.display_value(inst.arguments[1], block)
            },
            Opcode::Div => {
                self.display_value(inst.arguments[0], block) + " / " + &self.display_value(inst.arguments[1], block)
            },
            Opcode::Mod => {
                self.display_value(inst.arguments[0], block) + " % " + &self.display_value(inst.arguments[1], block)
            },
            Opcode::BitAnd => {
                self.display_value(inst.arguments[0], block) + " & " + &self.display_value(inst.arguments[1], block)
            },
            Opcode::BitOr => {
                self.display_value(inst.arguments[0], block) + " | " + &self.display_value(inst.arguments[1], block)
            },
            Opcode::BitXor => {
                self.display_value(inst.arguments[0], block) + " ^ " + &self.display_value(inst.arguments[1], block)
            },
            Opcode::BitNot => {
                "~".to_string() + &self.display_value(inst.arguments[0], block)
            },
            Opcode::BitLeft => {
                self.display_value(inst.arguments[0], block) + " << " + &self.display_value(inst.arguments[1], block)
            },
            Opcode::BitRight => {
                self.display_value(inst.arguments[0], block) + " >> " + &self.display_value(inst.arguments[1], block)
            },
            Opcode::TestEq => {
                self.display_value(inst.arguments[0], block) + " == " + &self.display_value(inst.arguments[1], block)
            },
            Opcode::TestNeq => {
                self.display_value(inst.arguments[0], block) + " != " + &self.display_value(inst.arguments[1], block)
            },
            Opcode::TestGt => {
                self.display_value(inst.arguments[0], block) + " > " + &self.display_value(inst.arguments[1], block)
            },
            Opcode::TestGtEq => {
                self.display_value(inst.arguments[0], block) + " >= " + &self.display_value(inst.arguments[1], block)
            },
            Opcode::TestLt => {
                self.display_value(inst.arguments[0], block) + " < " + &self.display_value(inst.arguments[1], block)
            },
            Opcode::TestLtEq => {
                self.display_value(inst.arguments[0], block) + " <= " + &self.display_value(inst.arguments[1], block)
            },
            Opcode::Not => {
                "!".to_string() + &self.display_value(inst.arguments[0], block)
            },
            Opcode::Or => {
                self.display_value(inst.arguments[0], block) + " || " + &self.display_value(inst.arguments[1], block)
            },
            Opcode::And => {
                self.display_value(inst.arguments[0], block) + " && " + &self.display_value(inst.arguments[1], block)
            },
            Opcode::Jmp => {
                "goto".to_string() + &self.display_value(inst.arguments[0], block)
            },
            Opcode::Set => {
                self.display_value(inst.arguments[0], block) + " = " + &self.display_value(inst.arguments[1], block)
            },
            Opcode::Call => {
                let mut args = vec![];

                let mut i = 1;
                while i < inst.arguments.len() {
                    args.push(self.display_value(inst.arguments[i], block));
                    i += 1;
                }

                self.display_value(inst.arguments[0], block) + "(" + &args.join(", ") + ")"
            },
            Opcode::Ret => {
                "return ".to_string() + &self.display_value(inst.arguments[0], block)
            },
        }
    }

    /// Displays a value from a block.
    fn display_value(&self, val: Value, block: &InstBlock) -> String {
        let v = &block.values[val.0 as usize];

        match v {
            ValueInfo::Block(b) => {
                format!("block{}", b.0)
            },
            ValueInfo::BooleanConstant(b) => {
                b.to_string()
            },
            ValueInfo::DoubleConstant(b) => {
                b.to_string()
            },
            ValueInfo::IntegerConstant(b) => {
                b.to_string()
            },
            ValueInfo::FloatConstant(b) => {
                b.to_string()
            },
            ValueInfo::Instruction(b) => {
                self.display_instruction(b, block)
            },
            ValueInfo::Named(b) => {
                self.display_named(b, block)
            },
            ValueInfo::StringConstant(b) => {
                "\"".to_string() + &b + "\""
            },
            ValueInfo::CharConstant(b) => {
                "'".to_string() + &b + "'"
            },
        }
    }

    fn display_named(&self, named: &Named, block: &InstBlock) -> String {
        let mut name = named.name.to_string();

        for item in &named.properties {
            match item {
                NamedProperty::Basic(n) => {
                    name.push('.');
                    name.push_str(&n);
                },
                NamedProperty::Index(n) => {
                    name.push('[');
                    name.push_str(&self.display_value(*n, block));
                    name.push(']');
                },
                NamedProperty::Pointer(n) => {
                    name.push_str("->");
                    name.push_str(&n);
                },
                NamedProperty::Static(_) => {
                    panic!("Static indexing isn't allowed with the C emitter.");
                    //name.push_str("::");
                    //name.push_str(&n);
                }
            }
        }

        name.to_string()
    }

    fn display_abitype(&self, abitype: &AbiType) -> String {
        let t = &abitype.1;

        match t {
            Type::Plain => {
                abitype.0.to_string()
            },
            Type::Array(n) => {
                if n > &-1 {
                    abitype.0.to_string() + "[]".into()
                } else {
                    abitype.0.to_string() + &format!("[{}]", n)
                }
            },
            Type::Pointer => {
                abitype.0.to_string() + "*".into()
            }
        }
    }

    /// Compiles a single function into C code.
    pub fn compile_function(&self, func: &Function) -> (String, Vec<String>) {
        let mut args = vec![];

        for item in &func.signature.arguments {
            args.push(format!("{} {}", self.display_abitype(&item.1), item.0));
        }

        let mut header = format!("{} {}({})", self.display_abitype(&func.signature.returns), func.name, args.join(", "));
        if func.blocks.len() == 0 {
            return (header, vec![]);
        } else {
            header.push_str(" {\n");
            let mut imports = vec![];
            let mut insts = vec![];

            for (i, v) in func.blocks.iter().enumerate() {
                let mut block = vec![];
                imports.append(&mut v.imports.clone());
                for inst in &v.insts {
                    block.push(self.display_instruction(inst, v));
                }
                
                insts.push(format!("block{}: {{\n", i) + &block.join(";\n") + ";\n}\n");
            }

            let mut vars = vec![];

            for var in &func.variables {
                vars.push(self.display_abitype(&var.1) + " " + var.0);
            }

            header.push_str(&(vars.join(";\n")));

            if vars.len() > 0 {
                header.push_str(";\n");
            }

            header.push_str(&(insts.join("\n")));

            header.push('}');

            return (header, imports);
        }

        
    }

    /// Compiles the provided module into a `String` of valid C code.
    pub fn emit(&mut self) -> String {
        let mut str = String::new();

        let mut f = vec![];

        for item in &self.module.functions {
            let x = item.1;
            let mut res = self.compile_function(x);
            f.push(res.0);

            self.imports.append(&mut res.1);
        }

        let mut includes = vec![];

        for item in &self.imports {
            includes.push(format!("#include <{}>", item));
        }

        str.push_str(&includes.join("\n"));
        str.push('\n');

        str.push_str(&f.join("\n"));

        str
    }

}