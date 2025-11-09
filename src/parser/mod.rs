/*
 * -------------------------------------------------------------------------
 * RemyLang — parser module
 * File : src/parser/mod.rs
 *
 * Description :
 *   Module organization for the parser component.
 *
 * Author  : Samuel 'Meeast' Bleau
 * Created : 2025-11-08
 *
 * -------------------------------------------------------------------------
*/

mod parser;
mod expr_parser;
mod stmt_parser;
pub mod error;

// Re-exports
pub use parser::Parser;
pub use error::{ParseError, ParseResult};
