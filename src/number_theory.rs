//! Number theory functions module
//!
//! Provides various number theory operations including:
//! - Parity checks (odd/even)
//! - Absolute value
//! - Greatest common divisor and least common multiple
//! - Divisors and factorization
//! - Prime number operations
//! - Rounding functions
//! - Modular arithmetic
//! - Euler's totient function
//! - Bernoulli numbers

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

/// Computes the greatest common divisor of two integers using Euclidean algorithm.
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

/// Computes the least common multiple of two integers.
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

/// Computes modular exponentiation (base^exp mod modulus).
///
/// Uses efficient binary exponentiation algorithm.
///
/// # Arguments
///
/// * `base` - The base
/// * `exp` - The exponent (must be non-negative)
/// * `modulus` - The modulus (must be positive)
///
/// # Returns
///
/// (base^exp) mod modulus
///
/// # Examples
///
/// ```
/// use tcal_rs::powmod;
///
/// let result = powmod(4, 13, 497);
/// assert_eq!(result, 445);
/// ```
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
    let base_abs = base.abs() as u64;
    let result = powmod(base_abs, exp, modulus as u64);
    if base < 0 && exp % 2 == 1 {
        -((modulus as i64) - (result as i64))
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
