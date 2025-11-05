/*
 * -------------------------------------------------------------------------
 * RemyLang — Lexer Tests
 * File : tests/lexer_tests.rs
 *
 * Description :
 *   Integration tests for the lexer component.
 *
 * Author  : Samuel 'Meeast' Bleau
 * Created : 2025-11-05
 *
 * -------------------------------------------------------------------------
*/

use remylang::lexer::{Lexer, Token};

// Helper function to tokenize input into a Vec
fn tokenize(input: &str) -> Vec<Token> {
    Lexer::new(input.to_string()).collect()
}

// =============================================================================
// Basic Tokens
// =============================================================================

#[test]
fn test_empty_input() {
    let mut lexer = Lexer::new("".to_string());
    assert_eq!(lexer.next_token(), Token::EOF);
}

#[test]
fn test_single_number() {
    let mut lexer = Lexer::new("42".to_string());
    assert_eq!(lexer.next_token(), Token::Number(42));
    assert_eq!(lexer.next_token(), Token::EOF);
}

#[test]
fn test_multiple_numbers() {
    let tokens = tokenize("123 456 789");
    assert_eq!(
        tokens,
        vec![Token::Number(123), Token::Number(456), Token::Number(789)]
    );
}

#[test]
fn test_whitespace_handling() {
    let tokens = tokenize("  42  \n\t  100  ");
    assert_eq!(tokens, vec![Token::Number(42), Token::Number(100)]);
}

// =============================================================================
// Arithmetic Operators
// =============================================================================

#[test]
fn test_arithmetic_operators() {
    let tokens = tokenize("+ - * / % **");
    assert_eq!(
        tokens,
        vec![
            Token::Plus,
            Token::Minus,
            Token::Star,
            Token::Slash,
            Token::Percent,
            Token::Power
        ]
    );
}

#[test]
fn test_simple_arithmetic_expression() {
    let tokens = tokenize("1 + 2 - 3");
    assert_eq!(
        tokens,
        vec![
            Token::Number(1),
            Token::Plus,
            Token::Number(2),
            Token::Minus,
            Token::Number(3)
        ]
    );
}

#[test]
fn test_complex_arithmetic() {
    let tokens = tokenize("5 * 3 + 10 / 2 - 4 % 2");
    assert_eq!(
        tokens,
        vec![
            Token::Number(5),
            Token::Star,
            Token::Number(3),
            Token::Plus,
            Token::Number(10),
            Token::Slash,
            Token::Number(2),
            Token::Minus,
            Token::Number(4),
            Token::Percent,
            Token::Number(2)
        ]
    );
}

#[test]
fn test_power_operator() {
    let tokens = tokenize("2 ** 8");
    assert_eq!(
        tokens,
        vec![Token::Number(2), Token::Power, Token::Number(8)]
    );
}

// =============================================================================
// Comparison Operators
// =============================================================================

#[test]
fn test_comparison_operators() {
    let tokens = tokenize("== != < > <= >=");
    assert_eq!(
        tokens,
        vec![
            Token::EqualEqual,
            Token::BangEqual,
            Token::Less,
            Token::Greater,
            Token::LessEqual,
            Token::GreaterEqual
        ]
    );
}

#[test]
fn test_comparison_expression() {
    let tokens = tokenize("x > 10");
    assert_eq!(
        tokens,
        vec![
            Token::Identifier("x".to_string()),
            Token::Greater,
            Token::Number(10)
        ]
    );
}

// =============================================================================
// Logical Operators
// =============================================================================

#[test]
fn test_logical_operators() {
    let tokens = tokenize("&& || !");
    assert_eq!(tokens, vec![Token::And, Token::Or, Token::Bang]);
}

#[test]
fn test_logical_expression() {
    let tokens = tokenize("a && b || !c");
    assert_eq!(
        tokens,
        vec![
            Token::Identifier("a".to_string()),
            Token::And,
            Token::Identifier("b".to_string()),
            Token::Or,
            Token::Bang,
            Token::Identifier("c".to_string())
        ]
    );
}

