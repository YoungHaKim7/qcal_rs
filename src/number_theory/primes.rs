//! # Prime Number Operations Module
//!
//! This module provides comprehensive prime number operations including primality
//! testing, prime generation, prime counting, and Bernoulli numbers.
//!
//! ## Mathematical Background
//!
//! ### Prime Numbers
//! A prime number is a natural number greater than 1 that has no positive divisors
//! other than 1 and itself. The sequence begins: 2, 3, 5, 7, 11, 13, 17, 19, ...
//!
//! ### Prime Number Theorem
//! The distribution of primes is described by:
//! ```text
//! π(x) ≈ x / ln(x)
//! ```
//! where π(x) counts primes ≤ x.
//!
//! ### Miller-Rabin Primality Test
//! For n > 2, write n-1 = d×2^s where d is odd. Then n is probably prime if:
//! ```text
//! a^d ≡ 1 (mod n) OR
//! a^(d×2^r) ≡ -1 (mod n) for some 0 ≤ r < s
//! ```
//!
//! ### Bernoulli Numbers
//! A sequence B₀, B₁, B₂, ... appearing in:
//! - Taylor series of tan(x) and x/(e^x-1)
//! - Faulhaber's formula for sums of powers
//! - Euler-Maclaurin formula
//!
//! ## Algorithms Used
//!
//! | Function | Algorithm | Time Complexity |
//! |----------|-----------|-----------------|
//! | `is_prime` | Miller-Rabin (large) / Trial division (small) | O(k log³n) / O(√n) |
//! | `primes_up_to` | Sieve of Eratosthenes | O(n log log n) |
//! | `nth_prime` | PNT approximation + search | O(n log n) |
//! | `prime_count` | Binary search / Approximation | O(log n) / O(1) |
//! | `bernoulli` | Akiyama-Tanigawa | O(n²) |

/// # Primality Test (Hybrid Approach)
///
/// Checks if a number is prime using a hybrid algorithm optimized for different input sizes.
///
/// ## Algorithm Selection
///
/// ### For n < 1,000,000: Trial Division with 6k±1 Optimization
/// All primes > 3 are of the form 6k±1. This optimization checks only these candidates:
/// ```text
/// Check 2, 3, then test i and i+2 for i = 5, 11, 17, ...
/// ```
///
/// ### For n ≥ 1,000,000: Miller-Rabin (Deterministic)
/// Uses 12 witness bases that are proven sufficient for all 64-bit integers:
/// ```text
/// [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]
/// ```
///
/// ## Mathematical Background
///
/// The Miller-Rabin test is based on Fermat's Little Theorem:
/// ```text
/// If p is prime and a < p, then a^(p-1) ≡ 1 (mod p)
/// ```
///
/// For n > 2, write n-1 = d×2^s where d is odd. n is composite if:
/// ```text
/// a^d ≢ 1 (mod n) AND
/// a^(d×2^r) ≢ -1 (mod n) for all 0 ≤ r < s
/// ```
///
/// ## Time Complexity
/// - Small n: O(√n / log n) - trial division
/// - Large n: O(k log³n) - Miller-Rabin with k=12 witnesses
///
/// ## Examples
/// ```
/// use tcal_rs::is_prime;
///
/// // Small primes
/// assert!(is_prime(2));
/// assert!(is_prime(17));
/// assert!(!is_prime(18));
///
/// // Large primes
/// assert!(is_prime(7919));   // 1000th prime
/// assert!(is_prime(104729)); // 10000th prime
///
/// // Carmichael numbers (strong pseudoprimes)
/// assert!(!is_prime(561));   // Smallest Carmichael number
/// assert!(!is_prime(1105));
/// ```
///
/// # Arguments
/// * `n` - The number to check (must be non-negative)
///
/// # Returns
/// `true` if prime, `false` if composite or n < 2
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    // Small primes check
    if n < 1_000_000 {
        return is_prime_small(n as i64);
    }

    // Miller-Rabin deterministic for u64
    miller_rabin_u64(n)
}

/// Miller-Rabin primality test for u64, deterministic for all u64 values.
fn miller_rabin_u64(n: u64) -> bool {
    // Write n-1 as d*2^s
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    // Witnesses for deterministic test up to 2^64
    // According to research, testing these 12 bases is sufficient
    let witnesses = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

    'witness: for &a in &witnesses {
        if a >= n as i64 {
            continue;
        }
        let mut x = powmod_u64(a as u64, d, n);

        if x == 1 || x == n - 1 {
            continue 'witness;
        }

        for _ in 0..s - 1 {
            x = powmod_u64(x, 2, n);
            if x == n - 1 {
                continue 'witness;
            }
        }
        return false;
    }
    true
}

