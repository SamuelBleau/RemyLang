use crate::ast::*;
use crate::semantic::symbol_table::{Symbol, SymbolTable};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TypeError {
    UndefinedVariable(String),
    TypeMismatch { expected: Type, found: Type },
    InvalidOperand(String),
    ReturnTypeMismatch,
    CannotAssignToFunction(String),
    ArgumentCountMismatch { expected: usize, found: usize },
    ArgumentTypeMismatch { position: usize, expected: Type, found: Type },
    NotCallable,
    InvalidCallTarget,
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

            Stmt::Return(expr_opt) => {
                let return_type = match expr_opt {
                    Some(expr) => self.infer_expr(expr)?,
                    None => Type::Void,
                };

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

            Stmt::Block(stmts) => {
                self.symbol_table.enter_scope();
                for stmt in stmts {
                    self.check_stmt(stmt)?;
                }
                self.symbol_table.exit_scope();
                Ok(())
            }

            Stmt::Expression(expr) => {
                self.infer_expr(expr)?;
                Ok(())
            }

            Stmt::Assignment { name, value } => {
                let symbol = self.symbol_table.get(name)
                    .ok_or_else(|| TypeError::UndefinedVariable(name.clone()))?;

                let var_type = match symbol {
                    Symbol::Variable(ty) => ty.clone(),
                    Symbol::Function { .. } => {
                        return Err(TypeError::CannotAssignToFunction(name.clone()));
                    }
                };

                let value_type = self.infer_expr(value)?;

                if var_type != value_type {
                    return Err(TypeError::TypeMismatch {
                        expected: var_type,
                        found: value_type,
                    });
                }

                Ok(())
            }

            Stmt::If { condition, then_branch, else_branch } => {
                let cond_type = self.infer_expr(condition)?;
                if cond_type != Type::Bool {
                    return Err(TypeError::TypeMismatch {
                        expected: Type::Bool,
                        found: cond_type,
                    });
                }

                self.check_stmt(then_branch)?;

                if let Some(else_stmt) = else_branch {
                    self.check_stmt(else_stmt)?;
                }

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
            Expr::Literal(Literal::Char(_)) => Ok(Type::Char),
            Expr::Call { callee, args } => {
                let func_symbol = if let Expr::Variable(name) = callee.as_ref() {
                self.symbol_table.get(name)
                    .ok_or_else(|| TypeError::UndefinedVariable(name.clone()))?
                } else {
                    return Err(TypeError::InvalidCallTarget);
                };
                match func_symbol {
                    Symbol::Function { params, return_type } => {
                        if params.len() != args.len() {
                            return Err(TypeError::ArgumentCountMismatch {
                                expected: params.len(),
                                found: args.len(),
                            });
                        }

                        for (i, (arg, expected_type)) in args.iter().zip(params.iter()).enumerate() {
                            let arg_type = self.infer_expr(arg)?;
                            if &arg_type != expected_type {
                                return Err(TypeError::ArgumentTypeMismatch {
                                    index: i,
                                    expected: expected_type.clone(),
                                    found: arg_type,
                                });
                            }
                        }
                        Ok(return_type.clone())
                    }
                    Symbol::Variable(_) => Err(TypeError::NotCallable)
                }
            }
            // _ => Ok(Type::Int), TODO: Take this off
        }
    }
}
