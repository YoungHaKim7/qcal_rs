# tcal_rs Library Documentation

## Overview

`tcal_rs` is a Rust port of libqalculate number theory module, providing comprehensive mathematical operations including number theory, calculator functionality, and utility functions.

---

## Module Structure

```
tcal_rs/
├── lib.rs                    # Main library entry point
├── number_theory.rs          # Core number theory functions
├── number_theory/
│   ├── parity.rs            # Even/odd checking
│   ├── primes.rs            # Prime number operations
│   ├── rounding.rs          # Rounding and absolute value
│   ├── totient.rs           # Euler's totient function
│   └── traits.rs            # Extension traits for i64/i128
├── calculator/
│   ├── mod.rs               # Calculator module
│   ├── lexer.rs             # Lexical analysis (tokenization)
│   ├── parser.rs            # Expression parsing (AST generation)
│   ├── ast.rs               # Abstract Syntax Tree definitions
│   ├── evaluator.rs         # Expression evaluation
│   ├── engine.rs            # Calculator engine with preprocessing
│   ├── formatter.rs         # Result formatting
│   └── converter.rs         # Unicode conversion utility
└── fprice.rs                # Price formatting with thousand separators
```

---

## Core Number Theory Module (`number_theory.rs`)

### Functions Overview

| Function | Description | Signature |
|----------|-------------|-----------|
| `gcd` | Greatest common divisor (Euclidean) | `(i64, i64) -> i64` |
| `gcd_i128` | GCD for i128 integers | `(i128, i128) -> i128` |
| `lcm` | Least common multiple | `(i64, i64) -> i64` |
| `lcm_i128` | LCM for i128 integers | `(i128, i128) -> i128` |
| `frac` | Fractional part of rational | `(i64, i64) -> (i64, i64)` |
| `rem` | Remainder of Euclidean division | `(i64, i64) -> i64` |
| `modulo` | Modulo operation (always non-negative) | `(i64, i64) -> i64` |
| `powmod` | Modular exponentiation | `(u64, u64, u64) -> u64` |
| `powmod_i64` | Modular exponentiation for signed integers | `(i64, u64, i64) -> i64` |
| `numerator` | Returns numerator of rational | `(i64, i64) -> i64` |
| `denominator` | Returns denominator of rational | `(i64, i64) -> i64` |

### Detailed Documentation

#### `gcd(a: i64, b: i64) -> i64`
Computes the greatest common divisor using the Euclidean algorithm.

**Algorithm:**
```rust
let mut a = a.abs();
let mut b = b.abs();
while b != 0 {
    let t = b;
    b = a % b;
    a = t;
}
a  // GCD
```

**Time Complexity:** O(log(min(a, b)))

**Examples:**
```rust
gcd(48, 18)  // Returns: 6
gcd(17, 23)  // Returns: 1 (coprime)
gcd(0, 5)    // Returns: 5
```

#### `lcm(a: i64, b: i64) -> i64`
Computes the least common multiple using the formula: `lcm(a,b) = |a/b| * gcd(a,b)`

**Edge Cases:**
- Returns 0 if either argument is 0

**Examples:**
```rust
lcm(21, 6)  // Returns: 42
lcm(5, 7)   // Returns: 35
lcm(0, 5)   // Returns: 0
```

#### `powmod(base: u64, exp: u64, modulus: u64) -> u64`
Computes `(base^exp) mod modulus` using binary exponentiation.

**Algorithm (Square-and-Multiply):**
```rust
while exp > 0 {
    if exp % 2 == 1 {
        result = (result * base) % modulus;
    }
    exp /= 2;
    base = (base * base) % modulus;
}
```

**Time Complexity:** O(log exp)

**Cryptographic Applications:**
- RSA encryption/decryption
- Diffie-Hellman key exchange
- Digital signatures

---

## Parity Module (`number_theory/parity.rs`)

### Functions

| Function | Description | Returns |
|----------|-------------|---------|
| `is_even` | Check if integer is even | `bool` |
| `is_odd` | Check if integer is odd | `bool` |
| `is_even_i128` | Check i128 parity | `bool` |
| `is_odd_i128` | Check i128 parity | `bool` |

### Implementation
```rust
pub fn is_even(n: i64) -> bool {
    n % 2 == 0
}

pub fn is_odd(n: i64) -> bool {
    n % 2 != 0
}
```

