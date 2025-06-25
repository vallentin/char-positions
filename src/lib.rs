#![no_std]
#![forbid(unsafe_code)]
#![forbid(elided_lifetimes_in_paths)]

use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::ops::Range;

use char_ranges::{CharRanges, CharRangesExt};

pub trait CharPositionsExt {
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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Line(
    /// 1-indexed line.
    pub usize,
);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Col(
    /// 1-indexed column.
    pub usize,
);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct ByteStart(
    /// The start (inclusive) byte positions.
    pub usize,
);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct ByteEnd(
    /// The end (exclusive) byte position.
    pub usize,
);

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct ByteRange(
    /// The start (inclusive) and end (exclusive) byte positions.
    pub Range<usize>,
);

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
