//! # Number Theory Functions Module
//!
//! This module provides comprehensive number theory operations and mathematical functions.
//!
//! ## Mathematical Background
//!
//! ### Greatest Common Divisor (GCD)
//! The GCD of two integers is the largest positive integer that divides both without remainder.
//! **Formula:** `gcd(a, b) = gcd(b, a mod b)` with base case `gcd(a, 0) = |a|`
//!
//! ### Least Common Multiple (LCM)
//! The LCM is the smallest positive integer divisible by both numbers.
//! **Formula:** `lcm(a, b) = |a × b| / gcd(a, b)`
//!
//! ### Modular Exponentiation
//! Computes `(base^exp) mod modulus` efficiently using binary exponentiation.
//! **Algorithm:** Square-and-multiply with O(log exp) complexity
//!
//! ## Module Contents
//!
//! - **Parity checks** (`is_even`, `is_odd`): Test if numbers are divisible by 2
//! - **Absolute value** (`abs`, `abs_integer`): Distance from zero
//! - **GCD/LCM**: Fundamental arithmetic operations using Euclidean algorithm
//! - **Divisors**: Find all positive divisors of an integer
//! - **Prime operations**: Primality testing, prime generation, counting
//! - **Rounding**: Multiple rounding modes (floor, ceil, trunc, banker's)
//! - **Modular arithmetic**: Remainder, modulo, modular exponentiation
//! - **Euler's totient**: Count integers coprime to n (φ function)
//! - **Bernoulli numbers**: Sequence appearing in Taylor series
//!
//! ## Usage Examples
//!
//! ```rust
//! use tcal_rs::*;
//!
//! // Basic arithmetic
//! let g = gcd(48, 18);   // 6
//! let l = lcm(21, 6);    // 42
//!
//! // Modular arithmetic
//! let r = powmod(4, 13, 497);  // 445: 4^13 mod 497
//! let m = modulo(-7, 3);      // 2: always non-negative
//!
//! // Number theory
//! let t = totient(30);   // 8: φ(30) = 8
//! let p = is_prime(7919); // true: 1000th prime
//! ```

// Re-exports
pub use self::parity::{is_even, is_odd};
pub use self::primes::{
    bernoulli, is_prime, next_prime, nth_prime, prev_prime, prime_count, primes_up_to,
};
pub use self::rounding::{
    RoundingMode, abs, abs_integer, ceil, floor, round, signum, signum_integer, trunc,
};
pub use self::totient::totient;
pub use self::traits::{Divisors, Gcd, Lcm};

pub mod parity;
pub mod primes;
pub mod rounding;
pub mod totient;
pub mod traits;

/// # Greatest Common Divisor (Euclidean Algorithm)
///
/// Computes the greatest common divisor of two integers using the Euclidean algorithm.
///
/// ## Mathematical Definition
/// The GCD of two integers a and b, denoted gcd(a,b), is the largest positive
/// integer that divides both a and b without leaving a remainder.
///
/// ## Algorithm
/// The Euclidean algorithm is based on the principle that:
/// ```text
/// gcd(a, b) = gcd(b, a mod b)
/// gcd(a, 0) = |a|
/// ```
///
/// ## Time Complexity
/// - O(log(min(a, b))) - logarithmic in the smaller input
///
/// ## Examples
/// ```
/// use tcal_rs::gcd;
///
/// assert_eq!(gcd(48, 18), 6);
/// assert_eq!(gcd(17, 23), 1);  // Coprime numbers
/// assert_eq!(gcd(0, 5), 5);
/// assert_eq!(gcd(48, 0), 48);
/// ```
///
/// ## Properties
/// - **Commutative**: gcd(a, b) = gcd(b, a)
/// - **Associative**: gcd(a, gcd(b, c)) = gcd(gcd(a, b), c)
/// - **Distribution**: gcd(a, lcm(b, c)) = lcm(gcd(a, b), gcd(a, c))
pub fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a.abs();
    let mut b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// Computes the greatest common divisor of two i128 integers.
