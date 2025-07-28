// Token types for Ferrum language
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Number(i64),
    String(String),
    Identifier(String),

    // Keywords
    Let,
    Mut,

    // Operators
    Assign, // =
    Plus,   // +
    Minus,  // -

    // Punctuation
    Semicolon, // ;

    // Special
    Eof,
    Invalid(char),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Self {
        Self {
            token_type,
            line,
            column,
        }
    }
}

// TODO: Implement the Lexer struct and tokenization logic
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.current_char() {
            match ch {
                // Skip whitespace
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.column = 1;
                    self.advance();
                }

                // Numbers
                '0'..='9' => {
                    let number = self.read_number();
                    tokens.push(Token::new(
                        TokenType::Number(number),
                        self.line,
                        self.column,
                    ));
                }

                // Strings
                '"' => {
                    let string = self.read_string();
                    tokens.push(Token::new(
                        TokenType::String(string),
                        self.line,
                        self.column,
                    ));
                }

                // Identifiers and keywords
                'a'..='z' | 'A'..='Z' | '_' => {
                    let identifier = self.read_identifier();
                    let token_type = match identifier.as_str() {
                        "let" => TokenType::Let,
                        "mut" => TokenType::Mut,
                        _ => TokenType::Identifier(identifier),
                    };
                    tokens.push(Token::new(token_type, self.line, self.column));
                }

                // Single-character tokens
                '=' => {
                    tokens.push(Token::new(TokenType::Assign, self.line, self.column));
                    self.advance();
                }
                '+' => {
                    tokens.push(Token::new(TokenType::Plus, self.line, self.column));
                    self.advance();
                }
                '-' => {
                    tokens.push(Token::new(TokenType::Minus, self.line, self.column));
                    self.advance();
                }
                ';' => {
                    tokens.push(Token::new(TokenType::Semicolon, self.line, self.column));
                    self.advance();
                }

                // Invalid characters
                _ => {
                    tokens.push(Token::new(TokenType::Invalid(ch), self.line, self.column));
                    self.advance();
                }
            }
        }

        tokens.push(Token::new(TokenType::Eof, self.line, self.column));
        tokens
    }

    fn current_char(&self) -> Option<char> {
        if self.position >= self.input.len() {
            None
        } else {
            Some(self.input[self.position])
        }
    }

    fn advance(&mut self) {
        if self.position < self.input.len() {
            self.position += 1;
            self.column += 1;
        }
    }

    fn read_number(&mut self) -> i64 {
        let start_pos = self.position;

        while let Some(ch) = self.current_char() {
            if ch.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        let number_str: String = self.input[start_pos..self.position].iter().collect();
        number_str.parse().unwrap_or(0)
    }

    fn read_string(&mut self) -> String {
        self.advance(); // Skip opening quote
        let start_pos = self.position;

        while let Some(ch) = self.current_char() {
            if ch == '"' {
                break;
            } else if ch == '\n' {
                self.line += 1;
                self.column = 1;
            }
            self.advance();
        }

        let string_content: String = self.input[start_pos..self.position].iter().collect();

        if self.current_char() == Some('"') {
            self.advance(); // Skip closing quote
        }

        string_content
    }

    fn read_identifier(&mut self) -> String {
        let start_pos = self.position;

        while let Some(ch) = self.current_char() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        self.input[start_pos..self.position].iter().collect()
    }
}

#[cfg(test)]
mod tests;
