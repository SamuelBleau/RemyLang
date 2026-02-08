use crate::ast::*;
use crate::semantic::symbol_table::{Symbol, SymbolTable};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TypeError {
    UndefinedVariable(String),
    TypeMismatch { expected: Type, found: Type },
    InvalidOperand(String),
    ReturnTypeMismatch,
}

pub struct TypeChecker {
    symbol_table: SymbolTable,
    current_function_return_type: Option<Type>,
    errors: Vec<TypeError>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            current_function_return_type: None,
            errors: Vec::new(),
        }
    }

    pub fn check_program(&mut self, stmts: &[Stmt]) -> Result<(), Vec<TypeError>> {
        for stmt in stmts {
            if let Err(e) = self.check_stmt(stmt) {
                self.errors.push(e);
            }
        }

        if self.errors.is_empty() {
            Ok(());
        } else {
            Err(self.errors.clone())
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), TypeError> {
        match stmt {
            Stmt::Let { name, type_annotation, value } => {
                let value_type = self.infer_expr(value)?;

                if let Some(expected_type) = type_annotation {
                    if expected_type != &value_type {
                        return Err(TypeError::TypeMismatch { expected: expected_type.clone(), found: value_type.clone() });
                    }
                }
                self.symbol_table.define(name.clone(), Symbol::Variable(value_type));
                Ok(())
            }

            Stmt::Return(expr) => {
                let return_type = self.infer_expr(expr)?;

                if let Some(expected) = &self.current_function_return_type {
                    if expected != &return_type {
                        return Err(TypeError::ReturnTypeMismatch);
                    }
                }
                Ok(())
            }

            Stmt::FunctionDecl { name, params, return_type, body } => {
                // Register the function in the current scope first
                let param_types: Vec<Type> = params.iter().map(|p| p.type_annotation.clone()).collect();
                self.symbol_table.define(
                    name.clone(),
                    Symbol::Function {
                        params: param_types,
                        return_type: return_type.clone().unwrap_or(Type::Void),
                    }
                );

                self.symbol_table.enter_scope();
                self.current_function_return_type = return_type.clone();

                for param in params {
                    self.symbol_table.define(
                        param.name.clone(),
                        Symbol::Variable(param.type_annotation.clone()),
                    )
                }

                self.check_stmt(body)?;
                self.symbol_table.exit_scope();
                self.current_function_return_type = None;
                Ok(())
            }

            _ => Ok(()),
        }
    }

    fn infer_expr(&mut self, expr: &Expr) -> Result<Type, TypeError> {
        match expr {
            Expr::Literal(Literal::Number(_)) => Ok(Type::Int),
            Expr::Literal(Literal::Bool(_)) => Ok(Type::Bool),
            Expr::Literal(Literal::String(_)) => Ok(Type::String),
            Expr::Variable(name) => {
                let symbol = self.symbol_table.get(name)
                    .ok_or_else(|| TypeError::UndefinedVariable(name.clone()))?;

                match symbol {
                    Symbol::Variable(ty) => Ok(ty.clone()),
                    Symbol::Function { .. } => {
                        // Functions used as values might need special handling
                        Err(TypeError::InvalidOperand(format!("Cannot use function '{}' as a value", name)))
                    }
                }
            }
            Expr::Binary { left, op, right } => {
                let left_type = self.infer_expr(left)?;
                let right_type = self.infer_expr(right)?;

                if left_type != right_type {
                    return Err(TypeError::TypeMismatch {
                        expected: left_type,
                        found: right_type
                    });
                }
                Ok(left_type)
            }
            _ => Ok(Type::Int), //TODO: Handle other expression types and operators
        }
    }
}
