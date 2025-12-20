/*
 * -------------------------------------------------------------------------
 * RemyLang — VM module
 * File : src/vm/mod.rs
 *
 * Description :
 *   Virtual machine module organization.
 *
 * Author  : Samuel 'Meeast' Bleau
 * Created : 2025-12-20
 *
 * -------------------------------------------------------------------------
*/

pub mod value;
pub mod error;
pub mod environment;
pub mod interpreter;
pub mod builtin;

// Re-exports for convenience
pub use value::Value;
pub use error::{RuntimeError, RuntimeResult};
pub use environment::Environment;
pub use interpreter::Interpreter;
