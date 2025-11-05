/*
 * -------------------------------------------------------------------------
 * RemyLang — ast implementation
 * File : src/ast/stmt.rs
 *
 * Description :
 *   Abstract syntax tree (AST) statement representation for the RemyLang language.
 *
 * Author  : Samuel 'Meeast' Bleau
 * Created : 2025-11-05
 *
 * -------------------------------------------------------------------------
*/

use super::expr::Expr;
use super::Type;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expression(Expr),
    Let {
        name: String,
        type_annotation: Option<Type>,
        value: Expr,
    },
    Assignment {
        name: String,
        value: Expr,
    },
    Block(Vec<Stmt>),
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Return(Option<Expr>),
    FunctionDecl {
        name: String,
        params: Vec<Param>,
        return_type: Option<Type>,
        body: Box<Stmt>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub type_annotation: Type,
}