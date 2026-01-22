/// Lexer for FORM language
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Number(f64),
    Identifier(String),
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Power,
    Equals,
    
    // Delimiters
    LParen,
    RParen,
    LBracket,
    RBracket,
    Comma,
    Semicolon,
    
    // Keywords
    Symbols,
    Expression,
    Local,
    Id,
    Print,
    Sort,
    
    // Special
    Eof,
    Newline,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "Number({})", n),
            Token::Identifier(s) => write!(f, "Identifier({})", s),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Power => write!(f, "^"),
            Token::Equals => write!(f, "="),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::Symbols => write!(f, "Symbols"),
            Token::Expression => write!(f, "Expression"),
            Token::Local => write!(f, "Local"),
            Token::Id => write!(f, "id"),
            Token::Print => write!(f, "Print"),
            Token::Sort => write!(f, ".sort"),
            Token::Eof => write!(f, "EOF"),
            Token::Newline => write!(f, "NEWLINE"),
        }
    }
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = if chars.is_empty() { None } else { Some(chars[0]) };
        Lexer {
            input: chars,
            position: 0,
            current_char,
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        if self.position >= self.input.len() {
            self.current_char = None;
        } else {
            self.current_char = Some(self.input[self.position]);
        }
    }

    fn peek(&self) -> Option<char> {
        if self.position + 1 >= self.input.len() {
            None
        } else {
            Some(self.input[self.position + 1])
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        // Skip * comment (only at start of line or after explicit newline)
        if self.current_char == Some('*') {
            while let Some(ch) = self.current_char {
                if ch == '\n' {
                    break;
                }
                self.advance();
            }
        }
    }

    fn is_comment_start(&self) -> bool {
        // In FORM, * at the beginning of a line is a comment
        // For now, we'll be conservative and only treat * followed by space at position 0 as comment
        self.position == 0 && self.current_char == Some('*') && self.peek() == Some(' ')
    }

    fn read_number(&mut self) -> f64 {
        let mut num_str = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() || ch == '.' {
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        num_str.parse().unwrap_or(0.0)
    }

    fn read_identifier(&mut self) -> String {
        let mut id = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                id.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        id
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if let Some(ch) = self.current_char {
            // Check for comments (only at start of input)
            if self.is_comment_start() {
                self.skip_comment();
                return self.next_token();
            }

            // Check for .sort
            if ch == '.' {
                self.advance();
                let id = self.read_identifier();
                if id == "sort" {
                    return Token::Sort;
                }
                // Otherwise, just skip the dot
                return self.next_token();
            }

            // Numbers
            if ch.is_ascii_digit() {
                let num = self.read_number();
                return Token::Number(num);
            }

            // Identifiers and keywords
            if ch.is_alphabetic() || ch == '_' {
                let id = self.read_identifier();
                return match id.as_str() {
                    "Symbols" => Token::Symbols,
                    "Expression" => Token::Expression,
                    "Local" => Token::Local,
                    "id" => Token::Id,
                    "Print" => Token::Print,
                    _ => Token::Identifier(id),
                };
            }

            // Operators and delimiters
            let token = match ch {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Star,
                '/' => Token::Slash,
                '^' => Token::Power,
                '=' => Token::Equals,
                '(' => Token::LParen,
                ')' => Token::RParen,
                '[' => Token::LBracket,
                ']' => Token::RBracket,
                ',' => Token::Comma,
                ';' => Token::Semicolon,
                '\n' => Token::Newline,
                _ => {
                    self.advance();
                    return self.next_token();
                }
            };

            self.advance();
            token
        } else {
            Token::Eof
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            if token == Token::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_number() {
        let mut lexer = Lexer::new("42");
        assert_eq!(lexer.next_token(), Token::Number(42.0));
    }

    #[test]
    fn test_tokenize_identifier() {
        let mut lexer = Lexer::new("x");
        assert_eq!(lexer.next_token(), Token::Identifier("x".to_string()));
    }

    #[test]
    fn test_tokenize_operators() {
        let mut lexer = Lexer::new("+ - * / ^");
        assert_eq!(lexer.next_token(), Token::Plus);
        assert_eq!(lexer.next_token(), Token::Minus);
        assert_eq!(lexer.next_token(), Token::Star);
        assert_eq!(lexer.next_token(), Token::Slash);
        assert_eq!(lexer.next_token(), Token::Power);
    }

    #[test]
    fn test_tokenize_expression() {
        let mut lexer = Lexer::new("(x + 1) * 2");
        let tokens = lexer.tokenize();
        assert_eq!(tokens.len(), 8); // ( x + 1 ) * 2 EOF
    }

    #[test]
    fn test_tokenize_keywords() {
        let mut lexer = Lexer::new("Symbols x");
        assert_eq!(lexer.next_token(), Token::Symbols);
        assert_eq!(lexer.next_token(), Token::Identifier("x".to_string()));
    }
}
