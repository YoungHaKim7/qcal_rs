//! # Expression Parser
//!
//! This module provides recursive descent parsing of token streams into an
//! Abstract Syntax Tree (AST) for evaluation.
//!
//! ## Parsing Algorithm
//!
//! Uses **recursive descent parsing** with operator precedence:
//!
//! ### Grammar (Simplified)
//! ```text
//! assignment  → expr ('=' IDENTIFIER expr)?
//! expr        → or_expr
//! or_expr     → and_expr ('or' and_expr)*
//! and_expr    → shift_expr ('and' shift_expr)*
//! shift_expr  → term (('<<' | '>>') term)*
//! term        → factor (('+' | '-') factor)*
//! factor      → power (('*' | '/') power)*
//! power       → unary ('^' unary)?
//! unary       → ('-')* primary
//! primary     → NUMBER | IDENTIFIER | '(' expr ')' | IDENTIFIER '(' args? ')'
//! ```
//!
//! ### Operator Precedence (Highest to Lowest)
//! 1. Function calls and grouping
//! 2. Exponentiation (^) - Right associative
//! 3. Unary minus
//! 4. Multiplication/Division (*, /)
//! 5. Addition/Subtraction (+, -)
//! 6. Bitwise shifts (<<, >>)
//! 7. Bitwise AND (&)
//! 8. Bitwise OR (|)
//! 9. Assignment (=)
//!
//! ## Examples
//! ```text
//! Input: "2 + 3 * x"
//! AST: Binary(Add, Number(2), Binary(Mul, Number(3), Variable("x")))
//!
//! Input: "sin(pi/2)"
//! AST: Call("sin", [Binary(Div, Variable("pi"), Number(2))])
//! ```

use super::ast::*;
use super::lexer::Token;

/// # Recursive Descent Parser
///
/// Parses tokens into an Abstract Syntax Tree (AST) using operator precedence.
pub struct Parser {
    /// Token stream being parsed
    tokens: Vec<Token>,
    /// Current position in token stream
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expr, String> {
        let expr = self.parse_expr()?;

        if self.match_token(&[Token::Assign]) {
            if let Expr::Variable(name) = expr {
                let value = self.parse_expr()?;
                return Ok(Expr::Assign {
                    name,
                    expr: Box::new(value),
                });
            }
            return Err("Invalid assignment".into());
        }

        Ok(expr)
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut node = self.parse_and()?;

        while self.match_token(&[Token::Or]) {
            let right = self.parse_and()?;
            node = Expr::Binary {
                left: Box::new(node),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }

        Ok(node)
    }

    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut node = self.parse_shift()?;

        while self.match_token(&[Token::And]) {
            let right = self.parse_shift()?;
            node = Expr::Binary {
                left: Box::new(node),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }

        Ok(node)
    }

    fn parse_shift(&mut self) -> Result<Expr, String> {
        let mut node = self.parse_term()?;

        while self.match_token(&[Token::Shl, Token::Shr]) {
            let op = match self.previous() {
                Token::Shl => BinaryOp::Shl,
                Token::Shr => BinaryOp::Shr,
                _ => unreachable!(),
            };

            let right = self.parse_term()?;

            node = Expr::Binary {
                left: Box::new(node),
                op,
                right: Box::new(right),
            };
        }

        Ok(node)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut node = self.parse_factor()?;

        while self.match_token(&[Token::Plus, Token::Minus]) {
            let op = match self.previous() {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            };

            let right = self.parse_factor()?;

            node = Expr::Binary {
                left: Box::new(node),
                op,
                right: Box::new(right),
            };
        }

        Ok(node)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        let mut node = self.parse_power()?;

        while self.match_token(&[Token::Mul, Token::Div]) {
            let op = match self.previous() {
                Token::Mul => BinaryOp::Mul,
                Token::Div => BinaryOp::Div,
                _ => unreachable!(),
            };

            let right = self.parse_power()?;

            node = Expr::Binary {
                left: Box::new(node),
                op,
                right: Box::new(right),
            };
        }

        Ok(node)
    }

    fn parse_power(&mut self) -> Result<Expr, String> {
        let mut node = self.parse_unary()?;

        while self.match_token(&[Token::Pow]) {
            let right = self.parse_unary()?;

            node = Expr::Binary {
                left: Box::new(node),
                op: BinaryOp::Pow,
                right: Box::new(right),
            };
        }

        Ok(node)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if self.match_token(&[Token::Minus]) {
            let expr = self.parse_unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::Neg,
                expr: Box::new(expr),
            });
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Token::Number(n) => Ok(Expr::Number(n)),

            Token::Ident(name) => {
                if self.match_token(&[Token::LParen]) {
                    let mut args = Vec::new();

                    if !self.check(&Token::RParen) {
                        loop {
                            args.push(self.parse_expr()?);
                            if !self.match_token(&[Token::Comma]) {
                                break;
                            }
                        }
                    }

                    self.consume(Token::RParen)?;
                    Ok(Expr::Call { name, args })
                } else {
                    Ok(Expr::Variable(name))
                }
            }

            Token::LParen => {
                let expr = self.parse_expr()?;
                self.consume(Token::RParen)?;
                Ok(expr)
            }

            _ => Err("Unexpected token".into()),
        }
    }

    fn match_token(&mut self, types: &[Token]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, t: &Token) -> bool {
        if self.pos >= self.tokens.len() {
            return false;
        }
        std::mem::discriminant(&self.tokens[self.pos]) == std::mem::discriminant(t)
    }

    fn advance(&mut self) -> Token {
        let tok = self.tokens[self.pos].clone();
        self.pos += 1;
        tok
    }

    fn previous(&self) -> Token {
        self.tokens[self.pos - 1].clone()
    }

    fn consume(&mut self, t: Token) -> Result<(), String> {
        if self.check(&t) {
            self.advance();
            Ok(())
        } else {
            Err("Expected token".into())
        }
    }
}