**Mathematical Definition:**
- n is even if n = 2k for some integer k
- n is odd if n = 2k + 1 for some integer k

---

## Primes Module (`number_theory/primes.rs`)

### Functions Overview

| Function | Description | Signature |
|----------|-------------|-----------|
| `is_prime` | Primality test (Miller-Rabin for large) | `u64 -> bool` |
| `next_prime` | Find smallest prime ≥ n | `i64 -> i64` |
| `prev_prime` | Find largest prime ≤ n | `i64 -> i64` |
| `nth_prime` | Get the nth prime (1-indexed) | `u64 -> u64` |
| `prime_count` | Count primes ≤ x (π function) | `i64 -> i64` |
| `primes_up_to` | Sieve of Eratosthenes | `i64 -> Vec<i64>` |
| `bernoulli` | Compute Bernoulli number B_n | `u64 -> Option<(i64,i64)>` |

### Algorithm Details

#### `is_prime(n: u64) -> bool`
Hybrid approach:
1. **Small numbers (< 1,000,000)**: Trial division with 6k±1 optimization
2. **Large numbers**: Miller-Rabin with 12 deterministic bases

**Miller-Rabin Witnesses for u64:**
```rust
const WITNESSES: &[u64] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
```

These bases are proven deterministic for all 64-bit integers.

#### `primes_up_to(n: i64) -> Vec<i64>`
Implements the Sieve of Eratosthenes:

```rust
let mut sieve = vec![true; n + 1];
sieve[0] = false;
sieve[1] = false;

for i in 2..=sqrt(n) {
    if sieve[i] {
        for j in (i * i..=n).step_by(i) {
            sieve[j] = false;
        }
    }
}
```

**Time Complexity:** O(n log log n)
**Space Complexity:** O(n)

#### `nth_prime(n: u64) -> u64`
Two-phase approach:
1. **n ≤ 10,000**: Lookup from precomputed table
2. **n > 10,000**: Use Prime Number Theorem approximation:
   ```
   p_n ≈ n(ln(n) + ln(ln(n)))
   ```

#### `bernoulli(n: u64) -> Option<(i64, i64)>`
Computes Bernoulli numbers using the Akiyama-Tanigawa algorithm:

```rust
let mut a: Vec<(i64, i64)> = (0..=m).map(|j| (1, j as i64 + 1)).collect();

for m_curr in 1..=m {
    for k in (1..=m_curr).rev() {
        // a[k-1] = k * (a[k-1] - a[k])
        let diff_num = num1 * den2 - num2 * den1;
        let new_num = k * diff_num;
        a[k - 1] = simplify(new_num, diff_den);
    }
}
```

**Special Cases:**
- B₀ = 1
- B₁ = -1/2
- Bₙ = 0 for odd n > 1

---

## Rounding Module (`number_theory/rounding.rs`)

### Rounding Modes

```rust
pub enum RoundingMode {
    HalfToEven,        // 0: Banker's rounding
    HalfAwayFromZero,  // 1: Round away from zero at 0.5
    HalfTowardZero,    // 2: Round toward zero at 0.5
    HalfUp,           // 3: Always round up at 0.5
    HalfDown,         // 4: Always round down at 0.5
    Up,               // 5: Ceiling (toward +∞)
    Down,             // 6: Floor (toward -∞)
    TowardZero,       // 7: Truncation
    AwayFromZero,     // 8: Away from zero
}
```

### Functions

| Function | Description | Signature |
|----------|-------------|-----------|
| `abs` | Absolute value of rational | `(i64, i64) -> (i64, i64)` |
| `abs_integer` | Absolute value of integer | `i64 -> i64` |
| `ceil` | Ceiling (round toward +∞) | `(i64, i64) -> i64` |
| `floor` | Floor (round toward -∞) | `(i64, i64) -> i64` |
| `trunc` | Truncate (toward zero) | `(i64, i64) -> i64` |
| `round` | Round with specified mode | `(i64, i64, RoundingMode) -> i64` |
| `signum` | Sign of a number (-1, 0, 1) | `i64 -> i64` |

### Implementation Details

#### `ceil(numerator: i64, denominator: i64) -> i64`
```rust
let mut result = numerator / denominator;
let remainder = numerator % denominator;

if remainder > 0 && denominator > 0 || remainder < 0 && denominator < 0 {
    result += 1;
}
result
```