// =============================================================================
// Assignment Operators
// =============================================================================

#[test]
fn test_assignment_operators() {
    let tokens = tokenize("= += -= *= /= %=");
    assert_eq!(
        tokens,
        vec![
            Token::Equal,
            Token::PlusEqual,
            Token::MinusEqual,
            Token::StarEqual,
            Token::SlashEqual,
            Token::PercentEqual
        ]
    );
}

// =============================================================================
// Keywords
// =============================================================================

#[test]
fn test_type_keywords() {
    let tokens = tokenize("Int String Char Bool Array");
    assert_eq!(
        tokens,
        vec![
            Token::Int,
            Token::String,
            Token::Char,
            Token::Bool,
            Token::Array
        ]
    );
}

#[test]
fn test_boolean_literals() {
    let tokens = tokenize("True False");
    assert_eq!(tokens, vec![Token::True, Token::False]);
}

#[test]
fn test_control_flow_keywords() {
    let tokens = tokenize("func return if else");
    assert_eq!(
        tokens,
        vec![Token::Func, Token::Return, Token::If, Token::Else]
    );
}

// =============================================================================
// Identifiers
// =============================================================================

#[test]
fn test_simple_identifiers() {
    let tokens = tokenize("myVar _test hello123");
    assert_eq!(
        tokens,
        vec![
            Token::Identifier("myVar".to_string()),
            Token::Identifier("_test".to_string()),
            Token::Identifier("hello123".to_string())
        ]
    );
}

#[test]
fn test_identifier_vs_keyword() {
    let tokens = tokenize("Int myInt IntValue");
    assert_eq!(
        tokens,
        vec![
            Token::Int,
            Token::Identifier("myInt".to_string()),
            Token::Identifier("IntValue".to_string())
        ]
    );
}

// =============================================================================
// String Literals
// =============================================================================

