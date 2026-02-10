/*
 * -------------------------------------------------------------------------
 * RemyLang - LLVM Context Management
 * File : src/llvm_backend/context.rs
 *
 * Description :
 *   Handle the initialization and the life of LLVM's context
 *   The context is created at the beginning of the compilation
 *
 * Author  : Samuel 'Meeast' Bleau
 * Created : 2026-02-09
 *
 * -------------------------------------------------------------------------
*/

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;

/// Wrapper around LLVM's context
pub struct LLVMCompilerContext {
    context: Context,
}

impl LLVMCompilerContext {
    /// Create a new LLVM context
    pub fn new() -> Self {
        Self {
            context: Context::create(),
        }
    }

    ///Create a new module to compile a file
    pub fn create_module(&self, name: &str) -> Module<'_> {
        self.context.create_module(name)
    }

    /// Create a new builder to build the IE
    pub fn create_builder(&self) -> Builder<'_> {
        self.context.create_builder()
    }

    /// Get a reference to the LLVM context
    pub fn get_context(&self) -> &Context {
        &self.context
    }
}

impl Default for LLVMCompilerContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn llvm_context_creation() {
        let ctx = LLVMCompilerContext::new();
        let module = ctx.create_module("test");
        assert_eq!(module.get_name().to_str().unwrap(), "test");
    }
}
//TODO: Add tests in the test folder if it's better