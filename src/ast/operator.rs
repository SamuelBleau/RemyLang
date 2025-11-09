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
    /// Returns the precedence and associativity of the operator.
    /// Higher precedence = tighter binding.
    /// Associativity: true = left-to-right, false = right-to-left
    pub fn precedence_and_associativity(&self) -> (u8, bool) {
        match self {
            BinaryOp::Or => (1, true),
            BinaryOp::And => (2, true),
            BinaryOp::Equal | BinaryOp::NotEqual => (3, true),
            BinaryOp::Less | BinaryOp::Greater | BinaryOp::LessEqual | BinaryOp::GreaterEqual => (4, true),
            BinaryOp::Add | BinaryOp::Sub => (5, true),
            BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => (6, true),
            BinaryOp::Pow => (7, false), // Right-associative: 2**3**2 = 2**(3**2)
        }
    }

    /// Returns just the precedence (for convenience)
    pub fn precedence(&self) -> u8 {
        self.precedence_and_associativity().0
    }
}