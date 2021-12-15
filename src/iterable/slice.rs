//! Stream implementation for Rust slice.
use super::Iterable;

impl<'a, T> Iterable for &'a [T] {
    type Item = &'a T;
    type Iter = std::slice::Iter<'a, T>;

    #[inline]
    fn iter(&self) -> Self::Iter {
        <[T]>::iter(self)
    }

    #[inline]
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
}

impl<'a, T> Iterable for ark_std::ops::Range<T>
where
    ark_std::ops::Range<T>: ExactSizeIterator<Item = T> + Clone,
{
    type Item = T;

    type Iter = Self;

    fn iter(&self) -> Self::Iter {
        self.clone()
    }

    fn len(&self) -> usize {
        ExactSizeIterator::len(self)
    }
}

/// Reversed stream for Rust slice.
/// It outputs elements in the slice in reversed order.
#[derive(Clone, Copy)]
pub struct Reversed<'a, T>(&'a [T]);

impl<'a, T> Reversed<'a, T> {
    /// Initialize a new stream for the slice.
    pub fn new(slice: &'a [T]) -> Self {
        Self(slice)
    }
}

impl<'a, T> Iterable for Reversed<'a, T>
where
    T: Copy,
{
    type Item = &'a T;

    type Iter = ark_std::iter::Rev<std::slice::Iter<'a, T>>;

    #[inline]
    fn iter(&self) -> Self::Iter {
        self.0.iter().rev()
    }

    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}