//! # tcal_rs - Number Theory & Calculator Library
//!
//! A comprehensive Rust library for number theory operations and mathematical
//! calculations, ported from libqalculate.
//!
//! ## Overview
//!
//! This library provides:
//!
//! ### Number Theory Operations
//! - **GCD/LCM**: Greatest common divisor and least common multiple
//! - **Modular Arithmetic**: Remainder, modulo, modular exponentiation
//! - **Prime Numbers**: Primality testing, generation, counting
//! - **Euler's Totient**: φ(n) function implementation
//! - **Bernoulli Numbers**: Rational sequence computation
//! - **Rounding**: Nine standard rounding modes
//! - **Parity**: Even/odd checking
//!
//! ### Calculator Engine
//! - **Expression Evaluation**: Parse and compute mathematical expressions
//! - **Built-in Functions**: Trigonometry, logarithms, roots
//! - **Variable Storage**: Assign and use variables
//! - **Multiple Number Bases**: Hex, binary, octal support
//!
//! ## Quick Start
//!
//! ### Number Theory
//!
//! ```rust
//! use tcal_rs::*;
//!
//! // GCD and LCM
//! assert_eq!(gcd(48, 18), 6);
//! assert_eq!(lcm(21, 6), 42);
//!
//! // Modular arithmetic
//! assert_eq!(powmod(4, 13, 497), 445);
//!
//! // Prime operations
//! assert!(is_prime(7919));  // 1000th prime
//! assert_eq!(next_prime(100), 101);
//!
//! // Euler's totient
//! assert_eq!(totient(30), 8);
//!
//! // Rounding
//! use tcal_rs::RoundingMode;
//! assert_eq!(round(5, 2, RoundingMode::HalfToEven), 2);
//! ```
//!
//! ### Calculator
//!
//! ```rust
//! use tcal_rs::calculator::engine::Engine;
//!
//! let mut engine = Engine::new();
//!
//! // Basic arithmetic
//! assert_eq!(engine.eval("2 + 2").unwrap(), "4");
//!
//! // Trigonometry
//! assert_eq!(engine.eval("sin(pi/2)").unwrap(), "1");
//!
//! // Number theory
//! assert_eq!(engine.eval("totient(30)").unwrap(), "8");
//!
//! // Variables
//! engine.eval("x = 5").unwrap();
//! assert_eq!(engine.eval("x * 2 + 3").unwrap(), "13");
//! ```
//!
//! ## Module Organization
//!
//! ```text
//! tcal_rs/
//! ├── number_theory/    # Core number theory functions
//! │   ├── parity.rs    # Even/odd checking
//! │   ├── primes.rs    # Prime operations
//! │   ├── rounding.rs  # Rounding modes
//! │   ├── totient.rs   # Euler's totient
//! │   └── traits.rs    # Extension traits
//! ├── calculator/      # Expression calculator
//! │   ├── lexer.rs     # Tokenization
//! │   ├── parser.rs    # AST generation
//! │   ├── evaluator.rs # Expression evaluation
//! │   └── engine.rs    # Main calculator engine
//! └── fprice.rs        # Price formatting
//! ```

pub mod number_theory;

// Re-export commonly used functions for convenience
pub use number_theory::parity::{is_even, is_odd};
pub use number_theory::primes::{
    bernoulli, is_prime, next_prime, nth_prime, prev_prime, prime_count, primes_up_to,
};
pub use number_theory::rounding::{
    RoundingMode, abs, abs_integer, ceil, floor, round, signum, signum_integer, trunc,
};
pub use number_theory::totient::totient;
pub use number_theory::{denominator, frac, gcd, lcm, modulo, numerator, powmod, rem};

// Re-export traits
pub use number_theory::traits::{Divisors, Gcd, Lcm};

pub mod calculator;
pub mod fprice;
pub mod readline;
pub mod save_history;
pub mod unicode;
