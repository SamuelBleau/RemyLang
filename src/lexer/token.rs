/*
 * -------------------------------------------------------------------------
 * RemyLang — token definitions
 * File : src/lexer/token.rs
 *
 * Description :
 *   Token definitions for the lexer component.
 *
 * Author  : Samuel 'Meeast' Bleau
 * Created : 2025-11-04
 *
 * -------------------------------------------------------------------------
*/

#[derive(Debug, Clone, PartialEq)]

pub enum Token {
    // Literals
    Number(i64),
    StringLit(String),
    CharLit(char),
    Identifier(String), // variable or function name

    // Keywords
    Int,
    String,
    Char,
    Bool,
    Array,
    True,
    False,

    Func,
    Return,
    If,
    Else,
    //TODO: Implement loop keywords
    // While,
    // For,
    // Break,
    // Continue,

    // Operators - Arithmetic
    Plus,
    Minus,
    Star,
    Power,
    Slash,
    Percent,

    // Operators - Comparison
    EqualEqual,
    BangEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,

    // Operators - Logical
    And,
    Or,
    Bang,

    // Operators - Assignment
    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    // Punctuation
    Semicolon,
    Comma,
    Arrow,

    EOF,
    Invalid(char),
}

impl Token {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Token::Int
                | Token::String
                | Token::Char
                | Token::Bool
                | Token::Array
                | Token::True
                | Token::False
                | Token::Func
                | Token::Return
                | Token::If
                | Token::Else
                | Token::While
                | Token::For
                | Token::Break
                | Token::Continue
        )
    }

    pub fn is_literal(&self) -> bool {
        matches!(
            self,
            Token::Number(_) | Token::StringLit(_) | Token::CharLit(_) | Token::True | Token::False
        )
    }

    pub fn is_operator(&self) -> bool {
        matches!(
            self,
            Token::Plus
                | Token::Minus
                | Token::Star
                | Token::Power
                | Token::Slash
                | Token::Percent
                | Token::EqualEqual
                | Token::BangEqual
                | Token::Less
                | Token::Greater
                | Token::LessEqual
                | Token::GreaterEqual
                | Token::And
                | Token::Or
                | Token::Bang
        )
    }
}
