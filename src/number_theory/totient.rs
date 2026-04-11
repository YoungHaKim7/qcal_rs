//! # Euler's Totient Function (φ)
//!
//! This module provides computation of Euler's totient function φ(n), one of the most
//! important functions in number theory.
//!
//! ## Mathematical Definition
//!
//! Euler's totient function φ(n) counts the positive integers up to n that are
//! relatively prime to n (i.e., their greatest common divisor with n is 1).
//!
//! ### Formula using Prime Factorization
//!
//! If n has the prime factorization:
//! ```text
//! n = p₁^k₁ × p₂^k₂ × ... × p_m^k_m
//! ```
//!
//! Then:
//! ```text
//! φ(n) = n × Π(1 - 1/p_i) for all distinct prime factors p_i
//! ```
//!
//! ### Equivalent Forms
//! ```text
//! φ(n) = Π(p_i^k_i - p_i^(k_i-1)) for all prime factors p_i
//! φ(n) = Π(p_i - 1) × p_i^(k_i-1) for all prime factors p_i
//! ```
//!
//! ## Key Properties
//!
//! - **Multiplicative**: If gcd(a, b) = 1, then φ(ab) = φ(a) × φ(b)
//! - **For prime p**: φ(p) = p - 1
//! - **For prime power**: φ(p^k) = p^k - p^(k-1) = p^k × (1 - 1/p)
//! - **For n > 2**: φ(n) is always even
//! - **Sum over divisors**: Σ φ(d) = n for all d dividing n
//!
//! ## Examples
//! ```text
//! φ(1) = 1
//! φ(7) = 6           [7 is prime, so φ(7) = 7-1]
//! φ(9) = 6           [9 = 3², φ(9) = 9(1-1/3) = 6]
//! φ(10) = 4          [Numbers coprime to 10: 1,3,7,9]
//! φ(30) = 8          [30 × (1-1/2)(1-1/3)(1-1/5) = 8]
//! ```
//!
//! ## Applications
//!
//! - **Euler's Theorem**: a^φ(n) ≡ 1 (mod n) for gcd(a, n) = 1
//! - **RSA Encryption**: φ(n) determines the private key
//! - **Cyclic Groups**: φ(n) counts generators of Z_n
//! - **Ramanujan Sum**: Number theory and signal processing

/// # Euler's Totient Function φ(n)
///
/// Computes Euler's totient function φ(n), which counts positive integers
/// up to n that are coprime with n (gcd(k, n) = 1).
///
/// ## Mathematical Formula
///
/// For n with prime factorization n = p₁^k₁ × p₂^k₂ × ... × p_m^k_m:
/// ```text
/// φ(n) = n × Π(1 - 1/p_i) for all distinct prime factors p_i
/// ```
///
/// ## Algorithm
///
/// 1. Start with result = n
/// 2. For each distinct prime factor p of n:
///    - Divide n by p as many times as possible
///    - Apply: result -= result / p (equivalent to result *= (1 - 1/p))
/// 3. Return the final result
///
/// ## Time Complexity
/// - O(√n) - checks all potential prime factors up to √n
///
/// ## Examples
/// ```
/// use tcal_rs::totient;
///
/// // φ(1) = 1 by definition
/// assert_eq!(totient(1), 1);
///
/// // For primes: φ(p) = p - 1
/// assert_eq!(totient(7), 6);   // 7 is prime
/// assert_eq!(totient(11), 10);
///
/// // For prime powers: φ(p^k) = p^k - p^(k-1)
/// assert_eq!(totient(9), 6);   // 9 = 3², φ(9) = 9(1-1/3) = 6
/// assert_eq!(totient(8), 4);   // 8 = 2³, φ(8) = 8(1-1/2) = 4
///
/// // For composite numbers
/// assert_eq!(totient(10), 4);  // φ(10) = 10(1-1/2)(1-1/5) = 4
/// assert_eq!(totient(30), 8);  // φ(30) = 30(1-1/2)(1-1/3)(1-1/5) = 8
///
/// // Multiplicative property: φ(ab) = φ(a)φ(b) for coprime a,b
/// // φ(15) = φ(3)φ(5) = 2 × 4 = 8
/// assert_eq!(totient(15), 8);
/// ```
///
/// # Arguments
/// * `n` - The integer to compute φ for (can be negative, uses absolute value)
///
/// # Returns
/// φ(n), or 0 if n = 0
pub fn totient(n: i64) -> i64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut n_abs = n.abs();
    let mut result = n_abs;
    let original = n_abs;

    // Check divisibility by 2
    if n_abs % 2 == 0 {
        while n_abs % 2 == 0 {
            n_abs /= 2;
        }
        result -= result / 2;
    }

    // Check odd divisors from 3 onwards
    let mut i = 3i64;
    while i * i <= original {
        if n_abs % i == 0 {
            while n_abs % i == 0 {
                n_abs /= i;
            }
            result -= result / i;
        }
        i += 2;
    }

    // If n is still greater than 1, then it's a prime factor
    if n_abs > 1 {
        result -= result / n_abs;
    }

    result
}

