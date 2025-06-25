//! Similar to the standard library's [`.char_indicies()`], but instead of only
//! producing the start byte position. This library implements [`.char_positions()`],
//! which can produce any combination of line, column, start byte, and end byte position.
//!
//! As an example use
//! <code>text.[char_positions]::&lt;[LineCol]&gt;()</code>
//! to get the line and column of each [`char`].
//! Use [`LineColByteRange`] to additionally get the byte range,
//! or just [`Line`] to simply get the line number.
//!
//! ## Example
//!
//! ```
//! use char_positions::{CharPositionsExt, LineCol};
//!
//! let text = "Hello 👋\nWorld 🌏\n🦀🦀";
//!
//! for (LineCol(line, col), c) in text.char_positions() {
//!     println!("[Ln {line}, Col {col}] {c:?}");
//! }
//! ```
//!
//! Which will output:
//!
//! ```text
//! [Ln 1, Col 1] 'H'
//! [Ln 1, Col 2] 'e'
//! [Ln 1, Col 3] 'l'
//! [Ln 1, Col 4] 'l'
//! [Ln 1, Col 5] 'o'
//! [Ln 1, Col 6] ' '
//! [Ln 1, Col 7] '👋'
//! [Ln 1, Col 8] '\n'
//! [Ln 2, Col 1] 'W'
//! [Ln 2, Col 2] 'o'
//! [Ln 2, Col 3] 'r'
//! [Ln 2, Col 4] 'l'
//! [Ln 2, Col 5] 'd'
//! [Ln 2, Col 6] ' '
//! [Ln 2, Col 7] '🌏'
//! [Ln 2, Col 8] '\n'
//! [Ln 3, Col 1] '🦀'
//! [Ln 3, Col 2] '🦀'
//! ```
//!
//! ## Supported
//!
//! | `.char_positions::<T>()` | Produces |
//! |:---|---|
//! | [`usize`] | Start byte index (same as [`.char_indicies()`]) |
//! | [`std::ops::Range<usize>`] | Start byte and end byte index, i.e. `&text[range]` is the `char` |
//! | [`LineColByteRange`] | Line number, column number, and byte range |
//! | [`LineCol`] | Line number and column number |
//! | [`Line`] | Line number |
//! | [`Col`] | Column number |
//! | [`ByteRange`] | Same as using [`std::ops::Range<usize>`] |
//! | [`ByteStart`] | Start byte index (same as [`.char_indicies()`]) |
//! | [`ByteEnd`] | End byte index |
//! | _Tuples are also supported, e.g._ | |
//! | <code>([Line],)</code> | _Produces the tuple_ |
//! | <code>([Line], [Col])</code> | _Produces the tuple_ |
//! | <code>([Line], [Col], [ByteStart], [ByteEnd])</code> | _Produces the tuple_ |
//! | _etc._ | |
//!
//! ## Example - `LineColByteRange`
//!
//! ```
//! use char_positions::{CharPositionsExt, LineColByteRange};
//!
//! let text = "Hello 👋\nWorld 🌏\n🦀🦀";
//!
//! let mut iter = text
//!     .char_positions::<LineColByteRange>()
//!     .map(|(LineColByteRange(line, col, range), c)| (line, col, range, c));
//!
//! assert_eq!(iter.next(), Some((1, 1, 0..1, 'H')));
//! assert_eq!(iter.next(), Some((1, 2, 1..2, 'e')));
//! assert_eq!(iter.next(), Some((1, 3, 2..3, 'l')));
//! assert_eq!(iter.next(), Some((1, 4, 3..4, 'l')));
//! assert_eq!(iter.next(), Some((1, 5, 4..5, 'o')));
//! assert_eq!(iter.next(), Some((1, 6, 5..6, ' ')));
//! assert_eq!(iter.next(), Some((1, 7, 6..10, '👋')));
//! assert_eq!(iter.next(), Some((1, 8, 10..11, '\n')));
//! assert_eq!(iter.next(), Some((2, 1, 11..12, 'W')));
//! assert_eq!(iter.next(), Some((2, 2, 12..13, 'o')));
//! assert_eq!(iter.next(), Some((2, 3, 13..14, 'r')));
//! assert_eq!(iter.next(), Some((2, 4, 14..15, 'l')));
//! assert_eq!(iter.next(), Some((2, 5, 15..16, 'd')));
//! assert_eq!(iter.next(), Some((2, 6, 16..17, ' ')));
//! assert_eq!(iter.next(), Some((2, 7, 17..21, '🌏')));
//! assert_eq!(iter.next(), Some((2, 8, 21..22, '\n')));
//! assert_eq!(iter.next(), Some((3, 1, 22..26, '🦀')));
//! assert_eq!(iter.next(), Some((3, 2, 26..30, '🦀')));
//! assert_eq!(iter.next(), None);
//! ```
//!
// Manually linking everything, as `cargo rdme` does not support intralinks
//!
//! [`.char_positions()`]: https://docs.rs/char-positions/*/char_positions/trait.CharPositionsExt.html#tymethod.char_positions
//! [char_positions]: https://docs.rs/char-positions/*/char_positions/trait.CharPositionsExt.html#tymethod.char_positions
//!
//! [`LineColByteRange`]: https://docs.rs/char-positions/*/char_positions/struct.LineColByteRange.html
//! [`LineCol`]: https://docs.rs/char-positions/*/char_positions/struct.LineCol.html
//! [`Line`]: https://docs.rs/char-positions/*/char_positions/struct.Line.html
//! [`Col`]: https://docs.rs/char-positions/*/char_positions/struct.Line.html
//! [`ByteRange`]: https://docs.rs/char-positions/*/char_positions/struct.ByteRange.html
//! [`ByteStart`]: https://docs.rs/char-positions/*/char_positions/struct.ByteStart.html
//! [`ByteEnd`]: https://docs.rs/char-positions/*/char_positions/struct.ByteEnd.html
//!
//! [LineColByteRange]: https://docs.rs/char-positions/*/char_positions/struct.LineColByteRange.html
//! [LineCol]: https://docs.rs/char-positions/*/char_positions/struct.LineCol.html
//! [Line]: https://docs.rs/char-positions/*/char_positions/struct.Line.html
//! [Col]: https://docs.rs/char-positions/*/char_positions/struct.Col.html
//! [ByteStart]: https://docs.rs/char-positions/*/char_positions/struct.ByteStart.html
//! [ByteEnd]: https://docs.rs/char-positions/*/char_positions/struct.ByteEnd.html
//!
//! [`.char_indicies()`]: https://doc.rust-lang.org/std/primitive.str.html#method.char_indices
//!
//! [`char`]: https://doc.rust-lang.org/std/primitive.char.html
//! [`usize`]: https://doc.rust-lang.org/std/primitive.usize.html
//! [`std::ops::Range<usize>`]: https://doc.rust-lang.org/std/ops/struct.Range.html

