/*
 * -------------------------------------------------------------------------
 * RemyLang — environment implementation
 * File : src/vm/environment.rs
 *
 * Description :
 *   Variable scope management for the RemyLang VM.
 *
 * Author  : Samuel 'Meeast' Bleau
 * Created : 2025-12-20
 *
 * -------------------------------------------------------------------------
*/

use std::collections::HashMap;
use crate::vm::value::Value;
use crate::vm::error::{RuntimeError, RuntimeResult};

/// Environment for managing variable scopes
#[derive(Debug, Clone)]
pub struct Environment {
    scopes: Vec<HashMap<String, Value>>,
}

impl Environment {
    /// Create a new environment with a global scope
    pub fn new() -> Self {
        Environment {
            scopes: vec![HashMap::new()],
        }
    }

    /// Push a new scope (for blocks, functions, etc.)
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Pop the current scope
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    /// Define a new variable in the current scope
    pub fn define(&mut self, name: String, value: Value) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
        }
    }

    /// Get a variable value (searches from innermost to outermost scope)
    pub fn get(&self, name: &str) -> RuntimeResult<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Ok(value.clone());
            }
        }
        Err(RuntimeError::UndefinedVariable { name: name.to_string() })
    }

    /// Update an existing variable (searches from innermost to outermost scope)
    pub fn set(&mut self, name: &str, value: Value) -> RuntimeResult<()> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return Ok(());
            }
        }
        Err(RuntimeError::AssignmentToUndefined { name: name.to_string() })
    }

    /// Check if a variable exists in any scope
    pub fn exists(&self, name: &str) -> bool {
        self.scopes.iter().rev().any(|scope| scope.contains_key(name))
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
