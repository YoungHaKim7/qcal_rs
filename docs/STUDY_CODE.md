# Lexer / Parser / AST / Evaluator (compiler-style)

- 🧠 FINAL ENGINE DESIGN
```txt
Lexer → Tokens → Parser → AST → Evaluator (with Context)
```

# We’ll move your calculator to a compiler-style architecture:

```txt
Input
 → Lexer (tokenize)
 → Parser (Pratt parser)
 → AST
 → Evaluator
 → Result
```
  

# 🧠 FINAL STRUCTURE

```bash
src/
├── main.rs
├── engine/
│   ├── mod.rs
│   ├── token.rs
│   ├── lexer.rs
│   ├── parser.rs
│   ├── ast.rs
│   └── evaluator.rs
```

<hr />

<br />

<hr />

# 🧠 What that means (important)

- Right now your code is:
  - String-based processing
  - Uses meval + manual preprocessing
  - Works like:

```txt
"0x1f + 2" → preprocess → eval_str(...)
```

- But a compiler-style architecture would be:

```txt
Input → Lexer → Tokens → Parser → AST → Evaluator

```

- Example:

```txt
2^3 + 5
↓
[2, ^, 3, +, 5]      (tokens)
↓
(+ (^ 2 3) 5)        (AST)
↓
13                   (evaluation)
```

- 👉 This gives:
  - better control
  - cleaner operator precedence
  - easier feature extension

  - BUT ❗
  - You explicitly said:

> ❌ Don't change code
> ✅ Only add functions

- So we will NOT replace your system, only extend it.

# 🚀 NEW FEATURES (Added via Functions Only)

- Here are safe add-ons that plug into your current system:

## ✅ 1. Signed / Unsigned View

```txt
> 0xFF to signed
→ -1 (8-bit)
```

## ✅ 2. Bit Width Views (8 / 16 / 32 / 64)

```txt
> 255 to bin8
> 255 to bin16
```

## ✅ 3. ASCII Conversion

```txt
> 65 to ascii
→ 'A'
```

## ✅ 4. Reverse Unicode (char → number already exists, now reverse)

```txt
> 65 to unicode
→ U+0041 → 'A'
```

## ✅ 5. Bit Count (popcount)

```txt
> 0xFF to bits
→ 8
```
## ✅ 6. Leading / Trailing Zeros

```txt
> 16 to lz
> 16 to tz
```

# 🧩 ADD THESE FUNCTIONS (ONLY)

## 🔹 1. Signed Conversion

```rs
fn convert_signed(value: i64, bits: u8) -> i64 {
    let shift = 64 - bits;
    (value << shift) >> shift
}
```

## 🔹 2. Bit Width Binary

```rs
fn format_bin_width(value: i64, bits: usize) -> String {
    let masked = value & ((1i64 << bits) - 1);
    let s = format!("{:0width$b}", masked, width = bits);

    s.chars()
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}
```

## 🔹 3. ASCII Conversion

```rs
fn to_ascii(value: i64) -> Result<String, String> {
    if value >= 0 && value <= 127 {
        Ok(format!("'{}'", value as u8 as char))
    } else {
        Err("Out of ASCII range".into())
    }
}
```

## 🔹 4. Bit Count (Popcount)

```rs
fn count_bits(value: i64) -> u32 {
    value.count_ones()
}
```

## 🔹 5. Leading / Trailing Zeros

```rs
fn leading_zeros(value: i64) -> u32 {
    value.leading_zeros()
}

fn trailing_zeros(value: i64) -> u32 {
    value.trailing_zeros()
}
```

# 🔧 EXTEND convert_result ONLY

- 👉 Add these match arms:

```rs
"signed8" => Ok(format!("{}", convert_signed(value, 8))),
"signed16" => Ok(format!("{}", convert_signed(value, 16))),
"signed32" => Ok(format!("{}", convert_signed(value, 32))),

"bin8" => Ok(format_bin_width(value, 8)),
"bin16" => Ok(format_bin_width(value, 16)),
"bin32" => Ok(format_bin_width(value, 32)),
"bin64" => Ok(format_bin_width(value, 64)),

"ascii" => to_ascii(value),

"bits" => Ok(format!("{}", count_bits(value))),
"lz" => Ok(format!("{}", leading_zeros(value))),
"tz" => Ok(format!("{}", trailing_zeros(value))),
```

## 💡 OPTIONAL (Very Powerful)
- Add this helper (no core change)


```rs
fn detect_auto_base(input: &str) -> Option<i64> {
    if input.starts_with("0x") {
        i64::from_str_radix(&input[2..], 16).ok()
    } else if input.starts_with("0b") {
        i64::from_str_radix(&input[2..], 2).ok()
    } else if input.starts_with("0o") {
        i64::from_str_radix(&input[2..], 8).ok()
    } else {
        None
    }
}
```

## 🧪 NEW COMMANDS YOU NOW GET

```bash
255 to ascii
255 to bits
255 to lz
255 to tz
255 to bin16
255 to signed8
0xFF to signed8
```

# 🧠 Summary

- You now have:
  - ✅ No architecture change
  - ✅ Still using meval
  - ✅ More CLI power
  - ✅ Bit-level analysis tools
  - ✅ Encoding tools

<br />

<hr />

<br />

<hr />

# Code
- Done! I've successfully split the code by function into three modules:

**1. `src/unicode/mod.rs`** - UTF-8 character handling
- `read_utf8_char()` - Reads a single UTF-8 character from stdin
- `utf8_char_width()` - Determines byte width of UTF-8 character

**2. `src/readline/mod.rs`** (NEW) - Interactive readline with terminal control
- `set_raw_mode()` - Terminal raw/cooked mode control
- `read_single_byte()` - Low-level byte reading
- `readline_with_history()` - Full readline implementation with:
  - History navigation (up/down arrows)
  - Cursor movement (left/right arrows)
  - Backspace handling
  - Ctrl+C/Ctrl+D exit

**3. `src/save_history/mod.rs`** - Simplified to only file I/O
- `load_history()` - Load command history from file
- `save_history()` - Save command history to file

**Changes made:**
- Created new `readline/` module
- Moved terminal and readline logic from `save_history/` to `readline/`
- Simplified `save_history/` to only contain file operations
- Updated `lib.rs` to include the `readline` module
- Updated `main.rs` imports to use the new module structure

The project builds successfully with `cargo build`.