#![no_std]
#![forbid(unsafe_code)]
#![forbid(elided_lifetimes_in_paths)]

use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::ops::Range;

use char_ranges::{CharRanges, CharRangesExt};

pub trait CharPositionsExt {
    /// Returns an iterator over [`char`]s and their positions.
    ///
    /// See examples in the [crate root](crate).
    fn char_positions<T>(&self) -> CharPositions<'_, T>
    where
        LineColByteRange: Into<T>;
}

impl CharPositionsExt for str {
    #[inline]
    fn char_positions<T>(&self) -> CharPositions<'_, T>
    where
        LineColByteRange: Into<T>,
    {
        CharPositions::new(self)
    }
}

/// An iterator over [`char`]s and their positions.
///
/// Note: Cloning this iterator is essentially a copy.
///
/// See examples in the [crate root](crate).
#[derive(Clone, Debug)]
pub struct CharPositions<'a, T> {
    iter: CharRanges<'a>,
    pos: LineCol,
    phantom: PhantomData<T>,
}

impl<'a, T> CharPositions<'a, T> {
    #[inline]
    fn new(s: &'a str) -> Self {
        Self {
            iter: s.char_ranges(),
            pos: LineCol::START,
            phantom: PhantomData,
        }
    }

    /// Returns the remaining substring.
    #[inline]
    pub fn as_str(&self) -> &'a str {
        self.iter.as_str()
    }
}

