/*
 * -------------------------------------------------------------------------
 * RemyLang — VM error handling
 * File : src/vm/error.rs
 *
 * Description :
 *   Runtime error types for the RemyLang virtual machine.
 *
 * Author  : Samuel 'Meeast' Bleau
 * Created : 2025-11-09
 *
 * -------------------------------------------------------------------------
*/

use std::fmt;

pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    /// Variable not found in current scope
    UndefinedVariable {
        name: String,
    },

    /// Function not found
    UndefinedFunction {
        name: String,
    },

    /// Type mismatch error
    TypeMismatch {
        operation: String,
        expected: String,
        found: String,
    },

    /// Division and Modulo by zero error
    DivisionByZero,
    ModuloByZero,

    /// Index out of bounds error for arrays
    IndexOutOfBounds {
        index: i64,
        length: usize,
    },

    /// Trying to index a non-array value
    NotIndexable {
        value_type: String,
    },

    /// Trying to call a non-function value
    NotCallable {
        value_type: String,
    },

    /// Wrong number of arguments in function call
    ArgumentCountMismatch {
        expected: usize,
        found: usize,
        function_name: String,
    },

    /// Stack overflow
    StackOverflow {
        max_depth: usize,
    },

    /// Return statement outside of function
    ReturnOutsideFunction,

    /// Invalid operation
    InvalidOperation {
        operation: String,
        left_type: String,
        right_type: String,
    },

    /// Assignment to undefined variable
    AssignmentToUndefined {
        name: String,
    },

    /// Custom error message
    Custom(String),

}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::UndefinedVariable { name } => {
                write!(f, "Runtime error: Undefined variable '{}'", name)
            }
            RuntimeError::UndefinedFunction { name } => {
                write!(f, "Runtime error: Undefined function '{}'", name)
            }
            RuntimeError::TypeMismatch { operation, expected, found } => {
                write!(
                    f,
                    "Runtime error: Type error in {}: expected {}, found {}",
                    operation, expected, found
                )
            }
            RuntimeError::DivisionByZero => {
                write!(f, "Runtime error: Division by zero")
            }
            RuntimeError::ModuloByZero => {
                write!(f, "Runtime error: Modulo by zero")
            }
            RuntimeError::IndexOutOfBounds { index, length } => {
                write!(
                    f,
                    "Runtime error: Index {} out of bounds for array of length {}",
                    index, length
                )
            }
            RuntimeError::NotIndexable { value_type } => {
                write!(
                    f,
                    "Runtime error: Cannot index into value of type {}",
                    value_type
                )
            }
            RuntimeError::NotCallable { value_type } => {
                write!(
                    f,
                    "Runtime error: Cannot call value of type {}",
                    value_type
                )
            }
            RuntimeError::ArgumentCountMismatch { expected, found, function_name } => {
                write!(
                    f,
                    "Runtime error: Function '{}' expects {} arguments, but {} were provided",
                    function_name, expected, found
                )
            }
            RuntimeError::StackOverflow { max_depth } => {
                write!(
                    f,
                    "Runtime error: Stack overflow (maximum call depth of {} exceeded)",
                    max_depth
                )
            }
            RuntimeError::ReturnOutsideFunction => {
                write!(f, "Runtime error: Return statement outside of function")
            }
            RuntimeError::InvalidOperation { operation, left_type, right_type } => {
                write!(
                    f,
                    "Runtime error: Invalid operation '{}' between {} and {}",
                    operation, left_type, right_type
                )
            }
            RuntimeError::AssignmentToUndefined { name } => {
                write!(
                    f,
                    "Runtime error: Assignment to undefined variable '{}'. Use 'Type {} = ...' to declare it first",
                    name, name
                )
            }
            RuntimeError::Custom(msg) => {
                write!(f, "Runtime error: {}", msg)
            }
        }
    }
}

impl std::error::Error for RuntimeError {}

/// Special control flow signal for return statements
/// This is not an error but a way to unwind the stack
#[derive(Debug, Clone, PartialEq)]
pub struct ReturnSignal(pub crate::vm::value::Value);

