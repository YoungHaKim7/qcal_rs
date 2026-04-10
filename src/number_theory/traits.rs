//! Extension traits for number theory operations
//!
//! Provides trait implementations for easy use of number theory functions.

use crate::number_theory::{gcd, gcd_i128, lcm, lcm_i128};

/// Trait for computing divisors of an integer.
pub trait Divisors {
    /// Returns all positive divisors of the number.
    ///
    /// # Examples
    ///
    /// ```
    /// use tcal_rs::number_theory::traits::Divisors;
    ///
    /// let n = 12i64;
    /// let divisors = n.divisors();
    /// assert_eq!(divisors, vec![1, 2, 3, 4, 6, 12]);
    /// ```
    fn divisors(&self) -> Vec<i64>;
}

/// Trait for computing greatest common divisor.
pub trait Gcd<T> {
    /// Computes the greatest common divisor with another number.
    ///
    /// # Examples
    ///
    /// ```
    /// use tcal_rs::number_theory::traits::Gcd;
    ///
    /// let a = 48i64;
    /// let b = 18i64;
    /// assert_eq!(a.gcd(&b), 6);
    /// ```
    fn gcd(&self, other: &T) -> i64;
}

/// Trait for computing greatest common divisor for i128.
pub trait GcdI128<T> {
    fn gcd_i128(&self, other: &T) -> i128;
}

/// Trait for computing least common multiple.
pub trait Lcm<T> {
    /// Computes the least common multiple with another number.
    ///
    /// # Examples
    ///
    /// ```
    /// use tcal_rs::number_theory::traits::Lcm;
    ///
    /// let a = 21i64;
    /// let b = 6i64;
    /// assert_eq!(a.lcm(&b), 42);
    /// ```
    fn lcm(&self, other: &T) -> i64;
}

/// Trait for computing least common multiple for i128.
pub trait LcmI128<T> {
    fn lcm_i128(&self, other: &T) -> i128;
}

impl Divisors for i64 {
    fn divisors(&self) -> Vec<i64> {
        if *self == 0 {
            return vec![];
        }

        let n = self.abs();
        if n == 1 {
            return vec![1];
        }

        let mut divisors = Vec::new();

        // Efficient divisor enumeration
        let sqrt_n = (n as f64).sqrt() as i64;

        for i in 1..=sqrt_n {
            if n % i == 0 {
                divisors.push(i);
                if i != n / i {
                    divisors.push(n / i);
                }
            }
        }

        divisors.sort();
        divisors.dedup();
        divisors
    }
}

impl Gcd<i64> for i64 {
    fn gcd(&self, other: &i64) -> i64 {
        gcd(*self, *other)
    }
}

impl Lcm<i64> for i64 {
    fn lcm(&self, other: &i64) -> i64 {
        lcm(*self, *other)
    }
}

// i128 implementations
impl Divisors for i128 {
    fn divisors(&self) -> Vec<i64> {
        if *self == 0 {
            return vec![];
        }

        let n = self.abs();
        if n == 1 {
            return vec![1];
        }

        // Check if the value fits in i64
        let n_i64 = if n > i64::MAX as i128 {
            // For very large numbers, return limited divisors
            return vec![1, *self as i64];
        } else {
            n as i64
        };

        let mut divisors = Vec::new();
        let sqrt_n = (n_i64 as f64).sqrt() as i64;

        for i in 1..=sqrt_n {
            if n_i64 % i == 0 {
                divisors.push(i);
                if i != n_i64 / i {
                    divisors.push(n_i64 / i);
                }
            }
        }

        divisors.sort();
        divisors.dedup();
        divisors
    }
}

impl GcdI128<i128> for i128 {
    fn gcd_i128(&self, other: &i128) -> i128 {
        gcd_i128(*self, *other)
    }
}

impl LcmI128<i128> for i128 {
    fn lcm_i128(&self, other: &i128) -> i128 {
        lcm_i128(*self, *other)
    }
}

