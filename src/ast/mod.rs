/*
 * -------------------------------------------------------------------------
 * RemyLang — mod implementation
 * File : src/ast/mod.rs
 *
 * Description :
 *      Organization module for the abstract syntax tree (AST) component.
 *
 * Author  : Samuel 'Meeast' Bleau
 * Created : 2025-11-05
 *
 * -------------------------------------------------------------------------
*/

mod expr;
mod stmt;
pub mod operator;

// Re-exports
pub use expr::{Expr, Literal};
pub use stmt::{Stmt, Param};
pub use operator::{BinaryOp, UnaryOp};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    String,
    Char,
    Bool,
    Array(Box<Type>),
}
