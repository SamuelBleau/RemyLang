/*
 * -------------------------------------------------------------------------
 * RemyLang — interpreter implementation
 * File : src/vm/interpreter.rs
 *
 * Description :
 *   Tree-walking interpreter for the RemyLang VM.
 *
 * Author  : Samuel 'Meeast' Bleau
 * Created : 2025-12-20
 *
 * -------------------------------------------------------------------------
*/

use crate::ast::*;
use crate::vm::value::Value;
use crate::vm::environment::Environment;
use crate::vm::error::{RuntimeError, RuntimeResult};
use crate::vm::builtin;

/// Main interpreter struct
pub struct Interpreter {
    env: Environment,
    in_function: bool, // Track if we're inside a function for return statements
}

/// Control flow result type (for handling returns)
type ControlFlowResult = Result<Value, ControlFlow>;

#[derive(Debug, Clone)]
enum ControlFlow {
    Return(Value),
    Error(RuntimeError),
}

impl From<RuntimeError> for ControlFlow {
    fn from(err: RuntimeError) -> Self {
        ControlFlow::Error(err)
    }
}

impl Interpreter {
    /// Create a new interpreter
    pub fn new() -> Self {
        Interpreter {
            env: Environment::new(),
            in_function: false,
        }
    }

    /// Execute a program (list of statements)
    pub fn execute(&mut self, statements: Vec<Stmt>) -> RuntimeResult<()> {
        for stmt in statements {
            match self.execute_stmt_cf(stmt) {
                Ok(_) => {}
                Err(ControlFlow::Error(e)) => return Err(e),
                Err(ControlFlow::Return(_)) => {
                    return Err(RuntimeError::ReturnOutsideFunction);
                }
            }
        }
        Ok(())
    }

    /// Execute a single statement (with control flow)
    fn execute_stmt_cf(&mut self, stmt: Stmt) -> ControlFlowResult {
        match stmt {
            Stmt::Expression(expr) => self.eval_expr(expr).map_err(ControlFlow::from),
            
            Stmt::Let { name, type_annotation: _, value } => {
                let val = self.eval_expr(value).map_err(ControlFlow::from)?;
                self.env.define(name, val);
                Ok(Value::Void)
            }
            
            Stmt::Assignment { name, value } => {
                let val = self.eval_expr(value).map_err(ControlFlow::from)?;
                self.env.set(&name, val).map_err(ControlFlow::from)?;
                Ok(Value::Void)
            }
            
            Stmt::Block(statements) => {
                self.env.push_scope();
                let mut result = Ok(Value::Void);
                
                for stmt in statements {
                    match self.execute_stmt_cf(stmt) {
                        Ok(val) => result = Ok(val),
                        Err(e) => {
                            result = Err(e);
                            break;
                        }
                    }
                }
                
                self.env.pop_scope();
                result
            }
            
            Stmt::If { condition, then_branch, else_branch } => {
                let cond_val = self.eval_expr(condition).map_err(ControlFlow::from)?;
                
                if cond_val.is_truthy() {
                    self.execute_stmt_cf(*then_branch)
                } else if let Some(else_stmt) = else_branch {
                    self.execute_stmt_cf(*else_stmt)
                } else {
                    Ok(Value::Void)
                }
            }
            
            Stmt::Return(expr) => {
                if !self.in_function {
                    return Err(ControlFlow::Error(RuntimeError::ReturnOutsideFunction));
                }
                let value = if let Some(e) = expr {
                    self.eval_expr(e).map_err(ControlFlow::from)?
                } else {
                    Value::Void
                };
                Err(ControlFlow::Return(value))
            }
            
            Stmt::FunctionDecl { name, params, return_type: _, body } => {
                let param_names: Vec<String> = params.into_iter()
                    .map(|p| p.name)
                    .collect();
                
                let func_value = Value::Function {
                    name: name.clone(),
                    params: param_names,
                    body,
                };
                
                self.env.define(name, func_value);
                Ok(Value::Void)
            }
        }
    }

