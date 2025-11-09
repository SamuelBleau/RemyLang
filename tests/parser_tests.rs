/*
 * -------------------------------------------------------------------------
 * RemyLang — Parser Tests
 * File : tests/parser_tests.rs
 *
 * Description :
 *   Unit tests for the RemyLang parser implementation.
 *
 * Author  : Samuel 'Meeast' Bleau
 * Created : 2025-11-09
 *
 * -------------------------------------------------------------------------
*/

use remylang::lexer::Lexer;
use remylang::parser::Parser;
use remylang::ast::*;

// Helper function to parse code
fn parse(code: &str) -> Result<Vec<Stmt>, remylang::parser::ParseError> {
    let mut lexer = Lexer::new(code.to_string());
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        if token == remylang::lexer::Token::EOF {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }
    let mut parser = Parser::new(tokens);
    parser.parse()
}

// ============================================================================
// EXPRESSION TESTS
// ============================================================================

#[test]
fn test_literal_number() {
    let code = "42;";
    let ast = parse(code).unwrap();
    assert_eq!(ast.len(), 1);
    
    match &ast[0] {
        Stmt::Expression(Expr::Literal(Literal::Number(n))) => {
            assert_eq!(*n, 42);
        }
        _ => panic!("Expected number literal expression"),
    }
}

#[test]
fn test_literal_string() {
    let code = r#""Hello, World!";"#;
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::Expression(Expr::Literal(Literal::String(s))) => {
            assert_eq!(s, "Hello, World!");
        }
        _ => panic!("Expected string literal"),
    }
}

#[test]
fn test_literal_char() {
    let code = "'a';";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::Expression(Expr::Literal(Literal::Char(c))) => {
            assert_eq!(*c, 'a');
        }
        _ => panic!("Expected char literal"),
    }
}

#[test]
fn test_literal_bool_true() {
    let code = "True;";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::Expression(Expr::Literal(Literal::Bool(b))) => {
            assert_eq!(*b, true);
        }
        _ => panic!("Expected bool literal"),
    }
}

#[test]
fn test_literal_bool_false() {
    let code = "False;";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::Expression(Expr::Literal(Literal::Bool(b))) => {
            assert_eq!(*b, false);
        }
        _ => panic!("Expected bool literal"),
    }
}

#[test]
fn test_variable() {
    let code = "myVar;";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::Expression(Expr::Variable(name)) => {
            assert_eq!(name, "myVar");
        }
        _ => panic!("Expected variable"),
    }
}

// ============================================================================
// BINARY OPERATIONS TESTS
// ============================================================================

#[test]
fn test_addition() {
    let code = "1 + 2;";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::Expression(Expr::Binary { left, op, right }) => {
            assert_eq!(*op, BinaryOp::Add);
            assert!(matches!(**left, Expr::Literal(Literal::Number(1))));
            assert!(matches!(**right, Expr::Literal(Literal::Number(2))));
        }
        _ => panic!("Expected binary operation"),
    }
}

#[test]
fn test_precedence_multiplication_over_addition() {
    let code = "1 + 2 * 3;";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::Expression(Expr::Binary { left, op, right }) => {
            assert_eq!(*op, BinaryOp::Add);
            assert!(matches!(**left, Expr::Literal(Literal::Number(1))));
            
            // Right side should be 2 * 3
            match &**right {
                Expr::Binary { left: l2, op: op2, right: r2 } => {
                    assert_eq!(*op2, BinaryOp::Mul);
                    assert!(matches!(**l2, Expr::Literal(Literal::Number(2))));
                    assert!(matches!(**r2, Expr::Literal(Literal::Number(3))));
                }
                _ => panic!("Expected multiplication on right"),
            }
        }
        _ => panic!("Expected binary operation"),
    }
}

#[test]
fn test_power_right_associativity() {
    let code = "2 ** 3 ** 2;";
    let ast = parse(code).unwrap();
    
    // Should parse as 2 ** (3 ** 2) = 2 ** 9 = 512
    match &ast[0] {
        Stmt::Expression(Expr::Binary { left, op, right }) => {
            assert_eq!(*op, BinaryOp::Pow);
            assert!(matches!(**left, Expr::Literal(Literal::Number(2))));
            
            // Right side should be 3 ** 2
            match &**right {
                Expr::Binary { left: l2, op: op2, right: r2 } => {
                    assert_eq!(*op2, BinaryOp::Pow);
                    assert!(matches!(**l2, Expr::Literal(Literal::Number(3))));
                    assert!(matches!(**r2, Expr::Literal(Literal::Number(2))));
                }
                _ => panic!("Expected power on right"),
            }
        }
        _ => panic!("Expected binary operation"),
    }
}

