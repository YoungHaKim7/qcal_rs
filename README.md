# qcal_rs
https://github.com/Qalculate/libqalculate


# `qcal_rs`

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
> 2^7
128

0000  0000  0000  0000  0000  0000  0000  0000
63                      47                  32

0000  0000  0000  0000  0000  0000  1000  0000
31                      15                  0
> 255
255

0000  0000  0000  0000  0000  0000  0000  0000
63                      47                  32

0000  0000  0000  0000  0000  0000  1111  1111
31                      15                  0
> 254
254

0000  0000  0000  0000  0000  0000  0000  0000
63                      47                  32

0000  0000  0000  0000  0000  0000  1111  1110
31                      15                  0

```
