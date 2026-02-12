/*
 * -------------------------------------------------------------------------
 * RemyLang - LLVM Code Generation
 * File : src/llvm_backend/codegen.rs
 *
 * Description :
 *   Principal LLVM code generator
 *   Transform the AST into LLVM IR
 *
 * Author  : Samuel 'Meeast' Bleau
 * Created : 2026-02-11
 *
 * -------------------------------------------------------------------------
*/
use std::collections::HashMap;
use std::cell::RefCell;
use crate::ast::{Expr, Literal, Stmt};
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{BasicValueEnum, PointerValue};
use inkwell::types::BasicTypeEnum;

///
pub struct CodeGenerator<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    variables: RefCell<HashMap<String, (PointerValue<'ctx>, BasicTypeEnum<'ctx>)>>,
}

impl<'ctx> CodeGenerator<'ctx> {
    /// Create a new code generator
    pub fn new(context: &'ctx Context, module: Module<'ctx>, builder: Builder<'ctx>) -> Self {
        Self {
            context,
            module,
            builder,
            variables: RefCell::new(HashMap::new()),
        }
    }
    pub fn create_test_function(&mut self, name: &str) {
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&[], false);
        let function = self.module.add_function(name, fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);
    }

    /// Compile a list of statements
    pub fn compile_program(&mut self, stmts: &[Stmt]) -> Result<(), String> {
        for stmt in stmts {
            self.compile_statement(stmt)?;
        }
        Ok(())
    }

    /// Compile a statement
    pub fn compile_statement(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Expression(expr) => {
                self.compile_expr(expr);
                Ok(())
            }
            Stmt::Let { name, type_annotation: _, value } => {
                self.compile_let(name, value)
            }
            //TODO : Implement other statements
            _ => Err("Statement not yet implemented".to_string())
        }
    }

    /// Compile an expression
    pub fn compile_expr(&self, expr: &Expr) -> Result<BasicValueEnum, String> {
        match expr {
            Expr::Literal(lit) => self.compile_literal(lit),
            Expr::Variable(name) => self.compile_variable(name),
            //TODO: Variables, operators, calls, etc...
            _ => Err("Expression not yet implemented".to_string())
        }
    }

    /// Compile a variable declaration
    pub fn compile_let(&mut self, name: &str, value: &Expr) -> Result<(), String> {
        let llvm_value = self.compile_expr(value)?;
        let value_type = llvm_value.get_type();

        let alloca = self.builder.build_alloca(value_type, name)
            .map_err(|e| format!("{:?}", e))?;

        self.builder.build_store(alloca, llvm_value)
            .map_err(|e| format!("{:?}", e))?;

        let alloca_ctx: PointerValue<'ctx> = unsafe { std::mem::transmute(alloca) };
        let value_type_ctx: BasicTypeEnum<'ctx> = unsafe { std::mem::transmute(value_type) };

        self.variables.borrow_mut().insert(name.to_string(), (alloca_ctx, value_type_ctx));

        Ok(())
    }

    /// Load a variable
    pub fn compile_variable(&self, name: &str) -> Result<BasicValueEnum, String> {
        let variables = self.variables.borrow();
        let (ptr, value_type) = variables
            .get(name)
            .ok_or_else(|| format!("Undefined variable: {}", name))?;

        let loaded_value = self.builder.build_load(*value_type, *ptr, name)
            .map_err(|e| format!("Failed to load: '{:?}'", e))?;

        Ok(loaded_value)
    }

    /// Compile a literal
    pub fn compile_literal(&self, lit: &Literal) -> Result<BasicValueEnum, String> {
        match lit {
            Literal::Number(n) => {
                let llvm_int = self.context.i64_type().const_int(*n as u64, true);
                Ok(llvm_int.into())
            }
            Literal::Bool(b) => {
                let llvm_bool = self.context.bool_type().const_int(if *b { 1 } else { 0 }, false);
                Ok(llvm_bool.into())
            }
            Literal::Char(c) => {
                let llvm_char = self.context.i8_type().const_int(*c as u64, false);
                Ok(llvm_char.into())
            }
            Literal::String(s) => {
                let llvm_string = self.context.const_string(s.as_bytes(), true);
                let global = self.module.add_global(llvm_string.get_type(), None, "str_const");
                global.set_initializer(&llvm_string);
                Ok(global.as_pointer_value().into())
            }
        }
    }

    // Get the compiled module
    pub fn get_module(&self) -> &Module<'ctx> {
        &self.module
    }

    /// Print the generated IR
    pub fn print_ir(&self) {
        println!("\n === Generated LLVM IR ===");
        println!("{}", self.module.print_to_string().to_string())
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llvm_backend::LLVMCompilerContext;

    #[test]
    fn test_compile_integer_literal() {
        let llvm_ctx = LLVMCompilerContext::new();
        let context = llvm_ctx.get_context();
        let module = llvm_ctx.create_module("test");
        let builder = llvm_ctx.create_builder();

        let mut gen = CodeGenerator::new(&context, module, builder);
        let expr = Expr::Literal(Literal::Number(42));

        let result = gen.compile_expr(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_bool_literal() {
        let llvm_ctx = LLVMCompilerContext::new();
        let context = llvm_ctx.get_context();
        let module = llvm_ctx.create_module("test");
        let builder = llvm_ctx.create_builder();

        let mut gen = CodeGenerator::new(&context, module, builder);
        let expr = Expr::Literal(Literal::Bool(true));
        let result = gen.compile_expr(&expr);
        assert!(result.is_ok())
    }
}