#### `round(numerator: i64, denominator: i64, mode: RoundingMode) -> i64`
Uses floating-point conversion for precise fractional comparison:

```rust
let integer_part = numerator / denominator;
let remainder = numerator.abs() % denominator.abs();
let fraction = (remainder as f64) / (denominator.abs() as f64);

match mode {
    RoundingMode::HalfToEven => {
        if fraction > 0.5 {
            integer_part + signum(integer_part)
        } else if fraction < 0.5 {
            integer_part
        } else {
            // At exactly 0.5, round to nearest even
            if integer_part % 2 == 0 { integer_part }
            else { integer_part + signum(integer_part) }
        }
    }
    // ... other modes
}
```

---

## Totient Module (`number_theory/totient.rs`)

### Euler's Totient Function

```rust
pub fn totient(n: i64) -> i64
```

**Algorithm (using prime factorization):**
```rust
let mut result = n;
let mut n_abs = n.abs();

// For each distinct prime factor p
if n_abs % 2 == 0 {
    while n_abs % 2 == 0 { n_abs /= 2; }
    result -= result / 2;  // Apply: result *= (1 - 1/2)
}

// Check odd factors
for i in (3..=sqrt(n)).step_by(2) {
    if n_abs % i == 0 {
        while n_abs % i == 0 { n_abs /= i; }
        result -= result / i;  // Apply: result *= (1 - 1/i)
    }
}

// If remainder > 1, it's prime
if n_abs > 1 {
    result -= result / n_abs;
}

result
```

**Time Complexity:** O(√n)

**Mathematical Formula:**
```
φ(n) = n × Π(1 - 1/p_i) for all distinct prime factors p_i
```

---

## Traits Module (`number_theory/traits.rs`)

### Extension Traits for i64/i128

```rust
pub trait Divisors {
    fn divisors(&self) -> Vec<i64>;
}

pub trait Gcd<T> {
    fn gcd(&self, other: &T) -> i64;
}

pub trait Lcm<T> {
    fn lcm(&self, other: &T) -> i64;
}
```

### Usage Example

```rust
use tcal_rs::number_theory::traits::{Divisors, Gcd, Lcm};

let n = 12i64;
let divisors = n.divisors();  // [1, 2, 3, 4, 6, 12]

let a = 48i64;
let b = 18i64;
let gcd = a.gcd(&b);  // 6
let lcm = a.lcm(&b);  // 144
```

### Implementation Notes

**Divisors Algorithm:**
```rust
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
```

**Time Complexity:** O(√n)

---

## Calculator Module (`calculator/`)

### Architecture Overview

```
Input String
    ↓
[Lexer] → Tokens
    ↓
[Parser] → AST (Abstract Syntax Tree)
    ↓
[Preprocessor] → Handle hex/bin/oct
    ↓
[Evaluator] → Numeric Result
    ↓
[Formatter] → Formatted Output
```

### Lexer (`lexer.rs`)

**Token Types:**
```rust
pub enum Token {
    Number(f64),
    Ident(String),
    Plus, Minus, Mul, Div, Pow,
    And, Or, Shl, Shr,
    Assign,
    LParen, RParen, Comma,
}
```

**Supported Operators:**
- Arithmetic: `+`, `-`, `*`, `/`, `^`
- Bitwise: `&` (AND), `|` (OR), `<<`, `>>`
- Assignment: `=`
- Grouping: `(`, `)`, `,`

### Parser (`parser.rs`)

**AST Definition:**
```rust
pub enum Expr {
    Number(f64),
    Variable(String),
    Assign { name: String, expr: Box<Expr> },
    Unary { op: UnaryOp, expr: Box<Expr> },
    Binary { left: Box<Expr>, op: BinaryOp, right: Box<Expr> },
    Call { name: String, args: Vec<Expr> },
}
```

**Operator Precedence (Highest to Lowest):**
1. Function calls
2. Power (`^`)
3. Unary minus
4. Multiplication/Division (`*`, `/`)
5. Addition/Subtraction (`+`, `-`)
6. Bitwise shifts (`<<`, `>>`)
7. Bitwise AND (`&`)
8. Bitwise OR (`|`)
9. Assignment (`=`)