impl<'a, T> Iterator for CharPositions<'a, T>
where
    LineColByteRange: Into<T>,
{
    type Item = (T, char);

    fn next(&mut self) -> Option<Self::Item> {
        let (r, c) = self.iter.next()?;
        let pos = LineColByteRange(self.pos.0, self.pos.1, r);

        match c {
            '\n' => {
                self.pos.0 += 1;
                self.pos.1 = 1;
            }
            _ => {
                self.pos.1 += 1;
            }
        }

        Some((pos.into(), c))
    }
}

impl<T> FusedIterator for CharPositions<'_, T> where Self: Iterator {}

/// `Line(line)`
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Line(
    /// 1-indexed line.
    pub usize,
);

/// `Col(col)`
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Col(
    /// 1-indexed column.
    pub usize,
);

/// `ByteStart(byte_start)`
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct ByteStart(
    /// The start (inclusive) byte positions.
    pub usize,
);

/// `ByteEnd(byte_end)`
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct ByteEnd(
    /// The end (exclusive) byte position.
    pub usize,
);

/// `ByteRange(byte_start..byte_end)`
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct ByteRange(
    /// The start (inclusive) and end (exclusive) byte positions.
    pub Range<usize>,
);

/// `LineCol(line, col)`
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct LineCol(
    /// 1-indexed line.
    pub usize,
    /// 1-indexed column.
    pub usize,
);

impl LineCol {
    const START: Self = Self(1, 1);

    #[inline]
    pub const fn line(&self) -> usize {
        self.0
    }

    #[inline]
    pub const fn column(&self) -> usize {
        self.1
    }
}

/// `LineColByte(line, col, byte_start)`
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct LineColByte(
    /// 1-indexed line.
    pub usize,
    /// 1-indexed column.
    pub usize,
    /// The start (inclusive) byte positions.
    pub usize,
);

impl LineColByte {
    #[inline]
    pub const fn line(&self) -> usize {
        self.0
    }

    #[inline]
    pub const fn column(&self) -> usize {
        self.1
    }

    /// Inclusive.
    #[doc(alias = "byte")]
    #[inline]
    pub const fn byte_start(&self) -> usize {
        self.2
    }
}

/// `LineColByteRange(line, col, byte_start..byte_end)`
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct LineColByteRange(
    /// 1-indexed line.
    pub usize,
    /// 1-indexed column.
    pub usize,
    /// The start (inclusive) and end (exclusive) byte positions.
    pub Range<usize>,
);

impl LineColByteRange {
    #[inline]
    pub const fn line(&self) -> usize {
        self.0
    }

    #[inline]
    pub const fn column(&self) -> usize {
        self.1
    }

    /// Inclusive.
    #[inline]
    pub const fn byte_start(&self) -> usize {
        self.2.start
    }

    /// Exclusive.
    #[inline]
    pub const fn byte_end(&self) -> usize {
        self.2.end
    }

    #[inline]
    pub const fn byte_range(&self) -> Range<usize> {
        self.2.start..self.2.end
    }
}

impl From<LineCol> for Line {
    #[inline]
    fn from(pos: LineCol) -> Self {
        Self(pos.0)
    }
}

impl From<LineCol> for Col {
    #[inline]
    fn from(pos: LineCol) -> Self {
        Self(pos.1)
    }
}

impl From<LineColByte> for Line {
    #[inline]
    fn from(pos: LineColByte) -> Self {
        Self(pos.0)
    }
}

impl From<LineColByte> for Col {
    #[inline]
    fn from(pos: LineColByte) -> Self {
        Self(pos.1)
    }
}

impl From<LineColByte> for LineCol {
    #[inline]
    fn from(pos: LineColByte) -> Self {
        Self(pos.0, pos.1)
    }
}

impl From<LineColByteRange> for Line {
    #[inline]
    fn from(pos: LineColByteRange) -> Self {
        Self(pos.0)
    }
}

impl From<LineColByteRange> for Col {
    #[inline]
    fn from(pos: LineColByteRange) -> Self {
        Self(pos.1)
    }
}