    /// Evaluate an expression
    fn eval_expr(&mut self, expr: Expr) -> RuntimeResult<Value> {
        match expr {
            Expr::Literal(lit) => Ok(self.eval_literal(lit)),
            
            Expr::Variable(name) => self.env.get(&name),
            
            Expr::Binary { left, op, right } => {
                let left_val = self.eval_expr(*left)?;
                let right_val = self.eval_expr(*right)?;
                self.eval_binary_op(left_val, op, right_val)
            }
            
            Expr::Unary { op, right } => {
                let right_val = self.eval_expr(*right)?;
                self.eval_unary_op(op, right_val)
            }
            
            Expr::Call { callee, args } => {
                self.eval_call(*callee, args)
            }
            
            Expr::ArrayLiteral(elements) => {
                let mut values = Vec::new();
                for elem in elements {
                    values.push(self.eval_expr(elem)?);
                }
                Ok(Value::Array(values))
            }
            
            Expr::ArrayAccess { array, index } => {
                let array_val = self.eval_expr(*array)?;
                let index_val = self.eval_expr(*index)?;
                
                match array_val {
                    Value::Array(arr) => {
                        match index_val.as_number() {
                            Some(idx) => {
                                if idx < 0 || idx >= arr.len() as i64 {
                                    return Err(RuntimeError::IndexOutOfBounds {
                                        index: idx,
                                        length: arr.len(),
                                    });
                                }
                                Ok(arr[idx as usize].clone())
                            }
                            None => Err(RuntimeError::TypeMismatch {
                                operation: "array indexing".to_string(),
                                expected: "Int".to_string(),
                                found: index_val.type_name().to_string(),
                            }),
                        }
                    }
                    _ => Err(RuntimeError::NotIndexable {
                        value_type: array_val.type_name().to_string(),
                    }),
                }
            }
        }
    }

    /// Evaluate a literal
    fn eval_literal(&self, lit: Literal) -> Value {
        match lit {
            Literal::Number(n) => Value::Number(n),
            Literal::String(s) => Value::String(s),
            Literal::Char(c) => Value::Char(c),
            Literal::Bool(b) => Value::Bool(b),
        }
    }

    /// Evaluate a binary operation
    fn eval_binary_op(&self, left: Value, op: BinaryOp, right: Value) -> RuntimeResult<Value> {
        match op {
            // Arithmetic operations
            BinaryOp::Add => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                (Value::String(l), Value::String(r)) => Ok(Value::String(l + &r)),
                (l, r) => Err(RuntimeError::InvalidOperation {
                    operation: "addition".to_string(),
                    left_type: l.type_name().to_string(),
                    right_type: r.type_name().to_string(),
                }),
            },
            
            BinaryOp::Sub => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
                (l, r) => Err(RuntimeError::InvalidOperation {
                    operation: "subtraction".to_string(),
                    left_type: l.type_name().to_string(),
                    right_type: r.type_name().to_string(),
                }),
            },
            