#[test]
fn test_comparison() {
    let code = "x > 10;";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::Expression(Expr::Binary { left, op, right }) => {
            assert_eq!(*op, BinaryOp::Greater);
            assert!(matches!(**left, Expr::Variable(_)));
            assert!(matches!(**right, Expr::Literal(Literal::Number(10))));
        }
        _ => panic!("Expected comparison"),
    }
}

#[test]
fn test_logical_and() {
    let code = "True && False;";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::Expression(Expr::Binary { left, op, right }) => {
            assert_eq!(*op, BinaryOp::And);
            assert!(matches!(**left, Expr::Literal(Literal::Bool(true))));
            assert!(matches!(**right, Expr::Literal(Literal::Bool(false))));
        }
        _ => panic!("Expected logical AND"),
    }
}

#[test]
fn test_logical_or() {
    let code = "True || False;";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::Expression(Expr::Binary { op, .. }) => {
            assert_eq!(*op, BinaryOp::Or);
        }
        _ => panic!("Expected logical OR"),
    }
}

// ============================================================================
// UNARY OPERATIONS TESTS
// ============================================================================

#[test]
fn test_unary_minus() {
    let code = "-42;";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::Expression(Expr::Unary { op, right }) => {
            assert_eq!(*op, UnaryOp::Minus);
            assert!(matches!(**right, Expr::Literal(Literal::Number(42))));
        }
        _ => panic!("Expected unary minus"),
    }
}

#[test]
fn test_unary_not() {
    let code = "!True;";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::Expression(Expr::Unary { op, right }) => {
            assert_eq!(*op, UnaryOp::Not);
            assert!(matches!(**right, Expr::Literal(Literal::Bool(true))));
        }
        _ => panic!("Expected unary not"),
    }
}

// ============================================================================
// PARENTHESES TESTS
// ============================================================================

#[test]
fn test_parentheses() {
    let code = "(1 + 2) * 3;";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::Expression(Expr::Binary { left, op, right }) => {
            assert_eq!(*op, BinaryOp::Mul);
            
            // Left side should be (1 + 2)
            match &**left {
                Expr::Binary { left: l2, op: op2, right: r2 } => {
                    assert_eq!(*op2, BinaryOp::Add);
                    assert!(matches!(**l2, Expr::Literal(Literal::Number(1))));
                    assert!(matches!(**r2, Expr::Literal(Literal::Number(2))));
                }
                _ => panic!("Expected addition in parentheses"),
            }
            
            assert!(matches!(**right, Expr::Literal(Literal::Number(3))));
        }
        _ => panic!("Expected binary operation"),
    }
}

// ============================================================================
// FUNCTION CALL TESTS
// ============================================================================

#[test]
fn test_function_call_no_args() {
    let code = "foo();";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::Expression(Expr::Call { callee, args }) => {
            assert!(matches!(**callee, Expr::Variable(_)));
            assert_eq!(args.len(), 0);
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_function_call_with_args() {
    let code = "add(1, 2);";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::Expression(Expr::Call { callee, args }) => {
            assert!(matches!(**callee, Expr::Variable(_)));
            assert_eq!(args.len(), 2);
            assert!(matches!(args[0], Expr::Literal(Literal::Number(1))));
            assert!(matches!(args[1], Expr::Literal(Literal::Number(2))));
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_nested_function_call() {
    let code = "outer(inner(5));";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::Expression(Expr::Call { callee, args }) => {
            assert!(matches!(**callee, Expr::Variable(_)));
            assert_eq!(args.len(), 1);
            
            // First arg should be inner(5)
            match &args[0] {
                Expr::Call { args: inner_args, .. } => {
                    assert_eq!(inner_args.len(), 1);
                    assert!(matches!(inner_args[0], Expr::Literal(Literal::Number(5))));
                }
                _ => panic!("Expected nested call"),
            }
        }
        _ => panic!("Expected function call"),
    }
}

// ============================================================================
// ARRAY TESTS
// ============================================================================

#[test]
fn test_array_literal_empty() {
    let code = "[];";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::Expression(Expr::ArrayLiteral(elements)) => {
            assert_eq!(elements.len(), 0);
        }
        _ => panic!("Expected array literal"),
    }
}

#[test]
fn test_array_literal_with_elements() {
    let code = "[1, 2, 3];";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::Expression(Expr::ArrayLiteral(elements)) => {
            assert_eq!(elements.len(), 3);
            assert!(matches!(elements[0], Expr::Literal(Literal::Number(1))));
            assert!(matches!(elements[1], Expr::Literal(Literal::Number(2))));
            assert!(matches!(elements[2], Expr::Literal(Literal::Number(3))));
        }
        _ => panic!("Expected array literal"),
    }
}

#[test]
fn test_array_access() {
    let code = "arr[0];";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::Expression(Expr::ArrayAccess { array, index }) => {
            assert!(matches!(**array, Expr::Variable(_)));
            assert!(matches!(**index, Expr::Literal(Literal::Number(0))));
        }
        _ => panic!("Expected array access"),
    }
}

