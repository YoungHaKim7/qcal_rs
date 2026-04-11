//! # Rounding and Absolute Value Functions
//!
//! This module provides comprehensive rounding operations and absolute value computations.
//!
//! ## Rounding Modes
//!
//! Different applications require different rounding strategies. This module supports
//! nine standard rounding modes defined in IEEE 754 and mathematical practice.
//!
//! ### Rounding Mode Reference
//!
//! | Mode | Description | Example at 0.5 | Example at 1.5 |
//! |------|-------------|-----------------|-----------------|
//! | `HalfToEven` | Round to nearest even (Banker's) | 0 | 2 |
//! | `HalfAwayFromZero` | Round away from zero | 1 | 2 |
//! | `HalfTowardZero` | Round toward zero | 0 | 1 |
//! | `HalfUp` | Round up (toward +∞) | 1 | 2 |
//! | `HalfDown` | Round down (toward -∞) | 0 | 1 |
//! | `Up` | Ceiling (toward +∞) | 1 | 2 |
//! | `Down` | Floor (toward -∞) | 0 | 1 |
//! | `TowardZero` | Truncate | 0 | 1 |
//! | `AwayFromZero` | Away from zero | 1 | 2 |
//!
//! ## Mathematical Functions
//!
//! ### Floor Function ⌊x⌋
//! Greatest integer less than or equal to x:
//! ```text
//! ⌊2.3⌋ = 2, ⌊-2.3⌋ = -3
//! ```
//!
//! ### Ceiling Function ⌈x⌉
//! Smallest integer greater than or equal to x:
//! ```text
//! ⌈2.3⌉ = 3, ⌈-2.3⌉ = -2
//! ```
//!
//! ### Signum Function sgn(x)
//! Sign of a number:
//! ```text
//! sgn(x) = -1 if x < 0, 0 if x = 0, 1 if x > 0
//! ```
//!
//! ## Applications
//!
//! - **Financial**: Banker's rounding (HalfToEven) for unbiased calculations
//! - **Statistics**: Various modes for confidence intervals
//! - **Graphics**: Rounding for pixel alignment
//! - **Scientific**: Different modes for error propagation

/// # Rounding Mode Enumeration
///
/// Defines nine standard rounding modes for the `round` function.
///
/// ## Mode Descriptions
///
/// ### `HalfToEven` (Banker's Rounding)
/// - At exactly 0.5, rounds to the nearest even integer
/// - Minimizes cumulative error in statistical calculations
/// - Default mode in IEEE 754 and many programming languages
/// - Example: 2.5 → 2, 3.5 → 4
///
/// ### `HalfAwayFromZero`
/// - At exactly 0.5, always rounds away from zero
/// - Common in everyday arithmetic ("round half up" for positive)
/// - Example: 2.5 → 3, -2.5 → -3
///
/// ### `HalfTowardZero`
/// - At exactly 0.5, always rounds toward zero
/// - Less commonly used
/// - Example: 2.5 → 2, -2.5 → -2
///
/// ### `HalfUp`
/// - At exactly 0.5, rounds toward +∞
/// - Example: 2.5 → 3, -2.5 → -2
///
/// ### `HalfDown`
/// - At exactly 0.5, rounds toward -∞
/// - Example: 2.5 → 2, -2.5 → -3
///
/// ### `Up` (Ceiling)
/// - Always rounds toward +∞
/// - Equivalent to the `ceil` function
/// - Example: 2.3 → 3, -2.3 → -2
///
/// ### `Down` (Floor)
/// - Always rounds toward -∞
/// - Equivalent to the `floor` function
/// - Example: 2.3 → 2, -2.3 → -3
///
/// ### `TowardZero`
/// - Always rounds toward zero (truncation)
/// - Removes fractional part
/// - Example: 2.3 → 2, -2.3 → -2
///
/// ### `AwayFromZero`
/// - Always rounds away from zero
/// - Example: 2.3 → 3, -2.3 → -3
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RoundingMode {
    /// Round half to even (banker's rounding)
    #[default]
    HalfToEven = 0,
    /// Round half away from zero
    HalfAwayFromZero = 1,
    /// Round half toward zero
    HalfTowardZero = 2,
    /// Round half up (toward positive infinity)
    HalfUp = 3,
    /// Round half down (toward negative infinity)
    HalfDown = 4,
    /// Round toward positive infinity (ceiling)
    Up = 5,
    /// Round toward negative infinity (floor)
    Down = 6,
    /// Round toward zero (truncation)
    TowardZero = 7,
    /// Round away from zero
    AwayFromZero = 8,
}

