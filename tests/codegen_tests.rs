use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use remylang::llvm_backend::CodeGenerator;

#[cfg(test)]
mod codegen_tests {
    use remylang::ast::{Expr, Literal};
    use remylang::llvm_backend::{LLVMCompilerContext, CodeGenerator};

    #[test]
    fn test_simple_literal_codegen() {
        let llvm_ctx = LLVMCompilerContext::new();
        let context = llvm_ctx.get_context();
        let module = llvm_ctx.create_module("test");
        let builder = llvm_ctx.create_builder();

        let mut gen = CodeGenerator::new(&context, module, builder);

        // Compiler : 42
        let expr = Expr::Literal(Literal::Number(42));
        assert!(gen.compile_expr(&expr).is_ok());

        // Afficher l'IR
        gen.print_ir();
    }
}