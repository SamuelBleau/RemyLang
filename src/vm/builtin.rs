/*
 * -------------------------------------------------------------------------
 * RemyLang — builtin functions implementation
 * File : src/vm/builtin.rs
 *
 * Description :
 *   Built-in functions for the RemyLang VM.
 *
 * Author  : Samuel 'Meeast' Bleau
 * Created : 2025-12-20
 *
 * -------------------------------------------------------------------------
*/

use crate::vm::value::Value;
use crate::vm::error::{RuntimeError, RuntimeResult};

/// Execute a built-in function
pub fn call_builtin(name: &str, args: Vec<Value>) -> RuntimeResult<Value> {
    match name {
        "print" => builtin_print(args),
        _ => Err(RuntimeError::UndefinedFunction { name: name.to_string() }),
    }
}

/// Built-in: print(...) - prints values to stdout
fn builtin_print(args: Vec<Value>) -> RuntimeResult<Value> {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", arg);
    }
    println!();
    Ok(Value::Void)
}

/// Check if a function name is a built-in
pub fn is_builtin(name: &str) -> bool {
    matches!(name, "print")
}
