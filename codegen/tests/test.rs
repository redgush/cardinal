extern crate cardinal_codegen;

use cardinal_codegen::entities::{AbiType, Named, Type};
use cardinal_codegen::function::{Function, FunctionSignature};
use cardinal_codegen::instbuilder::InstBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_function() {
        let sig = FunctionSignature::new();

        let mut func = Function::new("main".into(), sig);

        let v = func.declare_var("my_var".into(), AbiType(Named::new("int".into()), Type::Plain));
        
        let mut block0;
        {
            let block = func.create_block();
            block0 = func.use_block(block);
        }

        {
            let tmp0 = block0.iconst_int(21);
            let tmp1 = block0.iconst_int(21);
            let tmp2 = block0.iadd(tmp0, tmp1);

            let tmp3 = block0.iuse(v.named());
            block0.set(tmp3, tmp2);
        }
    }

}