### Evaluator (`evaluator.rs`)

**Built-in Constants:**
- `pi` → π (3.14159...)
- `e` → e (2.71828...)
- `res` → Previous result

**Built-in Functions:**
| Category | Functions |
|----------|-----------|
| Trigonometric | `sin`, `cos`, `tan`, `asin`, `acos`, `atan` |
| Roots | `sqrt`, `cbrt` |
| Logarithms | `ln`, `log`, `log10` |
| Other | `abs`, `exp` |
| Number Theory | `totient`, `gcd`, `lcm` |

### Engine (`engine.rs`)

**Features:**
1. **Hexadecimal Support:** `0x1A` → 26
2. **Binary Support:** `0b1010` → 10
3. **Octal Support:** `0o755` → 493
4. **Variable Storage:** `x = 5`
5. **Result Reference:** `res` holds last result

### Formatter (`formatter.rs`)

**Output Format:**
```
        42.000000
━━━━━━━━━━━━━━━━━━━━━━━━━━━━
HEX : "0x2A"
DEC : "42"
OCT : "0o52"
BIN : "0010 1010"
11111111111111111111111111111111 11111111111111111111111111010110
63                      47                  32

11111111111111111111111111010110
31                      15                   0
```

**Features:**
- Thousand separators in decimal
- Hexadecimal, octal, binary representations
- 64-bit binary visualization
- 4-bit grouping for readability

### Converter (`converter.rs`)

**Unicode Conversion:**
```
Input: "to unicode A"
Output: [0] 'A' → U+0041

Input: "Hello"
Output:
[0] 'H' → U+0048
[1] 'e' → U+0065
[2] 'l' → U+006C
[3] 'l' → U+006C
[4] 'o' → U+006F
```

---

## Price Formatter (`fprice.rs`)

### Functionality

Adds thousand separators to integers:

```rust
PriceFormatter::format(1234567)  // "1,234,567"
PriceFormatter::format(42)       // "42"
```

**Implementation:**
```rust
for (i, c) in chars.iter().enumerate() {
    if i > 0 && (chars.len() - i).is_multiple_of(3) {
        result.push(',');
    }
    result.push(*c);
}
```

---

## Usage Examples

### Number Theory Operations

```rust
use tcal_rs::*;

// GCD and LCM
let gcd_val = gcd(48, 18);   // 6
let lcm_val = lcm(21, 6);    // 42

// Modular arithmetic
let remainder = powmod(4, 13, 497);  // 445

// Totient
let phi = totient(30);  // 8

// Prime operations
let prime = is_prime(7919);        // true
let next = next_prime(100);        // 101
let count = prime_count(100);      // 25
```

### Calculator Usage

```rust
use tcal_rs::calculator::Engine;

let mut engine = Engine::new();

// Basic arithmetic
engine.eval("2 + 2");          // "4"
engine.eval("2^10");           // "1024"

// Trigonometry
engine.eval("sin(pi/2)");      // "1"

// Number theory
engine.eval("totient(30)");    // "8"
engine.eval("gcd(48, 18)");    // "6"

// Variables
engine.eval("x = 5");          // "5"
engine.eval("x * 2 + 3");      // "13"

// Different bases
engine.eval("0xFF + 1");       // "256"
engine.eval("0b1010 + 0b10");  // "12"
```

---

## Performance Considerations

| Operation | Time Complexity | Space Complexity |
|-----------|-----------------|------------------|
| GCD (Euclidean) | O(log min(a,b)) | O(1) |
| LCM | O(log min(a,b)) | O(1) |
| Totient | O(√n) | O(1) |
| Miller-Rabin | O(k log³n) | O(1) |
| Sieve of Eratosthenes | O(n log log n) | O(n) |
| Divisors | O(√n) | O(√n) |
| Modular Exponentiation | O(log exp) | O(1) |

---

## Thread Safety

Most functions in the library are pure functions with no shared state, making them safe to use in multi-threaded contexts. The `Engine` struct maintains internal state (variables, last result) and should not be shared between threads without synchronization.

---

## Error Handling

- **Calculator functions**: Return `Result<String, String>` for error propagation
- **Number theory functions**: May panic on invalid inputs (e.g., division by zero)
- **Parser**: Returns `Result<Expr, String>` with descriptive error messages
