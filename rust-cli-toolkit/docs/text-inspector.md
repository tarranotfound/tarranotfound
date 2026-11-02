# Text inspector

The text inspector reads one line and reports three properties:

1. The text in reverse character order
2. The number of Unicode scalar values, counted with `chars()`
3. The number of whitespace-separated words, counted with `split_whitespace()`

The reverse operation works on Rust `char` values rather than raw UTF-8 bytes, avoiding invalid UTF-8 output when text contains multibyte characters.

Note: a Rust `char` is not necessarily a complete user-perceived grapheme. Some combined emoji and accented sequences can contain multiple `char` values.