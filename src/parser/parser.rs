/*
 * -------------------------------------------------------------------------
 * RemyLang — parser implementation
 * File : src/parser/parser.rs
 *
 * Description :
 *   Parser implementation for the RemyLang language.
 *
 * Author  : Samuel 'Meeast' Bleau
 * Created : 2025-11-08
 *
 * -------------------------------------------------------------------------
*/

use crate::lexer::Token;
use crate::ast::{Expr, Stmt};
use super::error::{ParseError, ParseResult};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    /// Creates a new Parser instance with the given tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    /// Looks at the current token without consuming it.
    pub(super) fn peek(&self) -> Option<&Token> {
        &self.tokens[self.current]
    }

    /// Looks at the next token without consuming it.
    pub(super) fn peek_next(&self) -> Option<&Token> {
        if self.current + 1 < self.tokens.len() {
            Some(&self.tokens[self.current + 1])
        } else {
            None
        }
    }

    /// Advances to the next token and returns the current one.
    pub(super) fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }

    /// Checks if the actual token is of the given type.
    pub(super) fn check(&self, token_type: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(self.peek()) == std::mem::discriminant(token_type)
    }

    /// Checks if we have reached the end of the token stream.
    pub(super) fn is_at_end(&self) -> bool {
        matches!(self.peek(), Some(Token::EOF))
    }

    /// Advances if the current token matches the expected type.
    pub(super) fn expect(&mut self, expected: &Token, message: &str) -> ParseResult<&Token> {
        if self.check(expected) {
            Ok(self.advance().clone())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: message.to_string(),
                found: self.peek().clone(),
            })
        }
    }

    /// Verify and consume a given token
    pub(super) fn match_token(&mut self, tokens: &[Token]) -> bool {
        for token_type in tokens {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// Entry point for parsing
    pub fn parse(&mut self) -> ParseResult<Vec<Stmt>> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }
}