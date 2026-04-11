//! tcal_rs - Number theory functions library
//!
//! Rust port of libqalculate number theory module providing various
//! number theory operations including primality testing, GCD/LCM,
//! totient function, and more.

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
