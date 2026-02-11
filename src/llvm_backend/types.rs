/*
 * -------------------------------------------------------------------------
 * RemyLang - Type Conversion (RemyLang -> LLVM)
 * File : src/llvm_backend/types.rs
 *
 * Description :
 *   Convert RemyLang's types to LLVM's types
 *
 * Author  : Samuel 'Meeast' Bleau
 * Created : 2026-02-10
 *
 * -------------------------------------------------------------------------
*/
use crate::ast::Type;
use inkwell::context::Context;
use inkwell::types::{BasicType, BasicTypeEnum, FunctionType, BasicMetadataTypeEnum};

/// Convert a RemyLang type to an LLVM type
pub struct TypeConverter;

impl TypeConverter {
    /// Convert a RemyLang type to an LLVM type
    pub fn type_to_llvm_type<'ctx>(remy_type: &Type, context: &'ctx Context) -> BasicTypeEnum<'ctx> {
        match remy_type {
            Type::Int => context.i64_type().into(),
            Type::Bool => context.bool_type().into(),
            Type::Char => context.i8_type().into(),
            Type::String => context.ptr_type(inkwell::AddressSpace::default()).into(),
            Type::Array(_elem_type) => {
                context.ptr_type(inkwell::AddressSpace::default()).into()
            }
            Type::Void => panic!("Void type cannot be converted to an LLVM type"),
        }
    }

    /// Create an LLVM function type
    pub fn create_function_type<'ctx>(
        param_types: &[BasicTypeEnum<'ctx>],
        return_type: &Type,
        context: &'ctx Context,
    ) -> FunctionType<'ctx> {
        let metadata_params: Vec<BasicMetadataTypeEnum> = param_types
            .iter()
            .map(|t| (*t).into())
            .collect();

        match return_type {
            Type::Void => context.void_type().fn_type(&metadata_params, false),
            _ => Self::type_to_llvm_type(return_type, context).fn_type(&metadata_params, false),
        }
    }

    /// Return a text representation of a type for debugging purposes
    pub fn llvm_type_to_string(remy_type: &Type) -> &'static str {
        match remy_type {
            Type::Int => "i64",
            Type::Bool => "i1",
            Type::Char => "i8",
            Type::String => "i8*",
            Type::Array(_) => "[...] (array pointer)",
            Type::Void => "void",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;

    #[test]
    fn test_int_type_conversion() {
        let ctx = Context::create();
        let llvm_type = TypeConverter::type_to_llvm_type(&Type::Int, &ctx);
        assert!(llvm_type.is_int_type());
    }

    #[test]
    fn test_bool_type_conversion() {
        let ctx = Context::create();
        let llvm_type = TypeConverter::type_to_llvm_type(&Type::Bool, &ctx);
        assert!(llvm_type.is_int_type());
    }

    #[test]
    fn test_string_type_conversion() {
        let ctx = Context::create();
        let llvm_type = TypeConverter::type_to_llvm_type(&Type::String, &ctx);
        assert!(llvm_type.is_pointer_type());
    }
}