/// Computes the absolute value of a rational number.
///
/// # Arguments
///
/// * `numerator` - Numerator of the rational number
/// * `denominator` - Denominator of the rational number
///
/// # Returns
///
/// Absolute value as (numerator, denominator)
///
/// # Examples
///
/// ```
/// use tcal_rs::abs;
///
/// assert_eq!(abs(7, 3), (7, 3));
/// assert_eq!(abs(-7, 3), (7, 3));
/// ```
pub fn abs(numerator: i64, denominator: i64) -> (i64, i64) {
    (numerator.abs(), denominator.abs())
}

/// Computes the absolute value of an integer.
///
/// # Examples
///
/// ```
/// use tcal_rs::abs_integer;
///
/// assert_eq!(abs_integer(-42), 42);
/// ```
pub fn abs_integer(n: i64) -> i64 {
    n.abs()
}

/// Computes the absolute value of an i128 integer.
pub fn abs_integer_i128(n: i128) -> i128 {
    n.abs()
}

/// # Ceiling Function ⌈x⌉
///
/// Computes the ceiling of a rational number, which is the smallest integer
/// greater than or equal to the number (rounds toward +∞).
///
/// ## Mathematical Definition
/// ```text
/// ⌈x⌉ = min{k ∈ ℤ : k ≥ x}
/// ```
///
/// For a rational number a/b:
/// ```text
/// ⌈a/b⌉ = a/b if a is divisible by b
/// ⌈a/b⌉ = ⌊a/b⌋ + 1 otherwise
/// ```
///
/// ## Examples
/// ```
/// use tcal_rs::ceil;
///
/// // Positive numbers
/// assert_eq!(ceil(7, 3), 3);    // 2.33... → 3
/// assert_eq!(ceil(6, 3), 2);    // Exactly 2
///
/// // Negative numbers (rounds toward +∞, i.e., less negative)
/// assert_eq!(ceil(-7, 3), -2);  // -2.33... → -2
/// assert_eq!(ceil(-6, 3), -2);  // Exactly -2
///
/// // Edge cases
/// assert_eq!(ceil(1, 2), 1);    // 0.5 → 1
/// assert_eq!(ceil(-1, 2), 0);   // -0.5 → 0
/// ```
///
/// # Arguments
/// * `numerator` - Numerator of the rational number
/// * `denominator` - Denominator of the rational number (non-zero)
///
/// # Returns
/// The ceiling as an i64
pub fn ceil(numerator: i64, denominator: i64) -> i64 {
    let mut result = numerator / denominator;
    let remainder = numerator % denominator;

    if remainder > 0 && denominator > 0 || remainder < 0 && denominator < 0 {
        result += 1;
    }

    result
}

/// Computes the ceiling of a float.
pub fn ceil_float(f: f64) -> i64 {
    if f > 0.0 && f.fract() != 0.0 {
        f as i64 + 1
    } else {
        f as i64
    }
}

/// # Floor Function ⌊x⌋
///
/// Computes the floor of a rational number, which is the greatest integer
/// less than or equal to the number (rounds toward -∞).
///
/// ## Mathematical Definition
/// ```text
/// ⌊x⌋ = max{k ∈ ℤ : k ≤ x}
/// ```
///
/// For a rational number a/b:
/// ```text
/// ⌊a/b⌋ = a/b if a is divisible by b
/// ⌊a/b⌋ = ⌈a/b⌉ - 1 otherwise
/// ```
///
/// ## Examples
/// ```
/// use tcal_rs::floor;
///
/// // Positive numbers
/// assert_eq!(floor(7, 3), 2);    // 2.33... → 2
/// assert_eq!(floor(6, 3), 2);    // Exactly 2
///
/// // Negative numbers (rounds toward -∞, i.e., more negative)
/// assert_eq!(floor(-7, 3), -3);  // -2.33... → -3
/// assert_eq!(floor(-6, 3), -2);  // Exactly -2
///
/// // Edge cases
/// assert_eq!(floor(1, 2), 0);    // 0.5 → 0
/// assert_eq!(floor(-1, 2), -1);  // -0.5 → -1
/// ```
///
/// # Arguments
/// * `numerator` - Numerator of the rational number
/// * `denominator` - Denominator of the rational number (non-zero)
///
/// # Returns
/// The floor as an i64
pub fn floor(numerator: i64, denominator: i64) -> i64 {
    let mut result = numerator / denominator;
    let remainder = numerator % denominator;

    if remainder < 0 && denominator > 0 || remainder > 0 && denominator < 0 {
        result -= 1;
    }

    result
}

