# Mathematical Formulas Reference

This document provides detailed mathematical formulas and explanations for all number theory and mathematical operations in the tcal_rs library.

---

## Table of Contents

1. [Greatest Common Divisor (GCD)](#greatest-common-divisor-gcd)
2. [Least Common Multiple (LCM)](#least-common-multiple-lcm)
3. [Euler's Totient Function](#eulers-totient-function)
4. [Modular Arithmetic](#modular-arithmetic)
5. [Rounding Operations](#rounding-operations)
6. [Prime Numbers](#prime-numbers)
7. [Bernoulli Numbers](#bernoulli-numbers)
8. [Trigonometric Functions](#trigonometric-functions)

---

## Greatest Common Divisor (GCD)

### Definition
The greatest common divisor of two integers a and b, denoted gcd(a,b), is the largest positive integer that divides both a and b without leaving a remainder.

### Euclidean Algorithm
```
gcd(a, b) = gcd(b, a mod b)
gcd(a, 0) = |a|
```

### Example
```
gcd(48, 18):
48 = 18 × 2 + 12
18 = 12 × 1 + 6
12 = 6 × 2 + 0
Therefore: gcd(48, 18) = 6
```

### Properties
- **Commutative**: gcd(a, b) = gcd(b, a)
- **Associative**: gcd(a, gcd(b, c)) = gcd(gcd(a, b), c)
- **Distribution**: gcd(a, lcm(b, c)) = lcm(gcd(a, b), gcd(a, c))
- **Identity**: gcd(a, 0) = |a|

---

## Least Common Multiple (LCM)

### Definition
The least common multiple of two integers a and b, denoted lcm(a,b), is the smallest positive integer that is divisible by both a and b.

### Formula
```
lcm(a, b) = |a × b| / gcd(a, b)
lcm(a, 0) = 0
```

### Example
```
lcm(21, 6):
gcd(21, 6) = 3
lcm(21, 6) = (21 × 6) / 3 = 42
```

### Properties
- **Commutative**: lcm(a, b) = lcm(b, a)
- **Associative**: lcm(a, lcm(b, c)) = lcm(lcm(a, b), c)
- **Distribution**: lcm(a, gcd(b, c)) = gcd(lcm(a, b), lcm(a, c))
- **Identity**: lcm(a, 1) = |a|

---

## Euler's Totient Function

### Definition
Euler's totient function φ(n) counts the positive integers up to n that are relatively prime to n (i.e., gcd(k, n) = 1 for 1 ≤ k ≤ n).

### Formula using Prime Factorization
If n has the prime factorization:
```
n = p₁^k₁ × p₂^k₂ × ... × p_m^k_m
```

Then:
```
φ(n) = n × Π(1 - 1/p_i) for all distinct prime factors p_i
```

### Alternative Form
```
φ(n) = Π(p_i^k_i - p_i^(k_i-1)) for all prime factors p_i
```

### Examples
```
φ(7) = 7 × (1 - 1/7) = 6        [7 is prime]
φ(9) = 9 × (1 - 1/3) = 6        [9 = 3²]
φ(10) = 10 × (1 - 1/2)(1 - 1/5) = 4
φ(30) = 30 × (1 - 1/2)(1 - 1/3)(1 - 1/5) = 8
```

### Properties
- **Multiplicative**: If gcd(a, b) = 1, then φ(ab) = φ(a) × φ(b)
- **Prime**: If p is prime, φ(p) = p - 1
- **Power**: If p is prime, φ(p^k) = p^k - p^(k-1)

---

## Modular Arithmetic

### Modulo Operation
The modulo operation finds the remainder after division:
```
a mod n = r where a = qn + r and 0 ≤ r < n
```

### Modular Exponentiation (Binary Exponentiation)
Computes (base^exp) mod modulus efficiently:
```
powmod(base, exp, modulus):
    result = 1
    base = base mod modulus
    while exp > 0:
        if exp is odd:
            result = (result × base) mod modulus
        exp = exp / 2
        base = (base × base) mod modulus
    return result
```

### Example
```
powmod(4, 13, 497):
4^13 mod 497 = 445
```

### Remainder vs Modulo
- **Remainder**: `a % b` has the same sign as the dividend
  - `-7 % 3 = -1`
- **Modulo**: Always returns non-negative result
  - `modulo(-7, 3) = 2`

---

## Rounding Operations

### Types of Rounding

| Mode | Description | Example (2.5) |
|------|-------------|---------------|
| HalfToEven | Round to nearest even (Banker's) | 2 |
| HalfAwayFromZero | Round away from zero | 3 |
| HalfTowardZero | Round toward zero | 2 |
| HalfUp | Round up (toward +∞) | 3 |
| HalfDown | Round down (toward -∞) | 2 |
| Up | Ceiling (toward +∞) | 3 |
| Down | Floor (toward -∞) | 2 |
| TowardZero | Truncate | 2 |
| AwayFromZero | Away from zero | 3 |

### Floor Function
⌊x⌋ = greatest integer ≤ x

```
floor(7/3) = 2
floor(-7/3) = -3
```

### Ceiling Function
⌈x⌉ = smallest integer ≥ x

```
ceil(7/3) = 3
ceil(-7/3) = -2
```

### Truncation
trunc(x) = integer part of x (toward zero)

```
trunc(7/3) = 2
trunc(-7/3) = -2
```

---

## Prime Numbers

### Definition
A prime number is a natural number greater than 1 that has no positive divisors other than 1 and itself.

### Miller-Rabin Primality Test
For an odd number n > 2, write n-1 = d × 2^s where d is odd.

n is probably prime if for a base a:
```
a^d ≡ 1 (mod n) or
a^(d×2^r) ≡ -1 (mod n) for some 0 ≤ r < s
```

### Prime Number Theorem
The number of primes ≤ x (π function):
```
π(x) ≈ x / ln(x)
```

### nth Prime Approximation
```
p_n ≈ n(ln(n) + ln(ln(n)))
```

### Sieve of Eratosthenes
Algorithm to find all primes up to n:
1. Create boolean array of size n+1
2. Mark 0 and 1 as not prime
3. For each prime p starting from 2:
   - Mark all multiples of p as composite
4. Remaining unmarked numbers are prime

---

## Bernoulli Numbers

### Definition
Bernoulli numbers B_n are a sequence of rational numbers that appear in:
- Taylor series expansion of tan(x) and x/(e^x-1)
- Faulhaber's formula for sums of powers
- Euler-Maclaurin formula

### Values
```
B_0 = 1
B_1 = -1/2
B_2 = 1/6
B_4 = -1/30
B_6 = 1/42
B_8 = -1/30
B_odd>1 = 0
```

### Akiyama-Tanigawa Algorithm
```
Initialize: a[j] = 1/(j+1) for j = 0 to m

For m from 1 to n:
    For k from m down to 1:
        a[k-1] = k × (a[k-1] - a[k])

B_n = a[0]
```

### Faulhaber's Formula
Sum of p-th powers:
```
Σ(k^p) for k=1 to n = (1/(p+1)) × Σ(C(p+1,k) × B_k × n^(p+1-k))
```

---

## Trigonometric Functions

### Basic Functions
```
sin(θ) = opposite/hypotenuse
cos(θ) = adjacent/hypotenuse
tan(θ) = opposite/adjacent = sin(θ)/cos(θ)
```

### Inverse Functions
```
asin(y) = θ where sin(θ) = y and -π/2 ≤ θ ≤ π/2
acos(y) = θ where cos(θ) = y and 0 ≤ θ ≤ π
atan(y) = θ where tan(θ) = y and -π/2 < θ < π/2
```

### Hyperbolic Functions (via Bernoulli Numbers)
The Taylor series for tan(x) involves Bernoulli numbers:
```
tan(x) = Σ(B_2n × (-4)^n × (1-4^n) × x^(2n-1) / (2n)!)
```

---

## Additional Functions

### Signum Function
```
signum(x) = -1  if x < 0
signum(x) =  0  if x = 0
signum(x) =  1  if x > 0
```

### Absolute Value
```
|a/b| = (|a|, |b|)
|n| = n if n ≥ 0, -n if n < 0
```

### Divisors
For n = p₁^k₁ × p₂^k₂ × ... × p_m^k_m:
Number of divisors: (k₁+1)(k₂+1)...(k_m+1)

### Parity
```
is_even(n) = (n mod 2 = 0)
is_odd(n) = (n mod 2 = 1)
```
