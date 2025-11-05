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
    /// use remylang::lexer::Lexer;
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

    // NOTE: peek_ahead() can be added later if needed for complex multi-char operators
    // /// Peeks ahead n characters without consuming them
    // fn peek_ahead(&self, n: usize) -> Option<char> {
    //     if self.position + n >= self.input.len() {
    //         None
    //     } else {
    //         Some(self.input[self.position + n])
    //     }
    // }

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

    // TOKEN RECOGNITION - IDENTIFIERS AND KEYWORDS

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
        self.keyword_or_identifier(val)
    }

    /// Check if a string is a keyword and returns appropriate token
    fn keyword_or_identifier(&self, ident: String) -> Token {
        match ident.as_str() {
            "Int" => Token::Int,
            "String" => Token::String,
            "Char" => Token::Char,
            "Bool" => Token::Bool,
            "Array" => Token::Array,
            "True" => Token::True,
            "False" => Token::False,
            "func" => Token::Func,
            "return" => Token::Return,
            "if" => Token::If,
            "else" => Token::Else,
            _ => Token::Identifier(ident),
        }
    }

    // TOKEN RECOGNITION - STRINGS AND CHARACTERS

    /// Reads a string literal
    fn read_string(&mut self) -> Token {
        let mut val = String::new();
        self.advance();
        while let Some(c) = self.current_char {
            if c != '"' {
                val.push(c);
                self.advance();
            } else {
                break;
            }
        }
        self.advance();
        Token::StringLit(val)
    }

    /// Reads a character literal
    fn read_character(&mut self) -> Token {
        self.advance(); // Skip opening '
        
        let ch = if let Some(c) = self.current_char {
            self.advance();
            c
        } else {
            return Token::Invalid('\''); // Empty character literal
        };
        
        // Expect closing '
        if self.current_char != Some('\'') {
            return Token::Invalid('\''); // Missing closing '
        }
        self.advance(); // Skip closing '
        
        Token::CharLit(ch)
    }

    // TOKENIZATION LOGIC

    /// Retrieves the next token from the input
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.current_char {
            None => Token::EOF,
            Some(ch) if ch.is_ascii_digit() => self.read_number(),
            Some(ch) if ch.is_alphabetic() || ch == '_' => self.read_identifier(),
            Some('"') => self.read_string(),
            Some('\'') => self.read_character(),
            Some('+') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::PlusEqual
                } else if self.current_char == Some('+') {
                    self.advance();
                    // TODO: Add Token::Increment in token.rs if needed
                    Token::Invalid('+') // For now, treat as invalid
                } else {
                    Token::Plus
                }
            },
            Some('-') => {
                self.advance();
                if self.current_char == Some('>') {
                    self.advance();
                    Token::Arrow
                } else if self.current_char == Some('=') {
                    self.advance();
                    Token::MinusEqual
                } else {
                    Token::Minus
                }
            },
            Some('*') => {
                self.advance();
                if self.current_char == Some('*') {
                    self.advance();
                    Token::Power
                } else if self.current_char == Some('=') {
                    self.advance();
                    Token::StarEqual
                } else {
                    Token::Star
                }
            },
            Some('/') => {
                self.advance();
                if self.current_char == Some('/') {
                    self.skip_line_comment();
                    self.next_token() // Recursively get next token after comment
                } else if self.current_char == Some('*') {
                    self.advance();
                    self.skip_block_comment();
                    self.next_token() // Recursively get next token after comment
                } else if self.current_char == Some('=') {
                    self.advance();
                    Token::SlashEqual
                } else {
                    Token::Slash
                }
            },
            Some('%') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::PercentEqual
                } else {
                    Token::Percent
                }
            },
            Some('=') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::EqualEqual
                } else {
                    Token::Equal
                }
            },
            Some('!') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::BangEqual
                } else {
                    Token::Bang
                }
            },
            Some('<') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::LessEqual
                } else {
                    Token::Less
                }
            },
            Some('>') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::GreaterEqual
                } else {
                    Token::Greater
                }
            },
            Some('&') => {
                self.advance();
                if self.current_char == Some('&') {
                    self.advance();
                    Token::And
                } else {
                    Token::Invalid('&')
                }
            },
            Some('|') => {
                self.advance();
                if self.current_char == Some('|') {
                    self.advance();
                    Token::Or
                } else {
                    Token::Invalid('|')
                }
            },
            Some('(') => {
                self.advance();
                Token::LeftParen
            },
            Some(')') => {
                self.advance();
                Token::RightParen
            },
            Some('{') => {
                self.advance();
                Token::LeftBrace
            },
            Some('}') => {
                self.advance();
                Token::RightBrace
            },
            Some('[') => {
                self.advance();
                Token::LeftBracket
            },
            Some(']') => {
                self.advance();
                Token::RightBracket
            },
            Some(';') => {
                self.advance();
                Token::Semicolon
            },
            Some(',') => {
                self.advance();
                Token::Comma
            },
            Some(ch) => {
                self.advance();
                Token::Invalid(ch)
            },
        }
    }
}

// Iterator Implementation


impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        if token == Token::EOF {
            None
        } else {
            Some(token)
        }
    }
}
