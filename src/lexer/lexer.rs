/*
 * -------------------------------------------------------------------------
 * RemyLang — lexer implementation
 * File : src/lexer/lexer.rs
 *
 * Description :
 *   Tokenizer that converts source code into a stream of tokens.
 *
 * Author  : Samuel 'Meeast' Bleau
 * Created : 2025-11-04
 *
 * -------------------------------------------------------------------------
*/

use super::token::Token;

/// Main lexer structure
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>, //Sort of either char or None
    line: usize,
    column: usize,
}

impl Lexer {
    /// Creates a new Lexer from source code
    /// 
    /// # Arguments
    /// * `input` - Source code string
    /// 
    /// # Example
    /// ```
    /// let lexer = Lexer::new("Int nb = 42;".to_string());
    /// ```
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            current_char: None,
            line: 1,
            column: 0,
        };
        lexer.current_char = lexer.input.get(0).cloned();
        lexer
    }

    /// Advances to the next character in the input
    fn advance(&mut self) {
        // Handle newline tracking for error reporting
        if let Some('\n') = self.current_char {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }

        // Move to next character
        if self.position + 1 >= self.input.len() {
            self.current_char = None;
        } else {
            self.position += 1;
            self.current_char = Some(self.input[self.position]);
        }
    }

    /// Peeks at the next character without consuming it
    fn peek(&self) -> Option<char> {
        if self.position + 1 >= self.input.len() {
            None
        } else {
            Some(self.input[self.position + 1])
        }
            
    }

    /// Peeks ahead n characters without consuming them
    fn peek_ahead(&self, n: usize) -> Option<char> {
        if self.position + n >= self.input.len() {
            None
        } else {
            Some(self.input[self.position + n])
        }
    }

    // WHITESPACE AND COMMENTS HANDLING

    /// Skips whitespace characters
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Skips a line comment
    fn skip_line_comment(&mut self) {
        while let Some(c) = self.current_char {
            if c != '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Skips a block comment
    fn skip_block_comment(&mut self) {
        while let Some(c) = self.current_char {
            if c == '*' && self.peek() == Some('/') {
                self.advance();
                self.advance();
                break;
            } else {
                self.advance();
            }
        }
    }

    // TOKEN RECOGNITION

    /// Reads an integer literal
    fn read_number(&mut self) -> Token {
        let mut val = String::new();
        while let Some(c) = self.current_char {
            if c.is_ascii_digit() {
                val.push(c);
                self.advance();
            } else {
                break;
            }
        }
        Token::Number(val.parse::<i64>().unwrap())
    }

    /// Reads an identifier or keyword
    fn read_identifier(&mut self) -> Token {
        let mut val = String::new();
        while let Some(c) = self.current_char {
            if c.is_ascii_alphanumeric() || c == '_' {
                val.push(c);
                self.advance();
            } else {
                break;
            }
        }
        Token::Identifier(val)
    }

    /// Check if a string is a keyword and returns appropriate token
    fn keyword_or_identifier(&self, ident: String) -> Token {
    }
}