pub fn gcd_i128(a: i128, b: i128) -> i128 {
    let mut a = a.abs();
    let mut b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// # Least Common Multiple
///
/// Computes the least common multiple of two integers.
///
/// ## Mathematical Definition
/// The LCM of two integers a and b, denoted lcm(a,b), is the smallest positive
/// integer that is divisible by both a and b.
///
/// ## Formula
/// ```text
/// lcm(a, b) = |a × b| / gcd(a, b)
/// lcm(a, 0) = 0
/// ```
///
/// ## Time Complexity
/// - O(log(min(a, b))) - dominated by GCD computation
///
/// ## Examples
/// ```
/// use tcal_rs::lcm;
///
/// assert_eq!(lcm(21, 6), 42);
/// assert_eq!(lcm(5, 7), 35);
/// assert_eq!(lcm(4, 6), 12);
/// assert_eq!(lcm(0, 5), 0);
/// ```
///
/// ## Properties
/// - **Commutative**: lcm(a, b) = lcm(b, a)
/// - **Associative**: lcm(a, lcm(b, c)) = lcm(lcm(a, b), c)
/// - **Identity**: lcm(a, 1) = |a|
pub fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a.abs() / gcd(a, b)) * b.abs()
}

/// Computes the least common multiple of two i128 integers.
pub fn lcm_i128(a: i128, b: i128) -> i128 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a.abs() / gcd_i128(a, b)) * b.abs()
}

/// Computes the fractional part of a number.
///
/// Returns the fractional part of a rational number, i.e., the number
/// minus its integer part.
///
/// # Arguments
///
/// * `numerator` - Numerator of the rational number
/// * `denominator` - Denominator of the rational number
///
/// # Examples
///
/// ```
/// use tcal_rs::frac;
///
/// let (num, den) = frac(7, 4);
/// assert_eq!(num, 3);
/// assert_eq!(den, 4);
/// ```
pub fn frac(numerator: i64, denominator: i64) -> (i64, i64) {
    let int_part = numerator / denominator;
    let frac_num = numerator - int_part * denominator;
    (frac_num, denominator)
}

/// Computes the remainder of Euclidean division (rem).
///
/// This differs from `mod` in how it handles negative numbers.
/// The remainder has the same sign as the dividend.
///
/// # Arguments
///
/// * `a` - Dividend
/// * `b` - Divisor (must be non-zero)
///
/// # Returns
///
/// The remainder of a divided by b
///
/// # Examples
///
/// ```
/// use tcal_rs::rem;
///
/// assert_eq!(rem(7, 3), 1);
/// assert_eq!(rem(-7, 3), -1);  // Different from mod!
/// ```
pub fn rem(a: i64, b: i64) -> i64 {
    a % b
}

/// Computes the modulo operation.
///
/// The modulo operation always returns a non-negative result,
/// unlike the remainder operation.
///
/// # Arguments
///
/// * `a` - Dividend
/// * `b` - Divisor (must be positive)
///
/// # Returns
///
/// The modulo of a and b
///
/// # Examples
///
/// ```
/// use tcal_rs::modulo;
///
/// assert_eq!(modulo(7, 3), 1);
/// assert_eq!(modulo(-7, 3), 2);  // Always non-negative
/// ```
pub fn modulo(a: i64, b: i64) -> i64 {
    ((a % b) + b) % b
}

