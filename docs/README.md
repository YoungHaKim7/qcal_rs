# tcal_rs Documentation

Welcome to the comprehensive documentation for the tcal_rs library.

## Available Documentation

### 1. [Mathematical Formulas Reference](MATHEMATICAL_FORMULAS.md)
Detailed mathematical formulas and explanations for all operations:
- GCD/LCM formulas and algorithms
- Euler's totient function
- Modular arithmetic operations
- Rounding operations comparison
- Prime number theorems
- Bernoulli numbers
- Trigonometric functions

### 2. [Library Documentation](LIBRARY_DOCUMENTATION.md)
Complete API reference and usage guide:
- Module structure overview
- Function signatures and descriptions
- Algorithm explanations
- Time/space complexity analysis
- Usage examples
- Performance considerations

### 3. Inline Documentation (Run `cargo doc --open`)
All source files contain detailed Rust documentation comments:
- Mathematical background for each function
- Algorithm descriptions
- Example code
- Parameter descriptions
- Return value documentation

## Quick Reference

### Core Number Theory Functions

| Function | Description | Example |
|----------|-------------|---------|
| `gcd(a, b)` | Greatest common divisor | `gcd(48, 18)` → `6` |
| `lcm(a, b)` | Least common multiple | `lcm(21, 6)` → `42` |
| `totient(n)` | Euler's totient φ(n) | `totient(30)` → `8` |
| `is_prime(n)` | Primality test | `is_prime(17)` → `true` |
| `powmod(b, e, m)` | Modular exponentiation | `powmod(4, 13, 497)` → `445` |

### Calculator Functions

| Function | Description | Example |
|----------|-------------|---------|
| `sin(x)` | Sine | `sin(pi/2)` → `1` |
| `cos(x)` | Cosine | `cos(0)` → `1` |
| `tan(x)` | Tangent | `tan(pi/4)` → `1` |
| `sqrt(x)` | Square root | `sqrt(16)` → `4` |
| `ln(x)` | Natural log | `ln(e)` → `1` |

## Viewing Documentation

### Generate and View Locally

```bash
# Generate documentation and open in browser
cargo doc --open

# Generate documentation for public items only
cargo doc --no-deps --open

# Include private items
cargo doc --document-private-items --open
```

### Read Online Documentation

1. Open `docs/MATHEMATICAL_FORMULAS.md` for mathematical background
2. Open `docs/LIBRARY_DOCUMENTATION.md` for API reference
3. Run `cargo doc --open` for generated Rust docs

## Key Concepts

### Number Theory

**Greatest Common Divisor (GCD)**
The largest positive integer that divides both numbers without remainder.

**Euler's Totient φ(n)**
Counts integers up to n that are coprime to n. For prime p: φ(p) = p-1.

**Modular Exponentiation**
Efficiently computes (base^exp) mod modulus using binary exponentiation.

### Calculator

**Expression Pipeline**
Input → Preprocessor → Lexer → Parser → Evaluator → Formatter → Output

**Supported Bases**
- Hex: `0xFF` → 255
- Binary: `0b1010` → 10
- Octal: `0o755` → 493

## Examples

### Basic Number Theory

```rust
use tcal_rs::*;

// GCD of two numbers
let gcd = gcd(48, 18);  // 6

// Least common multiple
let lcm = lcm(21, 6);   // 42

// Modular exponentiation (useful in cryptography)
let result = powmod(4, 13, 497);  // 4^13 mod 497 = 445

// Check if a number is prime
let prime = is_prime(7919);  // true (1000th prime)

// Find the next prime
let next = next_prime(100);  // 101

// Euler's totient function
let phi = totient(30);  // 8
```

### Calculator Usage

```rust
use tcal_rs::calculator::Engine;

let mut engine = Engine::new();

// Basic arithmetic
engine.eval("2 + 2 * 3");  // "8"

// Trigonometry
engine.eval("sin(pi/2)");  // "1"

// Number theory functions
engine.eval("gcd(48, 18)");  // "6"
engine.eval("totient(30)");  // "8"

// Variable assignment
engine.eval("x = 5");  // "5"
engine.eval("x * 2");  // "10"

// Different number bases
engine.eval("0xFF + 1");     // "256"
engine.eval("0b1010 + 2");  // "12"
```

## Algorithm Complexity

| Operation | Time | Space |
|-----------|------|-------|
| GCD | O(log min(a,b)) | O(1) |
| LCM | O(log min(a,b)) | O(1) |
| Totient | O(√n) | O(1) |
| is_prime (small) | O(√n) | O(1) |
| is_prime (large) | O(k log³n) | O(1) |
| primes_up_to | O(n log log n) | O(n) |
| powmod | O(log exp) | O(1) |

## Contributing

When adding new functions:
1. Add detailed mathematical background in code comments
2. Include examples in doc comments
3. Update this documentation
4. Ensure `cargo doc --open` generates clean docs

## License

This is a Rust port of libqalculate number theory module.
