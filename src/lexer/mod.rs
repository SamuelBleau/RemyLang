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

pub use token::Token;
pub use lexer::Lexer;