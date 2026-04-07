# tcal_rs
- Converting C++ Code to Rust Code 
  - https://github.com/Qalculate/libqalculate

# `tcal_rs`

```bash
$ cargo r --release

Qalculate CLI - Interactive Calculator
Type 'exit' or 'quit' to exit

Supported: sqrt(72), 2^3 + 5, sin(pi), 133 to hex, etc.
> sqrt(72)
8.48528137423857
> 2^3
8
> 2^16
65536
> 2^32
4294967296
> 2^3
8
> 8 to hex
0x8
> 8 to binary
0b1000

> 1024 to binary
0b0100 0000 0000

> exit
Goodbye!

```

# 2진법으로 확인

```bash
Qalculate CLI - Interactive Calculator
Type 'exit' or 'quit' to exit

Supported: sqrt(72), 2^3 + 5, sin(pi), 133 to hex, etc.

> 0x1f3d + 0x1ffa
                16,183
         ━━━━━━━━━━━━━━━━━━━━━━━━━━━━
HEX : "0x3F37"
DEC : "16,183"
OCT : "0o37467"
BIN : "0b0011 1111 0011 0111"
0000  0000  0000  0000  0000  0000  0000  0000
63                      47                  32

0000  0000  0000  0000  0011  1111  0011  0111
31                      15                   0


> 0x132a
                4,906
         ━━━━━━━━━━━━━━━━━━━━━━━━━━━━
HEX : "0x132A"
DEC : "4,906"
OCT : "0o11452"
BIN : "0b0001 0011 0010 1010"
0000  0000  0000  0000  0000  0000  0000  0000
63                      47                  32

0000  0000  0000  0000  0001  0011  0010  1010
31                      15                   0


> 0b1111001
                121
         ━━━━━━━━━━━━━━━━━━━━━━━━━━━━
HEX : "0x79"
DEC : "121"
OCT : "0o171"
BIN : "0b0111 1001"
0000  0000  0000  0000  0000  0000  0000  0000
63                      47                  32

0000  0000  0000  0000  0000  0000  0111  1001
31                      15                   0


> 0o3434
                1,820
         ━━━━━━━━━━━━━━━━━━━━━━━━━━━━
HEX : "0x71C"
DEC : "1,820"
OCT : "0o3434"
BIN : "0b0111 0001 1100"
0000  0000  0000  0000  0000  0000  0000  0000
63                      47                  32

0000  0000  0000  0000  0000  0111  0001  1100
31                      15                   0
```