/// Computes the floor of a float.
pub fn floor_float(f: f64) -> i64 {
    if f < 0.0 && f.fract() != 0.0 {
        f as i64 - 1
    } else {
        f as i64
    }
}

/// Computes the truncation of a rational number (round toward zero).
///
/// # Arguments
///
/// * `numerator` - Numerator of the rational number
/// * `denominator` - Denominator of the rational number
///
/// # Returns
///
/// The truncated value as an i64
///
/// # Examples
///
/// ```
/// use tcal_rs::trunc;
///
/// assert_eq!(trunc(7, 3), 2);
/// assert_eq!(trunc(-7, 3), -2);
/// assert_eq!(trunc(6, 3), 2);
/// ```
pub fn trunc(numerator: i64, denominator: i64) -> i64 {
    numerator / denominator
}

/// Computes the truncation of a float.
pub fn trunc_float(f: f64) -> i64 {
    f as i64
}

/// # General Rounding Function
///
/// Rounds a rational number to the nearest integer using the specified rounding mode.
///
/// ## Algorithm
///
/// 1. Extract integer part and remainder
/// 2. Convert remainder to floating-point fraction for precise comparison
/// 3. Apply the selected rounding mode:
///    - Compare fraction to 0.5
///    - Handle ties (exactly 0.5) according to mode
///    - Apply directional modes (Up, Down, etc.)
///
/// ## Examples
/// ```
/// use tcal_rs::{round, RoundingMode};
///
/// // HalfToEven (Banker's rounding)
/// assert_eq!(round(7, 3, RoundingMode::HalfToEven), 2);  // 2.33... → 2
/// assert_eq!(round(8, 3, RoundingMode::HalfToEven), 3);  // 2.66... → 3
/// assert_eq!(round(5, 2, RoundingMode::HalfToEven), 2);  // 2.5 → 2 (even)
/// assert_eq!(round(7, 2, RoundingMode::HalfToEven), 4);  // 3.5 → 4 (even)
///
/// // HalfAwayFromZero
/// assert_eq!(round(5, 2, RoundingMode::HalfAwayFromZero), 3);  // 2.5 → 3
/// assert_eq!(round(-5, 2, RoundingMode::HalfAwayFromZero), -3); // -2.5 → -3
///
/// // Ceiling (Up)
/// assert_eq!(round(7, 3, RoundingMode::Up), 3);   // Same as ceil(7, 3)
/// assert_eq!(round(-7, 3, RoundingMode::Up), -2); // Same as ceil(-7, 3)
///
/// // Floor (Down)
/// assert_eq!(round(7, 3, RoundingMode::Down), 2);   // Same as floor(7, 3)
/// assert_eq!(round(-7, 3, RoundingMode::Down), -3); // Same as floor(-7, 3)
/// ```
///
/// # Arguments
/// * `numerator` - Numerator of the rational number
/// * `denominator` - Denominator of the rational number (non-zero)
/// * `mode` - Rounding mode to use
///
/// # Returns
/// The rounded value as an i64
pub fn round(numerator: i64, denominator: i64, mode: RoundingMode) -> i64 {
    let integer_part = numerator / denominator;
    let remainder = numerator.abs() % denominator.abs();

    if remainder == 0 {
        return integer_part;
    }

    // Convert to float for precise comparison
    let fraction = (remainder as f64) / (denominator.abs() as f64);

    match mode {
        RoundingMode::HalfToEven => {
            if fraction > 0.5 {
                integer_part + integer_part.signum()
            } else if fraction < 0.5 {
                integer_part
            } else {
                // Exactly at 0.5, round to nearest even
                if integer_part % 2 == 0 {
                    integer_part
                } else {
                    integer_part + integer_part.signum()
                }
            }
        }
        RoundingMode::HalfAwayFromZero => {
            if fraction >= 0.5 {
                integer_part + integer_part.signum()
            } else {
                integer_part
            }
        }
        RoundingMode::HalfTowardZero => {
            if fraction > 0.5 {
                integer_part + integer_part.signum()
            } else {
                integer_part
            }
        }
        RoundingMode::HalfUp => {
            if fraction >= 0.5 {
                integer_part + 1
            } else {
                integer_part
            }
        }
        RoundingMode::HalfDown => {
            if fraction > 0.5 {
                integer_part + 1
            } else {
                integer_part
            }
        }
        RoundingMode::Up => ceil(numerator, denominator),
        RoundingMode::Down => floor(numerator, denominator),
        RoundingMode::TowardZero => trunc(numerator, denominator),
        RoundingMode::AwayFromZero => {
            if integer_part >= 0 {
                ceil(numerator, denominator)
            } else {
                floor(numerator, denominator)
            }
        }
    }
}