/// Represents a prime factor with its exponent.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct PrimeFactor {
    prime: i64,
    exponent: u32,
}

/// Computes the prime factorization of a positive integer.
///
/// Returns a vector of (prime, exponent) pairs sorted by prime.
#[allow(dead_code)]
fn prime_factorization(n: i64) -> Vec<PrimeFactor> {
    if n <= 1 {
        return Vec::new();
    }

    let mut n = n;
    let mut factors = Vec::new();

    // Check divisibility by 2
    let mut exp = 0u32;
    while n % 2 == 0 {
        exp += 1;
        n /= 2;
    }
    if exp > 0 {
        factors.push(PrimeFactor {
            prime: 2,
            exponent: exp,
        });
    }

    // Check odd divisors from 3 onwards
    let mut i = 3i64;
    while i * i <= n {
        exp = 0;
        while n % i == 0 {
            exp += 1;
            n /= i;
        }
        if exp > 0 {
            factors.push(PrimeFactor {
                prime: i,
                exponent: exp,
            });
        }
        i += 2;
    }

    // If n is still greater than 1, then it's a prime factor
    if n > 1 {
        factors.push(PrimeFactor {
            prime: n,
            exponent: 1,
        });
    }

    factors
}

/// Recursively generates all divisors from prime factorization.
#[allow(dead_code)]
fn generate_divisors(
    factors: &[PrimeFactor],
    index: usize,
    current: i64,
    divisors: &mut Vec<i64>,
) {
    if index == factors.len() {
        divisors.push(current);
        return;
    }

    let factor = &factors[index];
    let mut power = 1i64;

    for _ in 0..=factor.exponent {
        generate_divisors(factors, index + 1, current * power, divisors);
        power *= factor.prime;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisors() {
        let n = 1i64;
        assert_eq!(n.divisors(), vec![1]);

        let n = 12i64;
        assert_eq!(n.divisors(), vec![1, 2, 3, 4, 6, 12]);

        let n = 28i64;
        assert_eq!(n.divisors(), vec![1, 2, 4, 7, 14, 28]);

        let n = 36i64;
        assert_eq!(n.divisors(), vec![1, 2, 3, 4, 6, 9, 12, 18, 36]);
    }

    #[test]
    fn test_gcd_trait() {
        let a = 48i64;
        let b = 18i64;
        assert_eq!(a.gcd(&b), 6);

        let a = 17i64;
        let b = 23i64;
        assert_eq!(a.gcd(&b), 1);

        let a = 0i64;
        let b = 5i64;
        assert_eq!(a.gcd(&b), 5);
    }

    #[test]
    fn test_lcm_trait() {
        let a = 21i64;
        let b = 6i64;
        assert_eq!(a.lcm(&b), 42);

        let a = 5i64;
        let b = 7i64;
        assert_eq!(a.lcm(&b), 35);
    }

    #[test]
    fn test_prime_factorization() {
        fn check_factors(n: i64, expected: &[(i64, u32)]) {
            let factors = prime_factorization(n);
            assert_eq!(factors.len(), expected.len());
            for (i, (prime, exp)) in expected.iter().enumerate() {
                assert_eq!(factors[i].prime, *prime);
                assert_eq!(factors[i].exponent, *exp);
            }
        }

        check_factors(12, &[(2, 2), (3, 1)]);
        check_factors(28, &[(2, 2), (7, 1)]);
        check_factors(360, &[(2, 3), (3, 2), (5, 1)]);
        check_factors(9973, &[(9973, 1)]); // prime
    }

    #[test]
    fn test_i128_traits() {
        let a = 48i128;
        let b = 18i128;
        assert_eq!(a.gcd_i128(&b), 6);

        let a = 21i128;
        let b = 6i128;
        assert_eq!(a.lcm_i128(&b), 42);
    }
}
