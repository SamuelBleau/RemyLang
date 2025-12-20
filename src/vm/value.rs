/*
 * -------------------------------------------------------------------------
 * RemyLang — value implementation
 * File : src/vm/value.rs
 *
 * Description :
 *   Runtime value representations for the RemyLang VM.
 *
 * Author  : Samuel 'Meeast' Bleau
 * Created : 2025-12-20
 *
 * -------------------------------------------------------------------------
*/

use std::fmt;
use crate::ast::Stmt;

/// Runtime value representation
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i64),
    String(String),
    Char(char),
    Bool(bool),
    Array(Vec<Value>),
    Function {
        name: String,
        params: Vec<String>,
        body: Box<Stmt>,
    },
    Void, // For functions that don't return anything
}

impl Value {
    /// Type name for error messages
    pub fn type_name(&self) -> &str {
        match self {
            Value::Number(_) => "Int",
            Value::String(_) => "String",
            Value::Char(_) => "Char",
            Value::Bool(_) => "Bool",
            Value::Array(_) => "Array",
            Value::Function { .. } => "Function",
            Value::Void => "Void",
        }
    }

    /// Check if the value is truthy (for conditionals)
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0,
            Value::String(s) => !s.is_empty(),
            Value::Array(arr) => !arr.is_empty(),
            Value::Void => false,
            _ => true,
        }
    }

    /// Convert to bool for logical operations
    pub fn as_bool(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            _ => self.is_truthy(),
        }
    }

    /// Try to convert to number
    pub fn as_number(&self) -> Option<i64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Char(c) => write!(f, "{}", c),
            Value::Bool(b) => write!(f, "{}", if *b { "True" } else { "False" }),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, val) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", val)?;
                }
                write!(f, "]")
            }
            Value::Function { name, .. } => write!(f, "<function {}>", name),
            Value::Void => write!(f, "void"),
        }
    }
}
