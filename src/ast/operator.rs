/*
 * -------------------------------------------------------------------------
 * RemyLang — operator implementation
 * File : src/ast/operator.rs
 *
 * Description :
 *   Abstract syntax tree (AST) representation for operators in the RemyLang language.
 *
 * Author  : Samuel 'Meeast' Bleau
 * Created : 2025-11-05
 *
 * -------------------------------------------------------------------------
*/

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, Mod, Pow,

    Equal, NotEqual, Less, Greater, LessEqual, GreaterEqual,

    And, Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Minus,
    Not,
}

impl BinaryOp {
    pub fn precedence(&self) -> u8 {
        match self {
            BinaryOp::Or => 1,
            BinaryOp::And => 2,
            BinaryOp::Equal | BinaryOp::NotEqual => 3,
            BinaryOp::Less | BinaryOp::Greater | BinaryOp::LessEqual | BinaryOp::GreaterEqual => 4,
            BinaryOp::Add | BinaryOp::Sub => 5,
            BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => 6,
            BinaryOp::Pow => 7,
        }
    }
}