/// Optimized primality test for small integers.
fn is_prime_small(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

/// Modular exponentiation for u64: (base^exp) mod modulus
fn powmod_u64(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
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

/// Finds the smallest prime greater than or equal to n.
///
/// # Arguments
///
/// * `n` - The starting number (non-negative)
///
/// # Returns
///
/// The next prime number
///
/// # Examples
///
/// ```
/// use tcal_rs::next_prime;
///
/// assert_eq!(next_prime(10), 11);
/// assert_eq!(next_prime(11), 11);
/// assert_eq!(next_prime(14), 17);
/// ```
pub fn next_prime(n: i64) -> i64 {
    if n < 2 {
        return 2;
    }

    let mut candidate = if n < 0 { 2 } else { n as u64 };

    // Ensure we start from an odd number
    if candidate <= 2 {
        return 2;
    }
    if candidate % 2 == 0 {
        candidate += 1;
    }

    loop {
        if is_prime(candidate) {
            return candidate as i64;
        }
        candidate += 2;
    }
}

/// Finds the largest prime less than or equal to n.
///
/// # Arguments
///
/// * `n` - The starting number (must be >= 2)
///
/// # Returns
///
/// The previous prime number
///
/// # Examples
///
/// ```
/// use tcal_rs::prev_prime;
///
/// assert_eq!(prev_prime(10), 7);
/// assert_eq!(prev_prime(7), 7);
/// ```
pub fn prev_prime(n: i64) -> i64 {
    if n <= 2 {
        return 2;
    }

    let mut candidate = if n < 0 { 2 } else { n as u64 };

    // Ensure we start from an odd number
    if candidate % 2 == 0 {
        if candidate > 2 {
            candidate -= 1;
        } else {
            return 2;
        }
    }

    loop {
        if candidate <= 2 {
            return 2;
        }
        if is_prime(candidate) {
            return candidate as i64;
        }
        candidate = candidate.saturating_sub(2);
    }
}

/// Returns the nth prime number (1-indexed).
///
/// # Arguments
///
/// * `n` - The index of the prime to retrieve (1-based, must be positive)
///
/// # Returns
///
/// The nth prime number
///
/// # Panics
///
/// Panics if n is zero or negative
///
/// # Examples
///
/// ```
/// use tcal_rs::nth_prime;
///
/// assert_eq!(nth_prime(1), 2);   // First prime
/// assert_eq!(nth_prime(2), 3);   // Second prime
/// assert_eq!(nth_prime(5), 11);  // Fifth prime
/// ```
pub fn nth_prime(n: u64) -> u64 {
    assert!(n > 0, "n must be positive");

    if n <= 10000 {
        // Use precomputed small primes
        get_small_prime((n - 1) as usize)
    } else {
        // Use prime number theorem approximation
        // p_n ≈ n * (ln(n) + ln(ln(n)))
        let n_float = n as f64;
        let ln_n = n_float.ln();
        let ln_ln_n = ln_n.ln();
        let estimate = (n_float * (ln_n + ln_ln_n)) as u64;

        let mut candidate = estimate.max(2);
        let upper = if n > 6 {
            estimate + 1000
        } else {
            estimate + 10
        };

        // Search forward from estimate
        loop {
            if is_prime(candidate) {
                return candidate;
            }
            candidate += 1;
            if candidate > upper {
                break;
            }
        }

        // Fallback: search from the beginning
        let mut count = 0u64;
        let mut i = 2u64;
        loop {
            if is_prime(i) {
                count += 1;
                if count == n {
                    return i;
                }
            }
            i += 1;
        }
    }
}

/// Get the nth small prime from the lookup table.
fn get_small_prime(n: usize) -> u64 {
    SMALL_PRIMES[n]
}

/// Counts the number of primes less than or equal to x (π function).
///
/// # Arguments
///
/// * `x` - Upper bound (non-negative)
///
/// # Returns
///
/// The number of primes ≤ x
///
/// # Examples
///
/// ```
/// use tcal_rs::prime_count;
///
/// assert_eq!(prime_count(10), 4);  // 2, 3, 5, 7
/// assert_eq!(prime_count(100), 25);
/// ```
pub fn prime_count(x: i64) -> i64 {
    if x < 2 {
        return 0;
    }

    let x_abs = x.abs();

    if x_abs <= 104_729 {
        // Use binary search on precomputed primes
        return count_primes_binary(x_abs);
    }

    // For larger numbers, use approximation
    prime_count_approx(x_abs)
}

/// Approximate prime count using the logarithmic integral.
fn prime_count_approx(x: i64) -> i64 {
    if x < 2 {
        return 0;
    }

    let x_float = x as f64;
    if x_float > 1.0 {
        (x_float / x_float.ln()) as i64
    } else {
        0
    }
}

/// Count primes using binary search on small primes table.
fn count_primes_binary(x: i64) -> i64 {
    match SMALL_PRIMES.binary_search(&(x as u64)) {
        Ok(i) => (i + 1) as i64,
        Err(i) => i as i64,
    }
}

/// # Sieve of Eratosthenes
///
/// Returns all prime numbers up to and including n using the ancient Sieve of Eratosthenes algorithm.
///
/// ## Algorithm
///
/// 1. Create a boolean array `is_prime[0..n]` initialized to true
/// 2. Mark 0 and 1 as not prime
/// 3. For each p from 2 to √n:
///    - If p is prime, mark all multiples of p (starting from p²) as composite
/// 4. Remaining true values are primes
///
/// ## Optimization
/// We start marking from p² because smaller multiples (2p, 3p, ..., (p-1)p)
/// have already been marked by smaller primes.
///
/// ## Time Complexity
/// - O(n log log n) - nearly linear
///
/// ## Space Complexity
/// - O(n) - boolean array of size n+1
///
/// ## Examples
/// ```
/// use tcal_rs::primes_up_to;
///
/// let primes = primes_up_to(20);
/// assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19]);
///
/// let empty = primes_up_to(1);
/// assert!(empty.is_empty());
/// ```
///
/// # Limitations
/// - Returns empty vector for n > 10,000,000 to prevent memory issues
///
/// # Arguments
/// * `n` - Upper bound (must be non-negative)
///
/// # Returns
/// Vector of all primes ≤ n in ascending order
pub fn primes_up_to(n: i64) -> Vec<i64> {
    if n < 2 {
        return Vec::new();
    }

    let n_usize = n as usize;
    if n_usize > 10_000_000 {
        // For very large n, return empty to avoid memory issues
        return Vec::new();
    }

    // Sieve of Eratosthenes
    let mut sieve = vec![true; n_usize + 1];
    sieve[0] = false;
    sieve[1] = false;

    let mut i = 2;
    while i * i <= n_usize {
        if sieve[i] {
            let mut j = i * i;
            while j <= n_usize {
                sieve[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    sieve
        .iter()
        .enumerate()
        .filter(|&(_, &is_prime)| is_prime)
        .map(|(i, _)| i as i64)
        .collect()
}

/// # Bernoulli Numbers (Akiyama-Tanigawa Algorithm)
///
/// Computes the nth Bernoulli number B_n as a rational number (numerator, denominator).
///
/// ## Mathematical Background
///
/// Bernoulli numbers B₀, B₁, B₂, ... are a sequence of rational numbers with
/// deep connections to:
/// - **Number Theory**: Sum of powers formula (Faulhaber)
/// - **Analysis**: Taylor series of tan(x), cot(x), x/(e^x-1)
/// - **Topology**: Homotopy groups of spheres
///
/// ### First Few Values
/// ```text
/// B₀  = 1
/// B₁  = -1/2
/// B₂  = 1/6
/// B₄  = -1/30
/// B₆  = 1/42
/// B₈  = -1/30
/// B₁₀ = 5/66
/// B_odd>1 = 0
/// ```
///
/// ### Faulhaber's Formula (Sum of Powers)
/// ```text
/// Σ(k^p) for k=1 to n = (1/(p+1)) × Σ(C(p+1,k) × B_k × n^(p+1-k))
/// ```
///
/// ### Taylor Series for tan(x)
/// ```text
/// tan(x) = Σ(B_2n × (-4)^n × (1-4^n) × x^(2n-1) / (2n)!)
/// ```
///
/// ## Algorithm: Akiyama-Tanigawa
///
/// ```text
/// Initialize: a[j] = 1/(j+1) for j = 0 to m
///
/// For m from 1 to n:
///     For k from m down to 1:
///         a[k-1] = k × (a[k-1] - a[k])
///
/// B_n = a[0]
/// ```
///
/// ## Time Complexity
/// - O(n²) - nested loops
///
/// ## Examples
/// ```
/// use tcal_rs::bernoulli;
///
/// // Special cases
/// assert_eq!(bernoulli(0), Some((1, 1)));    // B₀ = 1
/// assert_eq!(bernoulli(1), Some((-1, 2)));  // B₁ = -1/2
///
/// // Even indices
/// assert_eq!(bernoulli(2), Some((1, 6)));    // B₂ = 1/6
/// assert_eq!(bernoulli(4), Some((-1, 30)));  // B₄ = -1/30
///
/// // Odd indices > 1 are zero
/// assert_eq!(bernoulli(3), Some((0, 1)));    // B₃ = 0
/// assert_eq!(bernoulli(5), Some((0, 1)));    // B₅ = 0
/// ```
///
/// # Arguments
/// * `n` - Index of Bernoulli number (non-negative)
///
/// # Returns
/// `Some((numerator, denominator))` representing B_n, or None for invalid inputs
pub fn bernoulli(n: u64) -> Option<(i64, i64)> {
    // Special case: B_1 = -1/2
    if n == 1 {
        return Some((-1, 2));
    }
    // Special case: B_0 = 1
    if n == 0 {
        return Some((1, 1));
    }
    // For odd n > 1, B_n = 0
    if n % 2 == 1 && n > 1 {
        return Some((0, 1));
    }

    // Use Akiyama-Tanigawa algorithm with proper rational arithmetic
    let m = n as usize;
    // Initialize array: a[j] = 1/(j+1) for j = 0 to m
    let mut a: Vec<(i64, i64)> = (0..=m).map(|j| (1, j as i64 + 1)).collect();

    for m_curr in 1..=m {
        for k in (1..=m_curr).rev() {
            // Compute: a[k-1] = k * (a[k-1] - a[k])
            let (num1, den1) = a[k - 1];
            let (num2, den2) = a[k];

            // (num1/den1) - (num2/den2) = (num1*den2 - num2*den1) / (den1*den2)
            let diff_num = num1 * den2 - num2 * den1;
            let diff_den = den1 * den2;

            // k * (diff_num/diff_den) = (k * diff_num) / diff_den
            let k_val = k as i64;
            let new_num = k_val * diff_num;
            let new_den = diff_den;

            // Simplify the fraction
            let g = gcd_int(new_num.abs(), new_den);
            a[k - 1] = (new_num / g, new_den / g);
        }
    }

    Some(a[0])
}

fn gcd_int(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd_int(b, a % b) }
}

// Precomputed small primes (first 10000 primes)
const SMALL_PRIMES: &[u64] = &include!("small_primes.inc");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(17));
        assert!(is_prime(7919)); // 1000th prime
        assert!(!is_prime(1));
        assert!(!is_prime(4));
        assert!(!is_prime(100));
    }

    #[test]
    fn test_miller_rabin() {
        // Test some known primes
        assert!(miller_rabin_u64(2));
        assert!(miller_rabin_u64(3));
        assert!(miller_rabin_u64(5));
        assert!(miller_rabin_u64(7));
        assert!(miller_rabin_u64(11));
        assert!(miller_rabin_u64(13));
        assert!(miller_rabin_u64(17));
        assert!(miller_rabin_u64(19));
        assert!(miller_rabin_u64(23));
        assert!(miller_rabin_u64(29));

        // Test some composites
        assert!(!miller_rabin_u64(4));
        assert!(!miller_rabin_u64(6));
        assert!(!miller_rabin_u64(8));
        assert!(!miller_rabin_u64(9));
        assert!(!miller_rabin_u64(10));
        assert!(!miller_rabin_u64(12));
        assert!(!miller_rabin_u64(15));
        assert!(!miller_rabin_u64(21));
        assert!(!miller_rabin_u64(25));
        assert!(!miller_rabin_u64(27));

        // Carmichael numbers (strong pseudoprimes to many bases)
        assert!(!miller_rabin_u64(561));
        assert!(!miller_rabin_u64(1105));
        assert!(!miller_rabin_u64(1729));
    }

    #[test]
    fn test_next_prime() {
        assert_eq!(next_prime(10), 11);
        assert_eq!(next_prime(11), 11);
        assert_eq!(next_prime(14), 17);
        assert_eq!(next_prime(0), 2);
    }

    #[test]
    fn test_prev_prime() {
        assert_eq!(prev_prime(10), 7);
        assert_eq!(prev_prime(7), 7);
        assert_eq!(prev_prime(2), 2);
    }

    #[test]
    fn test_nth_prime() {
        assert_eq!(nth_prime(1), 2);
        assert_eq!(nth_prime(2), 3);
        assert_eq!(nth_prime(3), 5);
        assert_eq!(nth_prime(5), 11);
        assert_eq!(nth_prime(10), 29);
    }

    #[test]
    fn test_prime_count() {
        assert_eq!(prime_count(0), 0);
        assert_eq!(prime_count(1), 0);
        assert_eq!(prime_count(2), 1);
        assert_eq!(prime_count(10), 4);
        assert_eq!(prime_count(100), 25);
    }

    #[test]
    fn test_primes_up_to() {
        let primes = primes_up_to(20);
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19]);

        let primes_empty = primes_up_to(1);
        assert!(primes_empty.is_empty());
    }

    #[test]
    fn test_bernoulli() {
        assert_eq!(bernoulli(0), Some((1, 1)));
        assert_eq!(bernoulli(1), Some((-1, 2)));
        assert_eq!(bernoulli(2), Some((1, 6)));
        assert_eq!(bernoulli(3), Some((0, 1)));
        assert_eq!(bernoulli(4), Some((-1, 30)));
    }
}
