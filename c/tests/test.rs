extern crate cardinal_c;
extern crate cardinal_codegen;

use cardinal_c::CBackend;
use cardinal_codegen::entities::{AbiParam, AbiType, Named, Type};
use cardinal_codegen::function::{Function, FunctionSignature};
use cardinal_codegen::instbuilder::InstBuilder;
use cardinal_codegen::Module;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_function() {
        let mut m = Module::new();

        let sig = FunctionSignature::new();
        let mut func = Function::new("main".into(), sig);

        let v = func.declare_var("my_var".into(), AbiType("int".into(), Type::Plain));
        
        let block0;
        {
            let block = func.create_block();
            block0 = func.use_block(block);
        }

        block0.require_import("stdio.h".into());

        {
            let tmp0 = block0.iconst_int(21);
            let tmp1 = block0.iconst_int(21);
            let tmp2 = block0.iadd(tmp0, tmp1);

            let tmp3 = block0.iuse(v.named());
            block0.set(tmp3, tmp2);
        }

        {
            let tmp0 = Named::new("printf".into());
            let tmp1 = block0.iuse(tmp0);
            let tmp2 = block0.iuse(v.named());

            let str = block0.iconst_str("%d".into());

            block0.call(tmp1, vec![str, tmp2]);
        }

        m.define_function(func);

        let mut gen = CBackend::new(m);
        println!("{}", gen.emit());
    }

}