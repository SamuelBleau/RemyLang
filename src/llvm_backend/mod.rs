/*
 * -------------------------------------------------------------------------
 * RemyLang - LLVM Backend Module
 * File : src/llvm_backend/mod.rs
 *
 * Description :
 *   Principal LLVM backend module
 *   Export the components of the LLVM backend to the rest of the compiler
 *
 * Author  : Samuel 'Meeast' Bleau
 * Created : 2026-02-010
 *
 * -------------------------------------------------------------------------
*/

pub mod context;
pub mod types;
pub mod codegen;
pub mod module;

pub use context::LLVMCompilerContext;
// pub use codegen::CodeGenerator;
// pub use types::TypeConverter;