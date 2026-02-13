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

        let expr = Expr::Literal(Literal::Number(42));
        assert!(gen.compile_expr(&expr).is_ok());

        gen.print_ir();
    }

    #[test]
    fn test_variable_allocation() {
        let llvm_ctx = LLVMCompilerContext::new();
        let context = llvm_ctx.get_context();
        let module = llvm_ctx.create_module("test_vars");
        let builder = llvm_ctx.create_builder();

        let mut gen = CodeGenerator::new(context, module, builder);

        // Create a function context for allocations to work
        gen.create_test_function("test_fn");

        let stmt = remylang::ast::Stmt::Let {
            name: "x".to_string(),
            type_annotation: None,
            value: Expr::Literal(Literal::Number(42)),
        };

        assert!(gen.compile_statement(&stmt).is_ok());
        gen.print_ir();
    }

    #[test]
    fn test_binary_operations() {
        let llvm_ctx = LLVMCompilerContext::new();
        let context = llvm_ctx.get_context();
        let module = llvm_ctx.create_module("test_binops");
        let builder = llvm_ctx.create_builder();

        let mut gen = CodeGenerator::new(context, module, builder);

        // Create a function context for the builder to have a valid insertion point
        gen.create_test_function("test_fn");

        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Number(5))),
            op: remylang::ast::BinaryOp::Add,
            right: Box::new(Expr::Literal(Literal::Number(3))),
        };

        assert!(gen.compile_expr(&expr).is_ok());
        gen.print_ir();
    }
}