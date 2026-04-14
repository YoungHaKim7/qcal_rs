# How many bytes does one Unicode character take?

- Strangely enough, nobody pointed out how to calculate how many bytes is taking one Unicode char. Here is the rule for UTF-8 encoded strings:

```bash
Binary    Hex          Comments
0xxxxxxx  0x00..0x7F   Only byte of a 1-byte character encoding
10xxxxxx  0x80..0xBF   Continuation byte: one of 1-3 bytes following the first
110xxxxx  0xC0..0xDF   First byte of a 2-byte character encoding
1110xxxx  0xE0..0xEF   First byte of a 3-byte character encoding
11110xxx  0xF0..0xF7   First byte of a 4-byte character encoding
```

- So the quick answer is: it takes 1 to 4 bytes, depending on the first one which will indicate how many bytes it'll take up.
- https://stackoverflow.com/questions/5290182/how-many-bytes-does-one-unicode-character-take
