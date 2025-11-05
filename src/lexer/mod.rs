/*
 * -------------------------------------------------------------------------
 * RemyLang — module lexer
 * File : src/lexer/mod.rs
 *
 * Description :
 *   Organization module for the lexer component.
 *
 * Author  : Samuel 'Meeast' Bleau
 * Created : 2025-11-04
 *
 * -------------------------------------------------------------------------
*/


mod token;
mod lexer;

// Re-export for external use
pub use lexer::Lexer;

#[allow(unused_imports)]
pub use token::Token;