#[test]
fn test_nested_array_access() {
    let code = "matrix[i][j];";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::Expression(Expr::ArrayAccess { array, index }) => {
            // array should be matrix[i]
            match &**array {
                Expr::ArrayAccess { array: inner_array, index: inner_index } => {
                    assert!(matches!(**inner_array, Expr::Variable(_)));
                    assert!(matches!(**inner_index, Expr::Variable(_)));
                }
                _ => panic!("Expected nested array access"),
            }
            assert!(matches!(**index, Expr::Variable(_)));
        }
        _ => panic!("Expected array access"),
    }
}

// ============================================================================
// STATEMENT TESTS
// ============================================================================

#[test]
fn test_let_statement() {
    let code = "Int x = 42;";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::Let { name, type_annotation, value } => {
            assert_eq!(name, "x");
            assert_eq!(*type_annotation, Some(Type::Int));
            assert!(matches!(*value, Expr::Literal(Literal::Number(42))));
        }
        _ => panic!("Expected let statement"),
    }
}

#[test]
fn test_assignment_statement() {
    let code = "x = 100;";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::Assignment { name, value } => {
            assert_eq!(name, "x");
            assert!(matches!(*value, Expr::Literal(Literal::Number(100))));
        }
        _ => panic!("Expected assignment statement"),
    }
}

#[test]
fn test_block_statement() {
    let code = "{ Int x = 1; Int y = 2; }";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::Block(statements) => {
            assert_eq!(statements.len(), 2);
            assert!(matches!(statements[0], Stmt::Let { .. }));
            assert!(matches!(statements[1], Stmt::Let { .. }));
        }
        _ => panic!("Expected block statement"),
    }
}

#[test]
fn test_return_statement_with_value() {
    let code = "return 42;";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::Return(Some(expr)) => {
            assert!(matches!(*expr, Expr::Literal(Literal::Number(42))));
        }
        _ => panic!("Expected return statement with value"),
    }
}

#[test]
fn test_return_statement_without_value() {
    let code = "return;";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::Return(None) => {
            // Success
        }
        _ => panic!("Expected return statement without value"),
    }
}

// ============================================================================
// IF STATEMENT TESTS
// ============================================================================

#[test]
fn test_if_statement_without_else() {
    let code = "if (x > 0) { return x; }";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::If { condition, then_branch, else_branch } => {
            assert!(matches!(*condition, Expr::Binary { .. }));
            assert!(matches!(**then_branch, Stmt::Block(_)));
            assert!(else_branch.is_none());
        }
        _ => panic!("Expected if statement"),
    }
}

#[test]
fn test_if_statement_with_else() {
    let code = "if (x > 0) { return x; } else { return 0; }";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::If { condition, then_branch, else_branch } => {
            assert!(matches!(*condition, Expr::Binary { .. }));
            assert!(matches!(**then_branch, Stmt::Block(_)));
            assert!(else_branch.is_some());
            
            if let Some(else_b) = else_branch {
                assert!(matches!(**else_b, Stmt::Block(_)));
            }
        }
        _ => panic!("Expected if statement with else"),
    }
}

