/*
 * -------------------------------------------------------------------------
 * RemyLang — expr_parser implementation
 * File : src/parser/expr_parser.rs
 *
 * Description :
 *   Expression parser implementation for the RemyLang language.
 *   Uses Pratt parsing for handling operator precedence and associativity.
 *
 * Author  : Samuel 'Meeast' Bleau
 * Created : 2025-11-08
 *
 * -------------------------------------------------------------------------
*/

use super::parser::Parser;
use super::error::{ParseError, ParseResult};
use crate::ast::{Expr, Literal};
use crate::ast::operator::{BinaryOp, UnaryOp};
use crate::lexer::Token;

impl Parser {
    pub(super) fn parse_expression(&mut self) -> ParseResult<Expr> {
        self.parse_expression_with_precedence(0)
    }

    /// Parses an expression considering operator precedence using Pratt parsing.
    fn parse_expression_with_precedence(&mut self, min_precedence: u8) -> ParseResult<Expr> {
        let mut left = self.parse_prefix()?;

        while let Some(op) = self.current_binary_op() {
            let (precedence, is_left_assoc) = op.precedence_and_associativity();
            if precedence < min_precedence {
                break;
            }
            self.advance();
            
            // For left-associative operators, we increase precedence for the right side
            // For right-associative (like **), we keep the same precedence
            let next_min_precedence = if is_left_assoc {
                precedence + 1
            } else {
                precedence
            };
            
            let right = self.parse_expression_with_precedence(next_min_precedence)?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    /// Parses prefix expressions: unary operators, literals, identifiers, parenthesized expressions
    fn parse_prefix(&mut self) -> ParseResult<Expr> {
        match self.peek() {
            Some(Token::Minus) => {
                self.advance();
                let right = self.parse_prefix()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Minus,
                    right: Box::new(right),
                })
            }
            Some(Token::Bang) => {
                self.advance();
                let right = self.parse_prefix()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Not,
                    right: Box::new(right),
                })
            }
            _ => self.parse_primary(),
        }
    }

    /// Parses primary expressions: literals, identifiers, parenthesized expressions, arrays
    fn parse_primary(&mut self) -> ParseResult<Expr> {
        let expr = match self.peek().cloned() {
            Some(Token::Number(n)) => {
                self.advance();
                Expr::Literal(Literal::Number(n))
            }
            Some(Token::StringLit(s)) => {
                self.advance();
                Expr::Literal(Literal::String(s))
            }
            Some(Token::CharLit(c)) => {
                self.advance();
                Expr::Literal(Literal::Char(c))
            }
            Some(Token::True) => {
                self.advance();
                Expr::Literal(Literal::Bool(true))
            }
            Some(Token::False) => {
                self.advance();
                Expr::Literal(Literal::Bool(false))
            }
            Some(Token::Identifier(name)) => {
                self.advance();
                Expr::Variable(name)
            }
            Some(Token::LeftBracket) => {
                return self.parse_array_literal();
            }
            Some(Token::LeftParen) => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(&Token::RightParen, "')' after expression")?;
                expr
            }
            _ => {
                return Err(ParseError::ExpectedExpression {
                    found: self.peek().cloned().unwrap_or(Token::EOF),
                });
            }
        };
        self.parse_suffix(expr)
    }

    /// Parses suffixes for expressions (like function calls, array indexing, etc.)
    fn parse_suffix(&mut self, mut expr: Expr) -> ParseResult<Expr> {
        loop {
            match self.peek() {
                Some(Token::LeftParen) => {
                    self.advance();
                    let args = self.parse_argument_list()?;
                    self.expect(&Token::RightParen, "')' after arguments")?;

                    expr = Expr::Call {
                        callee: Box::new(expr),
                        args,
                    };
                }
                Some(Token::LeftBracket) => {
                    self.advance();
                    let index = self.parse_expression()?;
                    self.expect(&Token::RightBracket, "']' after array index")?;

                    expr = Expr::ArrayAccess {
                        array: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    /// Parses an array literal : [elem1, elem2, ...]
    fn parse_array_literal(&mut self) -> ParseResult<Expr> {
        self.expect(&Token::LeftBracket, "'['")?;

        let mut elements = Vec::new();
        if !self.check(&Token::RightBracket) {
            loop {
                elements.push(self.parse_expression()?);
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.expect(&Token::RightBracket, "']' after array elements")?;
        Ok(Expr::ArrayLiteral(elements))
    }

    /// Parses a list of arguments : (arg1, arg2, ...)
    fn parse_argument_list(&mut self) -> ParseResult<Vec<Expr>> {
        let mut args = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                args.push(self.parse_expression()?);
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        Ok(args)
    }

    /// Returns the current binary operator if the current token is one.
    fn current_binary_op(&self) -> Option<BinaryOp> {
        if self.is_at_end() {
            return None;
        }
        match self.peek() {
            Some(Token::Plus) => Some(BinaryOp::Add),
            Some(Token::Minus) => Some(BinaryOp::Sub),
            Some(Token::Star) => Some(BinaryOp::Mul),
            Some(Token::Power) => Some(BinaryOp::Pow),
            Some(Token::Slash) => Some(BinaryOp::Div),
            Some(Token::Percent) => Some(BinaryOp::Mod),
            Some(Token::EqualEqual) => Some(BinaryOp::Equal),
            Some(Token::BangEqual) => Some(BinaryOp::NotEqual),
            Some(Token::Less) => Some(BinaryOp::Less),
            Some(Token::Greater) => Some(BinaryOp::Greater),
            Some(Token::LessEqual) => Some(BinaryOp::LessEqual),
            Some(Token::GreaterEqual) => Some(BinaryOp::GreaterEqual),
            Some(Token::And) => Some(BinaryOp::And),
            Some(Token::Or) => Some(BinaryOp::Or),
            _ => None,
        }
    }
}
