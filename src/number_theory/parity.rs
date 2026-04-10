//! Parity checking functions
//!
//! Provides functions to check if a number is odd or even.

/// Checks if an integer is even.
///
/// # Arguments
///
/// * `n` - The integer to check
///
/// # Returns
///
/// `true` if the number is even, `false` otherwise
///
/// # Examples
///
/// ```
/// use tcal_rs::is_even;
///
/// assert!(is_even(4));
/// assert!(!is_even(5));
/// assert!(is_even(0));
/// assert!(is_even(-2));
/// ```
pub fn is_even(n: i64) -> bool {
    n % 2 == 0
}

/// Checks if an integer is odd.
///
/// # Arguments
///
/// * `n` - The integer to check
///
/// # Returns
///
/// `true` if the number is odd, `false` otherwise
///
/// # Examples
///
/// ```
/// use tcal_rs::is_odd;
///
/// assert!(is_odd(5));
/// assert!(!is_odd(4));
/// assert!(!is_odd(0));
/// assert!(is_odd(-3));
/// ```
pub fn is_odd(n: i64) -> bool {
    n % 2 != 0
}

/// Checks if an i128 integer is even.
pub fn is_even_i128(n: i128) -> bool {
    n % 2 == 0
}

/// Checks if an i128 integer is odd.
pub fn is_odd_i128(n: i128) -> bool {
    n % 2 != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even() {
        assert!(is_even(0));
        assert!(is_even(2));
        assert!(is_even(-4));
        assert!(is_even(100));
        assert!(!is_even(1));
        assert!(!is_even(-1));
        assert!(!is_even(99));
    }

    #[test]
    fn test_is_odd() {
        assert!(is_odd(1));
        assert!(is_odd(-1));
        assert!(is_odd(99));
        assert!(!is_odd(0));
        assert!(!is_odd(2));
        assert!(!is_odd(-4));
    }

    #[test]
    fn test_is_even_i128() {
        assert!(is_even_i128(0));
        assert!(is_even_i128(2));
        assert!(is_even_i128(-4));
        assert!(!is_even_i128(1));
        assert!(!is_even_i128(-1));
    }

    #[test]
    fn test_is_odd_i128() {
        assert!(is_odd_i128(1));
        assert!(is_odd_i128(-1));
        assert!(is_odd_i128(99));
        assert!(!is_odd_i128(0));
        assert!(!is_odd_i128(2));
    }
}
