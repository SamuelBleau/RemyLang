use crate::ast::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TypeError {
    UndefinedVariable(String),
    TypeMismatch { expected: Type, found: Type },
    InvalidOperand(String),
    ReturnTypeMismatch,
}

pub struct TypeChecker {
    symbol_table: SymbolTable,
    current_function_return_type: Option<Type>,
    errors: Vec<TypeError>,
}

impl TypeChecker {
    pub fn new() -> Self {
    }

    pub fn check_program(&mut self, stmts: &[Stmt]) -> Result<(), Vec<TypeError>> {
    }

    fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), TypeError> {
    }

    fn infer_expr(&mut self, expr: &Expr) -> Result<Type, TypeError> {
    }
}
