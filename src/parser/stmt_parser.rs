/*
 * -------------------------------------------------------------------------
 * RemyLang — stmt_parser implementation
 * File : src/parser/stmt_parser.rs
 *
 * Description :
 *   Statement parser implementation for the RemyLang language.
 *
 * Author  : Samuel 'Meeast' Bleau
 * Created : 2025-11-08
 *
 * -------------------------------------------------------------------------
*/

use super::parser::Parser;
use super::error::{ParseError, ParseResult};
use crate::ast::{Stmt, Param, Type};
use crate::lexer::Token;

impl Parser {
    /// Parses a statement.
    pub(super) fn parse_statement(&mut self) -> ParseResult<Stmt> {
        match self.peek() {
            Some(Token::Int) | Some(Token::String) | Some(Token::Bool) 
            | Some(Token::Char) | Some(Token::Array) => {
                self.parse_let_statement()
            }
            Some(Token::If) => self.parse_if_statement(),
            Some(Token::Return) => self.parse_return_statement(),
            Some(Token::LeftBrace) => self.parse_block_statement(),
            Some(Token::Func) => self.parse_function_declaration(),
            Some(Token::Identifier(_)) => {
                // Could be assignment or expression statement
                // Peek ahead to check for '='
                if matches!(self.peek_next(), Some(Token::Equal)) {
                    self.parse_assignment()
                } else {
                    self.parse_expression_statement()
                }
            }
            _ => self.parse_expression_statement(),
        }
    }

    /// Parse: Int x = 42;
    fn parse_let_statement(&mut self) -> ParseResult<Stmt> {
        let type_annotation = self.parse_type()?;
        
        let name = match self.advance() {
            Token::Identifier(n) => n.clone(),
            token => {
                return Err(ParseError::UnexpectedToken {
                    expected: "variable name".to_string(),
                    found: token.clone(),
                });
            }
        };

        self.expect(&Token::Equal, "'=' after variable name")?;
        let value = self.parse_expression()?;
        self.expect(&Token::Semicolon, "';' after expression")?;

        Ok(Stmt::Let {
            name,
            type_annotation: Some(type_annotation),
            value,
        })
    }
    /// Parse: x = 10;
    fn parse_assignment(&mut self) -> ParseResult<Stmt> {
        let name = match self.advance() {
            Token::Identifier(n) => n.clone(),
            token => {
                return Err(ParseError::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: token.clone(),
                });
            }
        };
        self.expect(&Token::Equal, "'='")?;
        let value = self.parse_expression()?;
        self.expect(&Token::Semicolon, "';' after expression")?;
        Ok(Stmt::Assignment { name, value })
    }

    /// Parse: expression;
    fn parse_expression_statement(&mut self) -> ParseResult<Stmt> {
        let expr = self.parse_expression()?;
        self.expect(&Token::Semicolon, "';' after expression")?;
        Ok(Stmt::Expression(expr))
    }

    /// Parse: if (condition) { ... } else { ... }
    fn parse_if_statement(&mut self) -> ParseResult<Stmt> {
        self.expect(&Token::If, "'if'")?;
        self.expect(&Token::LeftParen, "'(' after 'if'")?;
        let condition = self.parse_expression()?;
        self.expect(&Token::RightParen, "')' after condition")?;

        let then_branch = Box::new(self.parse_block_statement()?);
        
        let else_branch = if self.match_token(&[Token::Else]) {
            Some(Box::new(if self.check(&Token::If) {
                // else if
                self.parse_if_statement()?
            } else {
                // else
                self.parse_block_statement()?
            }))
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }
    /// Parse: return <expr>;
    fn parse_return_statement(&mut self) -> ParseResult<Stmt> {
        self.expect(&Token::Return, "'return'")?;
        let value = if self.check(&Token::Semicolon) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        self.expect(&Token::Semicolon, "';' after return")?;
        Ok(Stmt::Return(value))
    }
    /// Parse: { stmt1; stmt2; ... }
    fn parse_block_statement(&mut self) -> ParseResult<Stmt> {
        self.expect(&Token::LeftBrace, "'{'")?;

        let mut statements = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        self.expect(&Token::RightBrace, "'}' after block")?;
        Ok(Stmt::Block(statements))
    }

    /// Parse: func name(params) -> return_type { body }
    fn parse_function_declaration(&mut self) -> ParseResult<Stmt> {
        self.expect(&Token::Func, "'func'")?;
        
        let name = match self.advance() {
            Token::Identifier(n) => n.clone(),
            token => {
                return Err(ParseError::UnexpectedToken {
                    expected: "function name".to_string(),
                    found: token.clone(),
                })
            }
        };
        self.expect(&Token::LeftParen, "'(' after function name")?;
        let params = self.parse_parameter_list()?;
        self.expect(&Token::RightParen, "')' after parameters")?;
        
        let return_type = if self.match_token(&[Token::Arrow]) {
            Some(self.parse_type()?)
        } else {
            None
        };
        
        let body = self.parse_block_statement()?;
        Ok(Stmt::FunctionDecl {
            name,
            params,
            return_type,
            body: Box::new(body),
        })
    }

    /// Parses a list of function parameters.
    fn parse_parameter_list(&mut self) -> ParseResult<Vec<Param>> {
        let mut params = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                let type_annotation = self.parse_type()?;
                let name = match self.advance() {
                    Token::Identifier(n) => n.clone(),
                    token => {
                        return Err(ParseError::UnexpectedToken {
                            expected: "parameter name".to_string(),
                            found: token.clone(),
                        });
                    }
                };
                
                params.push(Param {
                    name,
                    type_annotation,
                });

                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        Ok(params)
    }

    /// Parses a type annotation.
    fn parse_type(&mut self) -> ParseResult<Type> {
        match self.advance() {
            Token::Int => Ok(Type::Int),
            Token::String => Ok(Type::String),
            Token::Bool => Ok(Type::Bool),
            Token::Char => Ok(Type::Char),
            Token::Array => {
                self.expect(&Token::Less, "'<' after 'Array'")?;
                let element_type = self.parse_type()?;
                self.expect(&Token::Greater, "'>' after array element type")?;
                Ok(Type::Array(Box::new(element_type)))
            }
            token => Err(ParseError::UnexpectedToken {
                expected: "type (Int, String, Bool, Char, Array)".to_string(),
                found: token.clone(),
            }),
        }
    }
}