#[test]
fn test_if_else_if_chain() {
    let code = "if (x > 10) { return 1; } else if (x > 5) { return 2; } else { return 3; }";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::If { condition, then_branch, else_branch } => {
            assert!(matches!(*condition, Expr::Binary { .. }));
            assert!(matches!(**then_branch, Stmt::Block(_)));
            
            // else_branch should contain another if statement
            if let Some(else_b) = else_branch {
                assert!(matches!(**else_b, Stmt::If { .. }));
            } else {
                panic!("Expected else if");
            }
        }
        _ => panic!("Expected if statement"),
    }
}

// ============================================================================
// FUNCTION DECLARATION TESTS
// ============================================================================

#[test]
fn test_function_no_params_no_return() {
    let code = "func Test() { return; }";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::FunctionDecl { name, params, return_type, body } => {
            assert_eq!(name, "Test");
            assert_eq!(params.len(), 0);
            assert!(return_type.is_none());
            assert!(matches!(**body, Stmt::Block(_)));
        }
        _ => panic!("Expected function declaration"),
    }
}

#[test]
fn test_function_with_params_and_return() {
    let code = "func Add(Int a, Int b) -> Int { return a + b; }";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::FunctionDecl { name, params, return_type, body } => {
            assert_eq!(name, "Add");
            assert_eq!(params.len(), 2);
            assert_eq!(params[0].name, "a");
            assert_eq!(params[0].type_annotation, Type::Int);
            assert_eq!(params[1].name, "b");
            assert_eq!(params[1].type_annotation, Type::Int);
            assert_eq!(*return_type, Some(Type::Int));
            assert!(matches!(**body, Stmt::Block(_)));
        }
        _ => panic!("Expected function declaration"),
    }
}

#[test]
fn test_function_with_array_type() {
    let code = "func GetArray() -> Array<Int> { return []; }";
    let ast = parse(code).unwrap();
    
    match &ast[0] {
        Stmt::FunctionDecl { name, params, return_type, body } => {
            assert_eq!(name, "GetArray");
            assert_eq!(params.len(), 0);
            
            match return_type {
                Some(Type::Array(inner)) => {
                    assert_eq!(**inner, Type::Int);
                }
                _ => panic!("Expected Array<Int> return type"),
            }
            
            assert!(matches!(**body, Stmt::Block(_)));
        }
        _ => panic!("Expected function declaration"),
    }
}

// ============================================================================
// COMPLEX TESTS
// ============================================================================

#[test]
fn test_complex_expression() {
    let code = "(a + b) * c - d / e ** 2;";
    let ast = parse(code).unwrap();
    
    // Just verify it parses without error
    assert_eq!(ast.len(), 1);
    assert!(matches!(ast[0], Stmt::Expression(Expr::Binary { .. })));
}

#[test]
fn test_multiple_statements() {
    let code = r#"
        Int x = 10;
        Int y = 20;
        Int sum = x + y;
    "#;
    let ast = parse(code).unwrap();
    
    assert_eq!(ast.len(), 3);
    assert!(matches!(ast[0], Stmt::Let { .. }));
    assert!(matches!(ast[1], Stmt::Let { .. }));
    assert!(matches!(ast[2], Stmt::Let { .. }));
}

#[test]
fn test_full_program() {
    let code = r#"
        func Fibonacci(Int n) -> Int {
            if (n <= 1) {
                return n;
            } else {
                return Fibonacci(n - 1) + Fibonacci(n - 2);
            }
        }
        
        func Main() -> Int {
            Int result = Fibonacci(10);
            return result;
        }
    "#;
    
    let ast = parse(code).unwrap();
    assert_eq!(ast.len(), 2);
    assert!(matches!(ast[0], Stmt::FunctionDecl { .. }));
    assert!(matches!(ast[1], Stmt::FunctionDecl { .. }));
}

// ============================================================================
// ERROR TESTS
// ============================================================================

#[test]
fn test_error_missing_semicolon() {
    let code = "Int x = 42";
    let result = parse(code);
    assert!(result.is_err());
}

#[test]
fn test_error_unclosed_paren() {
    let code = "(1 + 2;";
    let result = parse(code);
    assert!(result.is_err());
}

#[test]
fn test_error_invalid_expression() {
    let code = "+ 5;";
    let result = parse(code);
    assert!(result.is_err());
}

#[test]
fn test_error_missing_function_body() {
    let code = "func Test();";
    let result = parse(code);
    assert!(result.is_err());
}
