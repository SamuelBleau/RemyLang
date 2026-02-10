#[cfg(test)]

mod llvm_tests {
    use remylang::llvm_backend::LLVMCompilerContext;

    #[test]
    fn test_llvm_context_initialization() {
        let ctx = LLVMCompilerContext::new();
        let module = ctx.create_module("test_module");

        println!("Module created: {}", module.get_name().to_str().unwrap());
        assert_eq!(module.get_name().to_str().unwrap(), "test_module");
    }
}