/// # Modular Exponentiation (Square-and-Multiply Algorithm)
///
/// Computes `(base^exp) mod modulus` efficiently using binary exponentiation.
///
/// ## Mathematical Background
/// Modular exponentiation is a fundamental operation in:
/// - **Cryptography**: RSA encryption/decryption, Diffie-Hellman key exchange
/// - **Computer Science**: Hash functions, pseudo-random number generators
/// - **Number Theory**: Primality testing, factorization
///
/// ## Algorithm (Binary Exponentiation)
/// Also known as the "square-and-multiply" method:
/// ```text
/// result = 1
/// while exp > 0:
///     if exp is odd:
///         result = (result × base) mod modulus
///     exp = exp / 2
///     base = (base × base) mod modulus
/// return result
/// ```
///
/// ## Time Complexity
/// - O(log exp) - one iteration per bit of the exponent
///
/// ## Space Complexity
/// - O(1) - constant space
///
/// ## Examples
/// ```
/// use tcal_rs::powmod;
///
/// // 4^13 mod 497 = 445
/// assert_eq!(powmod(4, 13, 497), 445);
///
/// // 2^10 mod 1000 = 24
/// assert_eq!(powmod(2, 10, 1000), 24);
///
/// // Fermat's little theorem: a^(p-1) ≡ 1 (mod p) for prime p
/// // 3^6 mod 7 = 1 (since 7 is prime)
/// assert_eq!(powmod(3, 6, 7), 1);
/// ```
///
/// # Arguments
/// * `base` - The base (non-negative integer)
/// * `exp` - The exponent (must be non-negative)
/// * `modulus` - The modulus (must be positive)
///
/// # Returns
/// `(base^exp) mod modulus`
///
/// # Panics
/// Does not panic, but returns 0 immediately if modulus is 1.
pub fn powmod(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result.wrapping_mul(base) % modulus;
        }
        exp /= 2;
        base = base.wrapping_mul(base) % modulus;
    }
    result
}

/// Computes modular exponentiation for signed integers.
pub fn powmod_i64(base: i64, exp: u64, modulus: i64) -> i64 {
    if modulus <= 0 {
        panic!("modulus must be positive");
    }
    let base_abs = base.unsigned_abs();
    let result = powmod(base_abs, exp, modulus as u64);
    if base < 0 && exp % 2 == 1 {
        -(modulus - (result as i64))
    } else {
        result as i64
    }
}

/// Returns the numerator of a rational number.
///
/// # Examples
///
/// ```
/// use tcal_rs::numerator;
///
/// assert_eq!(numerator(7, 3), 7);
/// assert_eq!(numerator(-7, 3), -7);
/// ```
pub const fn numerator(numerator: i64, _denominator: i64) -> i64 {
    numerator
}

/// Returns the denominator of a rational number.
///
/// # Examples
///
/// ```
/// use tcal_rs::denominator;
///
/// assert_eq!(denominator(7, 3), 3);
/// assert_eq!(denominator(-7, 3), 3);
/// ```
pub const fn denominator(_numerator: i64, denominator: i64) -> i64 {
    denominator
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(17, 23), 1);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(5, 0), 5);
        assert_eq!(gcd(48, 0), 48);
        assert_eq!(gcd(0, 0), 0);
    }

    #[test]
    fn test_gcd_i128() {
        assert_eq!(gcd_i128(48, 18), 6);
        assert_eq!(gcd_i128(17, 23), 1);
        assert_eq!(gcd_i128(0, 5), 5);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(21, 6), 42);
        assert_eq!(lcm(5, 7), 35);
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(0, 5), 0);
    }

    #[test]
    fn test_lcm_i128() {
        assert_eq!(lcm_i128(21, 6), 42);
        assert_eq!(lcm_i128(5, 7), 35);
        assert_eq!(lcm_i128(0, 5), 0);
    }

    #[test]
    fn test_frac() {
        assert_eq!(frac(7, 4), (3, 4));
        assert_eq!(frac(-7, 4), (-3, 4));
        assert_eq!(frac(8, 4), (0, 4));
    }

    #[test]
    fn test_rem() {
        assert_eq!(rem(7, 3), 1);
        assert_eq!(rem(-7, 3), -1);
        assert_eq!(rem(7, -3), 1);
    }

    #[test]
    fn test_modulo() {
        assert_eq!(modulo(7, 3), 1);
        assert_eq!(modulo(-7, 3), 2);
        assert_eq!(modulo(7, 5), 2);
    }

    #[test]
    fn test_powmod() {
        assert_eq!(powmod(4, 13, 497), 445);
        assert_eq!(powmod(2, 10, 1000), 24);
        assert_eq!(powmod_i64(4, 13, 497), 445);
        assert_eq!(powmod_i64(2, 10, 1000), 24);
    }

    #[test]
    fn test_numerator_denominator() {
        assert_eq!(numerator(7, 3), 7);
        assert_eq!(denominator(7, 3), 3);
        assert_eq!(numerator(-7, 3), -7);
        assert_eq!(denominator(-7, 3), 3);
    }
}