            BinaryOp::Mul => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
                (l, r) => Err(RuntimeError::InvalidOperation {
                    operation: "multiplication".to_string(),
                    left_type: l.type_name().to_string(),
                    right_type: r.type_name().to_string(),
                }),
            },
            
            BinaryOp::Div => match (left, right) {
                (Value::Number(l), Value::Number(r)) => {
                    if r == 0 {
                        Err(RuntimeError::DivisionByZero)
                    } else {
                        Ok(Value::Number(l / r))
                    }
                }
                (l, r) => Err(RuntimeError::InvalidOperation {
                    operation: "division".to_string(),
                    left_type: l.type_name().to_string(),
                    right_type: r.type_name().to_string(),
                }),
            },
            
            BinaryOp::Mod => match (left, right) {
                (Value::Number(l), Value::Number(r)) => {
                    if r == 0 {
                        Err(RuntimeError::ModuloByZero)
                    } else {
                        Ok(Value::Number(l % r))
                    }
                }
                (l, r) => Err(RuntimeError::InvalidOperation {
                    operation: "modulo".to_string(),
                    left_type: l.type_name().to_string(),
                    right_type: r.type_name().to_string(),
                }),
            },
            
            BinaryOp::Pow => match (left, right) {
                (Value::Number(l), Value::Number(r)) => {
                    if r < 0 {
                        Err(RuntimeError::Custom("Negative exponents not supported for integers".to_string()))
                    } else {
                        Ok(Value::Number(l.pow(r as u32)))
                    }
                }
                (l, r) => Err(RuntimeError::InvalidOperation {
                    operation: "exponentiation".to_string(),
                    left_type: l.type_name().to_string(),
                    right_type: r.type_name().to_string(),
                }),
            },
            
            // Comparison operations
            BinaryOp::Equal => Ok(Value::Bool(left == right)),
            BinaryOp::NotEqual => Ok(Value::Bool(left != right)),
            
            BinaryOp::Less => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l < r)),
                (l, r) => Err(RuntimeError::InvalidOperation {
                    operation: "comparison".to_string(),
                    left_type: l.type_name().to_string(),
                    right_type: r.type_name().to_string(),
                }),
            },
            
            BinaryOp::Greater => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l > r)),
                (l, r) => Err(RuntimeError::InvalidOperation {
                    operation: "comparison".to_string(),
                    left_type: l.type_name().to_string(),
                    right_type: r.type_name().to_string(),
                }),
            },
            
            BinaryOp::LessEqual => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l <= r)),
                (l, r) => Err(RuntimeError::InvalidOperation {
                    operation: "comparison".to_string(),
                    left_type: l.type_name().to_string(),
                    right_type: r.type_name().to_string(),
                }),
            },
            
            BinaryOp::GreaterEqual => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l >= r)),
                (l, r) => Err(RuntimeError::InvalidOperation {
                    operation: "comparison".to_string(),
                    left_type: l.type_name().to_string(),
                    right_type: r.type_name().to_string(),
                }),
            },
            
            // Logical operations
            BinaryOp::And => Ok(Value::Bool(left.as_bool() && right.as_bool())),
            BinaryOp::Or => Ok(Value::Bool(left.as_bool() || right.as_bool())),
        }
    }

    /// Evaluate a unary operation
    fn eval_unary_op(&self, op: UnaryOp, right: Value) -> RuntimeResult<Value> {
        match op {
            UnaryOp::Minus => match right {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(RuntimeError::TypeMismatch {
                    operation: "unary minus".to_string(),
                    expected: "Int".to_string(),
                    found: right.type_name().to_string(),
                }),
            },
            
            UnaryOp::Not => Ok(Value::Bool(!right.as_bool())),
        }
    }

    /// Evaluate a function call
    fn eval_call(&mut self, callee: Expr, args: Vec<Expr>) -> RuntimeResult<Value> {
        // Evaluate arguments first
        let mut arg_values = Vec::new();
        for arg in args {
            arg_values.push(self.eval_expr(arg)?);
        }

        // Get the function
        match callee {
            Expr::Variable(name) => {
                // Check if it's a built-in function
                if builtin::is_builtin(&name) {
                    return builtin::call_builtin(&name, arg_values);
                }

                // Otherwise, get user-defined function
                let func = self.env.get(&name)?;
                
                match func {
                    Value::Function { name: fn_name, params, body } => {
                        // Check arity
                        if params.len() != arg_values.len() {
                            return Err(RuntimeError::ArgumentCountMismatch {
                                expected: params.len(),
                                found: arg_values.len(),
                                function_name: fn_name,
                            });
                        }

                        // Create new scope for function
                        self.env.push_scope();
                        
                        // Bind parameters
                        for (param, arg) in params.iter().zip(arg_values.iter()) {
                            self.env.define(param.clone(), arg.clone());
                        }

                        // Execute function body
                        let was_in_function = self.in_function;
                        self.in_function = true;
                        
                        let result = match self.execute_stmt_cf(*body) {
                            Ok(val) => Ok(val),
                            Err(ControlFlow::Return(val)) => Ok(val),
                            Err(ControlFlow::Error(e)) => Err(e),
                        };

                        self.in_function = was_in_function;
                        self.env.pop_scope();
                        
                        result
                    }
                    _ => Err(RuntimeError::NotCallable {
                        value_type: func.type_name().to_string(),
                    }),
                }
            }
            _ => Err(RuntimeError::Custom("Can only call functions by name".to_string())),
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}