impl From<LineColByteRange> for ByteStart {
    #[inline]
    fn from(pos: LineColByteRange) -> Self {
        Self(pos.2.start)
    }
}

impl From<LineColByteRange> for ByteEnd {
    #[inline]
    fn from(pos: LineColByteRange) -> Self {
        Self(pos.2.end)
    }
}

impl From<LineColByteRange> for ByteRange {
    #[inline]
    fn from(pos: LineColByteRange) -> Self {
        Self(pos.2)
    }
}

impl From<LineColByteRange> for LineCol {
    #[inline]
    fn from(pos: LineColByteRange) -> Self {
        Self(pos.0, pos.1)
    }
}

impl From<LineColByteRange> for LineColByte {
    #[inline]
    fn from(pos: LineColByteRange) -> Self {
        Self(pos.0, pos.1, pos.2.start)
    }
}

impl From<LineColByteRange> for usize {
    #[inline]
    fn from(pos: LineColByteRange) -> Self {
        pos.2.start
    }
}

impl From<LineColByteRange> for Range<usize> {
    #[inline]
    fn from(pos: LineColByteRange) -> Self {
        pos.2
    }
}

impl From<ByteStart> for usize {
    #[inline]
    fn from(ByteStart(pos): ByteStart) -> Self {
        pos
    }
}

impl From<ByteEnd> for usize {
    #[inline]
    fn from(ByteEnd(pos): ByteEnd) -> Self {
        pos
    }
}

impl From<ByteRange> for Range<usize> {
    #[inline]
    fn from(ByteRange(r): ByteRange) -> Self {
        r
    }
}

impl<A> From<LineColByteRange> for (A,)
where
    LineColByteRange: Into<A>,
{
    #[inline]
    fn from(pos: LineColByteRange) -> Self {
        (pos.into(),)
    }
}

impl<A, B> From<LineColByteRange> for (A, B)
where
    LineColByteRange: Into<A>,
    LineColByteRange: Into<B>,
{
    #[inline]
    fn from(pos: LineColByteRange) -> Self {
        (pos.clone().into(), pos.into())
    }
}

impl<A, B, C> From<LineColByteRange> for (A, B, C)
where
    LineColByteRange: Into<A>,
    LineColByteRange: Into<B>,
    LineColByteRange: Into<C>,
{
    #[inline]
    fn from(pos: LineColByteRange) -> Self {
        (pos.clone().into(), pos.clone().into(), pos.into())
    }
}

impl<A, B, C, D> From<LineColByteRange> for (A, B, C, D)
where
    LineColByteRange: Into<A>,
    LineColByteRange: Into<B>,
    LineColByteRange: Into<C>,
    LineColByteRange: Into<D>,
{
    #[inline]
    fn from(pos: LineColByteRange) -> Self {
        (
            pos.clone().into(),
            pos.clone().into(),
            pos.clone().into(),
            pos.into(),
        )
    }
}

impl<A, B, C, D, E> From<LineColByteRange> for (A, B, C, D, E)
where
    LineColByteRange: Into<A>,
    LineColByteRange: Into<B>,
    LineColByteRange: Into<C>,
    LineColByteRange: Into<D>,
    LineColByteRange: Into<E>,
{
    #[inline]
    fn from(pos: LineColByteRange) -> Self {
        (
            pos.clone().into(),
            pos.clone().into(),
            pos.clone().into(),
            pos.clone().into(),
            pos.into(),
        )
    }
}

impl<A, B, C, D, E, F> From<LineColByteRange> for (A, B, C, D, E, F)
where
    LineColByteRange: Into<A>,
    LineColByteRange: Into<B>,
    LineColByteRange: Into<C>,
    LineColByteRange: Into<D>,
    LineColByteRange: Into<E>,
    LineColByteRange: Into<F>,
{
    #[inline]
    fn from(pos: LineColByteRange) -> Self {
        (
            pos.clone().into(),
            pos.clone().into(),
            pos.clone().into(),
            pos.clone().into(),
            pos.clone().into(),
            pos.into(),
        )
    }
}
