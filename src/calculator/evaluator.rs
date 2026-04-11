//! # Expression Evaluator
//!
//! This module provides evaluation of Abstract Syntax Trees (AST) into numeric results.
//!
//! ## Features
//!
//! ### Built-in Constants
//! | Constant | Value | Description |
//! |----------|-------|-------------|
//! | `pi` | 3.14159... | Ratio of circle circumference to diameter |
//! | `e` | 2.71828... | Euler's number, base of natural logarithm |
//! | `res` | Previous result | Holds the last computed value |
//!
//! ### Mathematical Functions
//!
//! #### Trigonometric Functions
//! - `sin(x)` - Sine
//! - `cos(x)` - Cosine
//! - `tan(x)` - Tangent
//! - `asin(x)` - Arcsine (inverse sine)
//! - `acos(x)` - Arccosine (inverse cosine)
//! - `atan(x)` - Arctangent (inverse tangent)
//!
//! #### Root Functions
//! - `sqrt(x)` - Square root
//! - `cbrt(x)` - Cube root
//! - `abs(x)` - Absolute value
//!
//! #### Logarithmic Functions
//! - `ln(x)` or `log(x)` - Natural logarithm (base e)
//! - `log10(x)` - Base-10 logarithm
//! - `exp(x)` - Exponential function (e^x)
//!
//! #### Number Theory Functions
//! - `totient(n)` - Euler's totient function φ(n)
//! - `gcd(a, b)` - Greatest common divisor
//! - `lcm(a, b)` - Least common multiple
//!
//! ### Variable Assignment
//! ```text
//! x = 5          # Assign 5 to x
//! y = x * 2      # y = 10
//! ```
//!
//! ## Evaluation Algorithm
//!
//! 1. **Traverse AST recursively** in post-order (children before parent)
//! 2. **Evaluate each node type**:
//!    - Number: Return the value
//!    - Variable: Lookup in symbol table or return 0
//!    - Assignment: Evaluate RHS, store in symbol table
//!    - Unary/Binary: Evaluate operands, apply operator
//!    - Function Call: Evaluate arguments, call built-in function
//!
//! ## Examples
//! ```text
//! Input: "sin(pi/2)"
//! Output: 1.0
//!
//! Input: "totient(30)"
//! Output: 8.0
//!
//! Input: "x = 5; x * 2 + 3"
//! Output: 13.0
//! ```

use super::ast::*;
use crate::{totient, gcd, lcm};
use std::collections::HashMap;

/// # Expression Evaluator
///
/// Evaluates AST nodes into numeric results with support for variables
/// and built-in functions.
pub struct Evaluator {
    /// Symbol table storing variable assignments
    vars: HashMap<String, f64>,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn eval(&mut self, expr: &Expr) -> f64 {
        match expr {
            Expr::Number(n) => *n,

            Expr::Variable(name) => match name.as_str() {
                "pi" => std::f64::consts::PI,
                "e" => std::f64::consts::E,
                _ => *self.vars.get(name).unwrap_or(&0.0),
            },

            Expr::Assign { name, expr } => {
                let val = self.eval(expr);
                self.vars.insert(name.clone(), val);
                val
            }

            Expr::Unary { op, expr } => match op {
                UnaryOp::Neg => -self.eval(expr),
            },

            Expr::Binary { left, op, right } => {
                let l = self.eval(left);
                let r = self.eval(right);

                match op {
                    BinaryOp::Add => l + r,
                    BinaryOp::Sub => l - r,
                    BinaryOp::Mul => l * r,
                    BinaryOp::Div => l / r,
                    BinaryOp::Pow => l.powf(r),

                    BinaryOp::And => (l as i64 & r as i64) as f64,
                    BinaryOp::Or => (l as i64 | r as i64) as f64,
                    BinaryOp::Shl => ((l as i64) << r as i64) as f64,
                    BinaryOp::Shr => (l as i64 >> r as i64) as f64,
                }
            }

            Expr::Call { name, args } => {
                let vals: Vec<f64> = args.iter().map(|a| self.eval(a)).collect();

                match name.as_str() {
                    "sin" => vals[0].sin(),
                    "cos" => vals[0].cos(),
                    "tan" => vals[0].tan(),
                    "asin" => vals[0].asin(),
                    "acos" => vals[0].acos(),
                    "atan" => vals[0].atan(),
                    "sqrt" => vals[0].sqrt(),
                    "cbrt" => vals[0].cbrt(),
                    "abs" => vals[0].abs(),
                    "ln" | "log" => vals[0].ln(),
                    "log10" => vals[0].log10(),
                    "exp" => vals[0].exp(),
                    "totient" => totient(vals[0] as i64) as f64,
                    "gcd" => gcd(vals[0] as i64, vals[1] as i64) as f64,
                    "lcm" => lcm(vals[0] as i64, vals[1] as i64) as f64,
                    _ => panic!("Unknown function"),
                }
            }
        }
    }
}