#[test]
fn test_string_literal() {
    let tokens = tokenize(r#""Hello, World!""#);
    assert_eq!(tokens, vec![Token::StringLit("Hello, World!".to_string())]);
}

#[test]
fn test_empty_string() {
    let tokens = tokenize(r#""""#);
    assert_eq!(tokens, vec![Token::StringLit("".to_string())]);
}

#[test]
fn test_string_with_spaces() {
    let tokens = tokenize(r#""  spaces  inside  ""#);
    assert_eq!(
        tokens,
        vec![Token::StringLit("  spaces  inside  ".to_string())]
    );
}

// =============================================================================
// Character Literals
// =============================================================================

#[test]
fn test_char_literal() {
    let tokens = tokenize("'a'");
    assert_eq!(tokens, vec![Token::CharLit('a')]);
}

#[test]
fn test_multiple_chars() {
    let tokens = tokenize("'x' 'y' 'z'");
    assert_eq!(
        tokens,
        vec![Token::CharLit('x'), Token::CharLit('y'), Token::CharLit('z')]
    );
}

// =============================================================================
// Delimiters
// =============================================================================

#[test]
fn test_delimiters() {
    let tokens = tokenize("( ) { } [ ]");
    assert_eq!(
        tokens,
        vec![
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::RightBrace,
            Token::LeftBracket,
            Token::RightBracket
        ]
    );
}

// =============================================================================
// Punctuation
// =============================================================================

#[test]
fn test_punctuation() {
    let tokens = tokenize("; , ->");
    assert_eq!(tokens, vec![Token::Semicolon, Token::Comma, Token::Arrow]);
}

// =============================================================================
// Comments
// =============================================================================

#[test]
fn test_line_comment() {
    let tokens = tokenize("Int x // This is a comment\nInt y");
    assert_eq!(
        tokens,
        vec![
            Token::Int,
            Token::Identifier("x".to_string()),
            Token::Int,
            Token::Identifier("y".to_string())
        ]
    );
}

#[test]
fn test_line_comment_at_end() {
    let tokens = tokenize("42 // comment");
    assert_eq!(tokens, vec![Token::Number(42)]);
}

#[test]
fn test_block_comment() {
    let tokens = tokenize("Int x /* block comment */ Int y");
    assert_eq!(
        tokens,
        vec![
            Token::Int,
            Token::Identifier("x".to_string()),
            Token::Int,
            Token::Identifier("y".to_string())
        ]
    );
}

#[test]
fn test_multiline_block_comment() {
    let tokens = tokenize("Int a /* multi\nline\ncomment */ Int b");
    assert_eq!(
        tokens,
        vec![
            Token::Int,
            Token::Identifier("a".to_string()),
            Token::Int,
            Token::Identifier("b".to_string())
        ]
    );
}

// =============================================================================
// Variable Declarations
// =============================================================================

#[test]
fn test_simple_variable_declaration() {
    let tokens = tokenize("Int nb = 42;");
    assert_eq!(
        tokens,
        vec![
            Token::Int,
            Token::Identifier("nb".to_string()),
            Token::Equal,
            Token::Number(42),
            Token::Semicolon
        ]
    );
}

#[test]
fn test_string_variable_declaration() {
    let tokens = tokenize(r#"String str = "Hello";"#);
    assert_eq!(
        tokens,
        vec![
            Token::String,
            Token::Identifier("str".to_string()),
            Token::Equal,
            Token::StringLit("Hello".to_string()),
            Token::Semicolon
        ]
    );
}

#[test]
fn test_char_variable_declaration() {
    let tokens = tokenize("Char c = 'a';");
    assert_eq!(
        tokens,
        vec![
            Token::Char,
            Token::Identifier("c".to_string()),
            Token::Equal,
            Token::CharLit('a'),
            Token::Semicolon
        ]
    );
}

#[test]
fn test_bool_variable_declaration() {
    let tokens = tokenize("Bool isOk = True;");
    assert_eq!(
        tokens,
        vec![
            Token::Bool,
            Token::Identifier("isOk".to_string()),
            Token::Equal,
            Token::True,
            Token::Semicolon
        ]
    );
}

// =============================================================================
// Array Syntax
// =============================================================================

#[test]
fn test_array_type() {
    let tokens = tokenize("Array<Int>");
    assert_eq!(
        tokens,
        vec![Token::Array, Token::Less, Token::Int, Token::Greater]
    );
}

#[test]
fn test_array_declaration() {
    let tokens = tokenize("Array<Int> list = [1, 2, 3];");
    assert_eq!(
        tokens,
        vec![
            Token::Array,
            Token::Less,
            Token::Int,
            Token::Greater,
            Token::Identifier("list".to_string()),
            Token::Equal,
            Token::LeftBracket,
            Token::Number(1),
            Token::Comma,
            Token::Number(2),
            Token::Comma,
            Token::Number(3),
            Token::RightBracket,
            Token::Semicolon
        ]
    );
}

#[test]
fn test_empty_array() {
    let tokens = tokenize("Array<String> list = [];");
    assert_eq!(
        tokens,
        vec![
            Token::Array,
            Token::Less,
            Token::String,
            Token::Greater,
            Token::Identifier("list".to_string()),
            Token::Equal,
            Token::LeftBracket,
            Token::RightBracket,
            Token::Semicolon
        ]
    );
}

// =============================================================================
// Function Declarations
// =============================================================================

#[test]
fn test_simple_function() {
    let tokens = tokenize("func HelloWorld() {");
    assert_eq!(
        tokens,
        vec![
            Token::Func,
            Token::Identifier("HelloWorld".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace
        ]
    );
}

#[test]
fn test_function_with_params() {
    let tokens = tokenize("func Add(Int a, Int b) -> Int {");
    assert_eq!(
        tokens,
        vec![
            Token::Func,
            Token::Identifier("Add".to_string()),
            Token::LeftParen,
            Token::Int,
            Token::Identifier("a".to_string()),
            Token::Comma,
            Token::Int,
            Token::Identifier("b".to_string()),
            Token::RightParen,
            Token::Arrow,
            Token::Int,
            Token::LeftBrace
        ]
    );
}

#[test]
fn test_return_statement() {
    let tokens = tokenize("return 42;");
    assert_eq!(
        tokens,
        vec![Token::Return, Token::Number(42), Token::Semicolon]
    );
}

// =============================================================================
// Conditionals
// =============================================================================

#[test]
fn test_if_statement() {
    let tokens = tokenize("if (x > 10) {");
    assert_eq!(
        tokens,
        vec![
            Token::If,
            Token::LeftParen,
            Token::Identifier("x".to_string()),
            Token::Greater,
            Token::Number(10),
            Token::RightParen,
            Token::LeftBrace
        ]
    );
}

#[test]
fn test_if_else_statement() {
    let tokens = tokenize("if (x > 10) { } else {");
    assert_eq!(
        tokens,
        vec![
            Token::If,
            Token::LeftParen,
            Token::Identifier("x".to_string()),
            Token::Greater,
            Token::Number(10),
            Token::RightParen,
            Token::LeftBrace,
            Token::RightBrace,
            Token::Else,
            Token::LeftBrace
        ]
    );
}

#[test]
fn test_complex_condition() {
    let tokens = tokenize("if (age >= 18 && isStudent) {");
    assert_eq!(
        tokens,
        vec![
            Token::If,
            Token::LeftParen,
            Token::Identifier("age".to_string()),
            Token::GreaterEqual,
            Token::Number(18),
            Token::And,
            Token::Identifier("isStudent".to_string()),
            Token::RightParen,
            Token::LeftBrace
        ]
    );
}

// =============================================================================
// Compound Assignments
// =============================================================================

#[test]
fn test_compound_assignment() {
    let tokens = tokenize("a += 5;");
    assert_eq!(
        tokens,
        vec![
            Token::Identifier("a".to_string()),
            Token::PlusEqual,
            Token::Number(5),
            Token::Semicolon
        ]
    );
}

// =============================================================================
// Invalid Tokens
// =============================================================================

#[test]
fn test_invalid_character() {
    let tokens = tokenize("@");
    assert_eq!(tokens, vec![Token::Invalid('@')]);
}

#[test]
fn test_single_ampersand() {
    let tokens = tokenize("&");
    assert_eq!(tokens, vec![Token::Invalid('&')]);
}

#[test]
fn test_single_pipe() {
    let tokens = tokenize("|");
    assert_eq!(tokens, vec![Token::Invalid('|')]);
}

// =============================================================================
// Complete Program
// =============================================================================

#[test]
fn test_complete_program() {
    let input = r#"
func Add(Int a, Int b) -> Int {
    return a + b;
}

func Main() -> Int {
    Int result = Add(5, 10);
    return result;
}
"#;
    let tokens = tokenize(input);

    // Just verify it tokenizes without panicking and has reasonable length
    assert!(tokens.len() > 20);
    assert_eq!(tokens[0], Token::Func);
    assert_eq!(tokens[1], Token::Identifier("Add".to_string()));
}

// =============================================================================
// Iterator Tests
// =============================================================================

#[test]
fn test_iterator() {
    let lexer = Lexer::new("1 + 2".to_string());
    let tokens: Vec<Token> = lexer.collect();

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], Token::Number(1));
    assert_eq!(tokens[1], Token::Plus);
    assert_eq!(tokens[2], Token::Number(2));
}

#[test]
fn test_iterator_stops_at_eof() {
    let lexer = Lexer::new("42".to_string());
    let tokens: Vec<Token> = lexer.collect();

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0], Token::Number(42));
    // EOF should not be in the collected tokens
}
