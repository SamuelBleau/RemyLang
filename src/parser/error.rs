/*
 * -------------------------------------------------------------------------
 * RemyLang — parser error handling
 * File : src/parser/error.rs
 *
 * Description :
 *   Error handling for the RemyLang parser.
 *
 * Author  : Samuel 'Meeast' Bleau
 * Created : 2025-11-05
 *
 * -------------------------------------------------------------------------
*/

use std::fmt;
use crate::lexer::Token;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedToken { expected: String, found: Token },
    UnexpectedEOF { expected: String },
    InvalidSyntax { message: String, token: Option<Token> },
    ExpectedExpression { found: Token },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken { expected, found } => {
                write!(f, "Parse error: Expected {}, found {:?}", expected, found)
            }
            ParseError::UnexpectedEOF { expected } => {
                write!(f, "Parse error: Unexpected end of file, expected {}", expected)
            }
            ParseError::InvalidSyntax { message, token } => {
                if let Some(tok) = token {
                    write!(f, "Parse error: {} at {:?}", message, tok)
                } else {
                    write!(f, "Parse error: {}", message)
                }
            }
            ParseError::ExpectedExpression { found } => {
                write!(f, "Parse error: Expected expression, found {:?}", found)
            }
        }
    }
}

impl std::error::Error for ParseError {}