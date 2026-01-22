/// Parser for FORM language
use crate::ast::*;
use crate::lexer::{Lexer, Token};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    current_token: Token,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let current_token = if tokens.is_empty() {
            Token::Eof
        } else {
            tokens[0].clone()
        };
        Parser {
            tokens,
            position: 0,
            current_token,
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        if self.position >= self.tokens.len() {
            self.current_token = Token::Eof;
        } else {
            self.current_token = self.tokens[self.position].clone();
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if self.current_token == expected {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", expected, self.current_token))
        }
    }

    pub fn parse_statement(&mut self) -> Result<Statement, String> {
        // Skip newlines
        while self.current_token == Token::Newline {
            self.advance();
        }

        match &self.current_token {
            Token::Symbols => self.parse_symbols_decl(),
            Token::Expression => self.parse_expression_decl(),
            Token::Local => self.parse_local_decl(),
            Token::Id => self.parse_id_rule(),
            Token::Print => self.parse_print(),
            Token::Sort => {
                self.advance();
                Ok(Statement::Sort)
            }
            Token::Eof => Err("End of input".to_string()),
            _ => {
                let expr = self.parse_expression()?;
                Ok(Statement::EvalExpr(expr))
            }
        }
    }

    fn parse_symbols_decl(&mut self) -> Result<Statement, String> {
        self.expect(Token::Symbols)?;
        let mut symbols = Vec::new();

        loop {
            if let Token::Identifier(name) = &self.current_token {
                symbols.push(name.clone());
                self.advance();

                if self.current_token == Token::Comma {
                    self.advance();
                } else {
                    break;
                }
            } else {
                return Err(format!("Expected identifier, got {:?}", self.current_token));
            }
        }

        // Skip optional semicolon
        if self.current_token == Token::Semicolon {
            self.advance();
        }

        Ok(Statement::SymbolsDecl(symbols))
    }

    fn parse_expression_decl(&mut self) -> Result<Statement, String> {
        self.expect(Token::Expression)?;

        if let Token::Identifier(name) = &self.current_token {
            let name = name.clone();
            self.advance();
            self.expect(Token::Equals)?;
            let expr = self.parse_expression()?;

            // Skip optional semicolon
            if self.current_token == Token::Semicolon {
                self.advance();
            }

            Ok(Statement::ExpressionDecl { name, expr })
        } else {
            Err(format!("Expected identifier after Expression, got {:?}", self.current_token))
        }
    }

    fn parse_local_decl(&mut self) -> Result<Statement, String> {
        self.expect(Token::Local)?;

        if let Token::Identifier(name) = &self.current_token {
            let name = name.clone();
            self.advance();
            self.expect(Token::Equals)?;
            let expr = self.parse_expression()?;

            // Skip optional semicolon
            if self.current_token == Token::Semicolon {
                self.advance();
            }

            Ok(Statement::LocalDecl { name, expr })
        } else {
            Err(format!("Expected identifier after Local, got {:?}", self.current_token))
        }
    }

    fn parse_id_rule(&mut self) -> Result<Statement, String> {
        self.expect(Token::Id)?;
        let pattern = self.parse_expression()?;
        self.expect(Token::Equals)?;
        let replacement = self.parse_expression()?;

        // Skip optional semicolon
        if self.current_token == Token::Semicolon {
            self.advance();
        }

        Ok(Statement::IdRule { pattern, replacement })
    }

    fn parse_print(&mut self) -> Result<Statement, String> {
        self.expect(Token::Print)?;

        if let Token::Identifier(name) = &self.current_token {
            let name = name.clone();
            self.advance();

            // Skip optional semicolon
            if self.current_token == Token::Semicolon {
                self.advance();
            }

            Ok(Statement::Print(name))
        } else {
            Err(format!("Expected identifier after Print, got {:?}", self.current_token))
        }
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_additive()
    }

    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplicative()?;

        loop {
            match &self.current_token {
                Token::Plus => {
                    self.advance();
                    let right = self.parse_multiplicative()?;
                    left = Expr::BinOp {
                        op: BinOpKind::Add,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                Token::Minus => {
                    self.advance();
                    let right = self.parse_multiplicative()?;
                    left = Expr::BinOp {
                        op: BinOpKind::Sub,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_power()?;

        loop {
            match &self.current_token {
                Token::Star => {
                    self.advance();
                    let right = self.parse_power()?;
                    left = Expr::BinOp {
                        op: BinOpKind::Mul,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                Token::Slash => {
                    self.advance();
                    let right = self.parse_power()?;
                    left = Expr::BinOp {
                        op: BinOpKind::Div,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_power(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;

        if self.current_token == Token::Power {
            self.advance();
            let right = self.parse_power()?; // Right associative
            left = Expr::BinOp {
                op: BinOpKind::Pow,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        match &self.current_token {
            Token::Minus => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::UnOp {
                    op: UnOpKind::Neg,
                    operand: Box::new(operand),
                })
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match &self.current_token {
            Token::Number(n) => {
                let num = *n;
                self.advance();
                Ok(Expr::Number(num))
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();

                // Check for function call
                if self.current_token == Token::LParen {
                    self.advance();
                    let mut args = Vec::new();

                    if self.current_token != Token::RParen {
                        loop {
                            args.push(self.parse_expression()?);
                            if self.current_token == Token::Comma {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }

                    self.expect(Token::RParen)?;
                    Ok(Expr::FunctionCall { name, args })
                } else {
                    Ok(Expr::Symbol(name))
                }
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            _ => Err(format!("Unexpected token: {:?}", self.current_token)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        let mut parser = Parser::new("42");
        let stmt = parser.parse_statement().unwrap();
        match stmt {
            Statement::EvalExpr(Expr::Number(n)) => assert_eq!(n, 42.0),
            _ => panic!("Expected number expression"),
        }
    }

    #[test]
    fn test_parse_symbol() {
        let mut parser = Parser::new("x");
        let stmt = parser.parse_statement().unwrap();
        match stmt {
            Statement::EvalExpr(Expr::Symbol(s)) => assert_eq!(s, "x"),
            _ => panic!("Expected symbol expression"),
        }
    }

    #[test]
    fn test_parse_addition() {
        let mut parser = Parser::new("1 + 2");
        let stmt = parser.parse_statement().unwrap();
        match stmt {
            Statement::EvalExpr(Expr::BinOp { op, .. }) => {
                assert_eq!(op, BinOpKind::Add);
            }
            _ => panic!("Expected binary operation"),
        }
    }

    #[test]
    fn test_parse_symbols_decl() {
        let mut parser = Parser::new("Symbols x, y, z");
        let stmt = parser.parse_statement().unwrap();
        match stmt {
            Statement::SymbolsDecl(syms) => {
                assert_eq!(syms, vec!["x", "y", "z"]);
            }
            _ => panic!("Expected symbols declaration"),
        }
    }
}