/// Computes Euler's totient function φ(n) for i128.
pub fn totient_i128(n: i128) -> i128 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut n_abs = n.abs();
    let mut result = n_abs;
    let original = n_abs;

    // Check divisibility by 2
    if n_abs % 2 == 0 {
        while n_abs % 2 == 0 {
            n_abs /= 2;
        }
        result -= result / 2;
    }

    // Check odd divisors from 3 onwards
    let mut i = 3i128;
    while i * i <= original {
        if n_abs % i == 0 {
            while n_abs % i == 0 {
                n_abs /= i;
            }
            result -= result / i;
        }
        i += 2;
    }

    // If n is still greater than 1, then it's a prime factor
    if n_abs > 1 {
        result -= result / n_abs;
    }

    result
}

/// Computes the totient function using prime factorization.
///
/// This alternative implementation explicitly factors n first.
///
/// # Arguments
///
/// * `n` - The integer to compute φ for
///
/// # Returns
///
/// φ(n), or None if n is zero
pub fn totient_from_factors(n: i64) -> Option<i64> {
    if n == 0 {
        return Some(0);
    }
    if n == 1 {
        return Some(1);
    }

    let n_abs = n.abs();
    let factors = prime_factors(n_abs)?;

    // φ(n) = n × Π(1 - 1/p) for distinct primes p
    let mut result = n_abs;
    for prime in factors {
        result = (result / prime) * (prime - 1);
    }

    Some(result)
}

/// Returns the distinct prime factors of n.
///
/// # Arguments
///
/// * `n` - The integer to factor (must be positive)
///
/// # Returns
///
/// Vector of distinct prime factors, or None for n = 0
fn prime_factors(n: i64) -> Option<Vec<i64>> {
    if n == 0 {
        return None;
    }
    if n == 1 {
        return Some(Vec::new());
    }

    let mut n = n;
    let mut factors = Vec::new();

    // Check divisibility by 2
    while n % 2 == 0 {
        if factors.is_empty() || factors.last() != Some(&2) {
            factors.push(2);
        }
        n /= 2;
    }

    // Check odd divisors from 3 onwards
    let mut i = 3i64;
    while i * i <= n {
        while n % i == 0 {
            if factors.last() != Some(&i) {
                factors.push(i);
            }
            n /= i;
        }
        i += 2;
    }

    // If n is still greater than 1, then it's prime
    if n > 1 {
        factors.push(n);
    }

    Some(factors)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_totient() {
        // φ(1) = 1
        assert_eq!(totient(1), 1);

        // φ(p) = p - 1 for prime p
        assert_eq!(totient(2), 1);
        assert_eq!(totient(3), 2);
        assert_eq!(totient(5), 4);
        assert_eq!(totient(7), 6);
        assert_eq!(totient(11), 10);

        // φ(p^k) = p^k - p^(k-1) = p^k × (1 - 1/p)
        assert_eq!(totient(4), 2); // 4 = 2²
        assert_eq!(totient(8), 4); // 8 = 2³
        assert_eq!(totient(9), 6); // 9 = 3²
        assert_eq!(totient(27), 18); // 27 = 3³

        // φ(ab) = φ(a) × φ(b) for coprime a, b
        assert_eq!(totient(10), 4); // φ(2) × φ(5) = 1 × 4 = 4
        assert_eq!(totient(14), 6); // φ(2) × φ(7) = 1 × 6 = 6
        assert_eq!(totient(15), 8); // φ(3) × φ(5) = 2 × 4 = 8

        // φ(30) = 30 × (1-1/2) × (1-1/3) × (1-1/5) = 30 × 1/2 × 2/3 × 4/5 = 8
        assert_eq!(totient(30), 8);
    }

    #[test]
    fn test_totient_negative() {
        // φ(-n) = φ(n) for n > 0
        assert_eq!(totient(-7), 6);
        assert_eq!(totient(-10), 4);
    }

    #[test]
    fn test_totient_i128() {
        assert_eq!(totient_i128(1), 1);
        assert_eq!(totient_i128(7), 6);
        assert_eq!(totient_i128(30), 8);
        assert_eq!(totient_i128(-7), 6);
    }

    #[test]
    fn test_prime_factors() {
        assert_eq!(prime_factors(1), Some(vec![]));
        assert_eq!(prime_factors(2), Some(vec![2]));
        assert_eq!(prime_factors(12), Some(vec![2, 3]));
        assert_eq!(prime_factors(30), Some(vec![2, 3, 5]));
        assert_eq!(prime_factors(0), None);
    }

    #[test]
    fn test_totient_from_factors() {
        assert_eq!(totient_from_factors(1), Some(1));
        assert_eq!(totient_from_factors(7), Some(6));
        assert_eq!(totient_from_factors(10), Some(4));
        assert_eq!(totient_from_factors(30), Some(8));
    }
}
