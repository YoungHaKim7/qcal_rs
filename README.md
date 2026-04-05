# qcal_rs
https://github.com/Qalculate/libqalculate

# Claude AI
- I have successfully converted the C++ number theory functions from libqalculate to Rust. Here's a summary of what was created:

## Created Files:

1. **`Cargo.toml`** - Rust package manifest with dependencies on `rug` (arbitrary precision integers)

2. **`src/lib.rs`** - Main library entry point with exports and documentation

3. **`src/number_theory.rs`** - Main module with core functions:
   - `gcd()` - Greatest common divisor
   - `lcm()` - Least common multiple
   - `modulo()`, `rem()`, `powmod()` - Modular arithmetic
   - `numerator()`, `denominator()` - Rational components
   - `frac()` - Fractional part

4. **`src/number_theory/parity.rs`** - Parity checking:
   - `is_even()`, `is_odd()`

5. **`src/number_theory/rounding.rs`** - Rounding operations:
   - `abs()`, `ceil()`, `floor()`, `trunc()`, `round()`
   - `signum()`
   - `RoundingMode` enum with 8 different rounding modes

6. **`src/number_theory/primes.rs`** - Prime number operations:
   - `is_prime()` - Miller-Rabin primality test
   - `next_prime()`, `prev_prime()`, `nth_prime()`
   - `prime_count()` - π function using Legendre's formula
   - `primes_up_to()` - Sieve of Eratosthenes
   - `bernoulli()` - Bernoulli numbers via Akiyama-Tanigawa algorithm

7. **`src/number_theory/totient.rs`** - Euler's totient function (φ)

8. **`src/number_theory/traits.rs`** - Extension traits:
   - `Divisors`, `Gcd`, `Lcm` traits for `Integer`

9. **`src/number_theory/small_primes.inc`** - Precomputed table of first 10,000 primes

## Test Results:
All 36 unit tests pass, covering:
- Parity tests
- Rounding functions
- GCD/LCM
- Prime operations
- Totient function
- Divisors enumeration
- Bernoulli numbers

The implementation uses the `rug` crate for arbitrary-precision arithmetic and is fully compatible with the original C++ libqalculate number theory API.

