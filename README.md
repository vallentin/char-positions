# char-positions

[![CI](https://github.com/vallentin/char-positions/actions/workflows/rust.yml/badge.svg)](https://github.com/vallentin/char-positions/actions/workflows/rust.yml)
[![Latest Version](https://img.shields.io/crates/v/char-positions.svg)](https://crates.io/crates/char-positions)
[![Docs](https://docs.rs/char-positions/badge.svg)](https://docs.rs/char-positions)
[![dependency status](https://deps.rs/repo/github/vallentin/char-positions/status.svg)](https://deps.rs/repo/github/vallentin/char-positions)
[![License](https://img.shields.io/github/license/vallentin/char-positions.svg)](https://github.com/vallentin/char-positions)

<!-- cargo-rdme start -->

Similar to the standard library's [`.char_indicies()`], but instead of only
producing the start byte position. This library implements [`.char_positions()`],
which can produce any combination of line, column, start byte, and end byte position.

As an example use
<code>text.[char_positions]::&lt;[LineCol]&gt;()</code>
to get the line and column of each [`char`].
Use [`LineColByteRange`] to additionally get the byte range,
or just [`Line`] to simply get the line number.

### Example

```rust
use char_positions::{CharPositionsExt, LineCol};

let text = "Hello 👋\nWorld 🌏\n🦀🦀";

for (LineCol(line, col), c) in text.char_positions() {
    println!("[Ln {line}, Col {col}] {c:?}");
}
```

Which will output:

```text
[Ln 1, Col 1] 'H'
[Ln 1, Col 2] 'e'
[Ln 1, Col 3] 'l'
[Ln 1, Col 4] 'l'
[Ln 1, Col 5] 'o'
[Ln 1, Col 6] ' '
[Ln 1, Col 7] '👋'
[Ln 1, Col 8] '\n'
[Ln 2, Col 1] 'W'
[Ln 2, Col 2] 'o'
[Ln 2, Col 3] 'r'
[Ln 2, Col 4] 'l'
[Ln 2, Col 5] 'd'
[Ln 2, Col 6] ' '
[Ln 2, Col 7] '🌏'
[Ln 2, Col 8] '\n'
[Ln 3, Col 1] '🦀'
[Ln 3, Col 2] '🦀'
```

### Supported

| `.char_positions::<T>()` | Produces |
|:---|---|
| [`usize`] | Start byte index (same as [`.char_indicies()`]) |
| [`std::ops::Range<usize>`] | Start byte and end byte index, i.e. `&text[range]` is the `char` |
| [`LineColByteRange`] | Line number, column number, and byte range |
| [`LineCol`] | Line number and column number |
| [`Line`] | Line number |
| [`Col`] | Column number |
| [`ByteRange`] | Same as using [`std::ops::Range<usize>`] |
| [`ByteStart`] | Start byte index (same as [`.char_indicies()`]) |
| [`ByteEnd`] | End byte index |
| _Tuples are also supported, e.g._ | |
| <code>([Line],)</code> | _Produces the tuple_ |
| <code>([Line], [Col])</code> | _Produces the tuple_ |
| <code>([Line], [Col], [ByteStart], [ByteEnd])</code> | _Produces the tuple_ |
| _etc._ | |

### Example - `LineColByteRange`

```rust
use char_positions::{CharPositionsExt, LineColByteRange};

let text = "Hello 👋\nWorld 🌏\n🦀🦀";

let mut iter = text
    .char_positions::<LineColByteRange>()
    .map(|(LineColByteRange(line, col, range), c)| (line, col, range, c));

assert_eq!(iter.next(), Some((1, 1, 0..1, 'H')));
assert_eq!(iter.next(), Some((1, 2, 1..2, 'e')));
assert_eq!(iter.next(), Some((1, 3, 2..3, 'l')));
assert_eq!(iter.next(), Some((1, 4, 3..4, 'l')));
assert_eq!(iter.next(), Some((1, 5, 4..5, 'o')));
assert_eq!(iter.next(), Some((1, 6, 5..6, ' ')));
assert_eq!(iter.next(), Some((1, 7, 6..10, '👋')));
assert_eq!(iter.next(), Some((1, 8, 10..11, '\n')));
assert_eq!(iter.next(), Some((2, 1, 11..12, 'W')));
assert_eq!(iter.next(), Some((2, 2, 12..13, 'o')));
assert_eq!(iter.next(), Some((2, 3, 13..14, 'r')));
assert_eq!(iter.next(), Some((2, 4, 14..15, 'l')));
assert_eq!(iter.next(), Some((2, 5, 15..16, 'd')));
assert_eq!(iter.next(), Some((2, 6, 16..17, ' ')));
assert_eq!(iter.next(), Some((2, 7, 17..21, '🌏')));
assert_eq!(iter.next(), Some((2, 8, 21..22, '\n')));
assert_eq!(iter.next(), Some((3, 1, 22..26, '🦀')));
assert_eq!(iter.next(), Some((3, 2, 26..30, '🦀')));
assert_eq!(iter.next(), None);
```


[`.char_positions()`]: https://docs.rs/char-positions/*/char_positions/trait.CharPositionsExt.html#tymethod.char_positions
[char_positions]: https://docs.rs/char-positions/*/char_positions/trait.CharPositionsExt.html#tymethod.char_positions

[`LineColByteRange`]: https://docs.rs/char-positions/*/char_positions/struct.LineColByteRange.html
[`LineCol`]: https://docs.rs/char-positions/*/char_positions/struct.LineCol.html
[`Line`]: https://docs.rs/char-positions/*/char_positions/struct.Line.html
[`Col`]: https://docs.rs/char-positions/*/char_positions/struct.Line.html
[`ByteRange`]: https://docs.rs/char-positions/*/char_positions/struct.ByteRange.html
[`ByteStart`]: https://docs.rs/char-positions/*/char_positions/struct.ByteStart.html
[`ByteEnd`]: https://docs.rs/char-positions/*/char_positions/struct.ByteEnd.html

[LineColByteRange]: https://docs.rs/char-positions/*/char_positions/struct.LineColByteRange.html
[LineCol]: https://docs.rs/char-positions/*/char_positions/struct.LineCol.html
[Line]: https://docs.rs/char-positions/*/char_positions/struct.Line.html
[Col]: https://docs.rs/char-positions/*/char_positions/struct.Col.html
[ByteStart]: https://docs.rs/char-positions/*/char_positions/struct.ByteStart.html
[ByteEnd]: https://docs.rs/char-positions/*/char_positions/struct.ByteEnd.html

[`.char_indicies()`]: https://doc.rust-lang.org/std/primitive.str.html#method.char_indices

[`char`]: https://doc.rust-lang.org/std/primitive.char.html
[`usize`]: https://doc.rust-lang.org/std/primitive.usize.html
[`std::ops::Range<usize>`]: https://doc.rust-lang.org/std/ops/struct.Range.html

<!-- cargo-rdme end -->
