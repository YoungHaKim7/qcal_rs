//! # Calculator Engine
//!
//! This module provides the main calculator engine that orchestrates
//! lexing, parsing, preprocessing, and evaluation.
//!
//! ## Pipeline
//!
//! Input String → Preprocessor → Lexer → Parser → Evaluator → Formatter → Output
//!
//! ## Features
//!
//! ### Number Base Support
//! - **Hexadecimal**: `0x1A` → 26
//! - **Binary**: `0b1010` → 10
//! - **Octal**: `0o755` → 493
//!
//! ### Special Substitutions
//! - `res` → Last computed result
//!
//! ### Unicode Conversion
//! - `to unicode "Hello"` → Display Unicode code points

use super::{
    converter::Converter, evaluator::Evaluator, formatter::Formatter, lexer::Lexer, parser::Parser,
};

/// # Calculator Engine
///
/// Main engine orchestrating the complete evaluation pipeline.
pub struct Engine {
    /// Expression evaluator with variable storage
    evaluator: Evaluator,
    /// Last computed result (for 'res' substitution)
    last: Option<f64>,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine {
    /// # Create New Engine
    ///
    /// Creates a new calculator engine with empty variable storage.
    pub fn new() -> Self {
        Self {
            evaluator: Evaluator::new(),
            last: None,
        }
    }

    /// # Evaluate Expression
    ///
    /// Evaluates a mathematical expression string and returns the formatted result.
    ///
    /// ## Processing Steps
    /// 1. Check for Unicode conversion command
    /// 2. Substitute `res` with previous result
    /// 3. Preprocess (convert hex/bin/oct literals)
    /// 4. Tokenize input
    /// 5. Parse tokens into AST
    /// 6. Evaluate AST
    /// 7. Store result for future `res` references
    /// 8. Format and return result
    ///
    /// ## Examples
    /// ```text
    /// engine.eval("2 + 2")        // Ok("4")
    /// engine.eval("0xFF + 1")     // Ok("256")
    /// engine.eval("sin(pi/2)")    // Ok("1")
    /// ```
    pub fn eval(&mut self, input: &str) -> Result<String, String> {
        if input.contains("to unicode") || input.contains("to uni") {
            return Ok(Converter::unicode(input));
        }

        let mut input = input.to_string();

        if let Some(last) = self.last {
            input = input.replace("res", &last.to_string());
        }

        let input = self.preprocess(&input);

        let tokens = Lexer::tokenize(&input)?;
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;

        let val = self.evaluator.eval(&ast);
        self.last = Some(val);

        Ok(Formatter::full(val))
    }

    /// # Preprocess Input
    ///
    /// Converts number literals in different bases to decimal.
    ///
    /// ## Supported Formats
    /// - `0x...` - Hexadecimal (base 16)
    /// - `0b...` - Binary (base 2)
    /// - `0o...` - Octal (base 8)
    ///
    /// ## Examples
    /// ```text
    /// preprocess("0xFF")      // "255"
    /// preprocess("0b1010")    // "10"
    /// preprocess("0o755")     // "493"
    /// ```
    fn preprocess(&self, input: &str) -> String {
        let mut s = input.to_string();

        while let Some(pos) = s.find("0x") {
            let hex: String = s[pos + 2..]
                .chars()
                .take_while(|c| c.is_ascii_hexdigit())
                .collect();

            if !hex.is_empty() {
                let val = i64::from_str_radix(&hex, 16).unwrap();
                s.replace_range(pos..pos + 2 + hex.len(), &val.to_string());
            } else {
                break;
            }
        }

        while let Some(pos) = s.find("0b") {
            let bin: String = s[pos + 2..]
                .chars()
                .take_while(|c| *c == '0' || *c == '1')
                .collect();

            if !bin.is_empty() {
                let val = i64::from_str_radix(&bin, 2).unwrap();
                s.replace_range(pos..pos + 2 + bin.len(), &val.to_string());
            } else {
                break;
            }
        }

        while let Some(pos) = s.find("0o") {
            let oct: String = s[pos + 2..]
                .chars()
                .take_while(|c| *c >= '0' && *c <= '7')
                .collect();

            if !oct.is_empty() {
                let val = i64::from_str_radix(&oct, 8).unwrap();
                s.replace_range(pos..pos + 2 + oct.len(), &val.to_string());
            } else {
                break;
            }
        }

        s
    }
}