/// Signum function - returns the sign of a number.
///
/// Returns:
/// - `-1` if the number is negative
/// - `0` if the number is zero
/// - `1` if the number is positive
///
/// # Examples
///
/// ```
/// use tcal_rs::signum;
///
/// assert_eq!(signum(-5), -1);
/// assert_eq!(signum(0), 0);
/// assert_eq!(signum(5), 1);
/// ```
pub fn signum(n: i64) -> i64 {
    n.signum()
}

/// Signum function for i128.
pub fn signum_i128(n: i128) -> i128 {
    n.signum()
}

/// Signum function for generic integers.
pub fn signum_integer(n: i64) -> i64 {
    if n == 0 {
        0
    } else if n > 0 {
        1
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs() {
        assert_eq!(abs(7, 3), (7, 3));
        assert_eq!(abs(-7, 3), (7, 3));
        assert_eq!(abs(7, -3), (7, 3));
    }

    #[test]
    fn test_abs_integer() {
        assert_eq!(abs_integer(-42), 42);
        assert_eq!(abs_integer(42), 42);
        assert_eq!(abs_integer(0), 0);
    }

    #[test]
    fn test_ceil() {
        assert_eq!(ceil(7, 3), 3);
        assert_eq!(ceil(-7, 3), -2);
        assert_eq!(ceil(6, 3), 2);
        assert_eq!(ceil(-6, 3), -2);
        assert_eq!(ceil(1, 2), 1);
        assert_eq!(ceil(-1, 2), 0);
    }

    #[test]
    fn test_floor() {
        assert_eq!(floor(7, 3), 2);
        assert_eq!(floor(-7, 3), -3);
        assert_eq!(floor(6, 3), 2);
        assert_eq!(floor(-6, 3), -2);
        assert_eq!(floor(1, 2), 0);
        assert_eq!(floor(-1, 2), -1);
    }

    #[test]
    fn test_trunc() {
        assert_eq!(trunc(7, 3), 2);
        assert_eq!(trunc(-7, 3), -2);
        assert_eq!(trunc(6, 3), 2);
        assert_eq!(trunc(1, 2), 0);
        assert_eq!(trunc(-1, 2), 0);
    }

    #[test]
    fn test_round_half_to_even() {
        assert_eq!(round(7, 3, RoundingMode::HalfToEven), 2);
        assert_eq!(round(8, 3, RoundingMode::HalfToEven), 3);
        assert_eq!(round(5, 2, RoundingMode::HalfToEven), 2);
        assert_eq!(round(7, 2, RoundingMode::HalfToEven), 4);
        assert_eq!(round(9, 2, RoundingMode::HalfToEven), 4);
        assert_eq!(round(11, 2, RoundingMode::HalfToEven), 6);
    }

    #[test]
    fn test_round_half_away_from_zero() {
        assert_eq!(round(7, 3, RoundingMode::HalfAwayFromZero), 2);
        assert_eq!(round(8, 3, RoundingMode::HalfAwayFromZero), 3);
        assert_eq!(round(5, 2, RoundingMode::HalfAwayFromZero), 3);
        assert_eq!(round(-5, 2, RoundingMode::HalfAwayFromZero), -3);
    }

    #[test]
    fn test_signum() {
        assert_eq!(signum(-5), -1);
        assert_eq!(signum(0), 0);
        assert_eq!(signum(5), 1);
    }

    #[test]
    fn test_ceil_float() {
        assert_eq!(ceil_float(2.3), 3);
        assert_eq!(ceil_float(-2.3), -2);
        assert_eq!(ceil_float(2.0), 2);
    }

    #[test]
    fn test_floor_float() {
        assert_eq!(floor_float(2.3), 2);
        assert_eq!(floor_float(-2.3), -3);
        assert_eq!(floor_float(2.0), 2);
    }
}
