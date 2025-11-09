/*
 * -------------------------------------------------------------------------
 * RemyLang — expr implementation
 * File : src/ast/expr.rs
 *
 * Description :
 *   Abstract syntax tree (AST) representation for the RemyLang language.
 *
 * Author  : Samuel 'Meeast' Bleau
 * Created : 2025-11-05
 *
 * -------------------------------------------------------------------------
*/

use super::operator::{BinaryOp, UnaryOp};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Variable(String),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    ArrayAccess {
        array: Box<Expr>,
        index: Box<Expr>,
    },
    ArrayLiteral(Vec<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(i64),
    String(String),
    Char(char),
    Bool(bool),
}