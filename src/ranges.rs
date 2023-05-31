use crate::ranges_iterator::RangesIterator;

/// Generate ranges from integer sequences
///
/// # Examples
///
/// ```
/// use std::ops::RangeInclusive;
/// use ranges::{Range, Ranges};
///
/// let sequence = vec![1, 2, 3, 6, 7, 9, 9, 9, 11, 20, 21, 22, 24, 23, 22];
/// let target: Vec<Range<_>> = vec![1..=3, 6..=7, 9..=9, 9..=9, 9..=9, 11..=11, 20..=22, 24..=22].into_iter().map(
///     |range| range.into()
/// ).collect();
/// let ranges: Vec<_> = sequence.ranges().collect();
///
/// assert_eq!(ranges, target);
/// ```
pub trait Ranges<T>
where
    T: Iterator,
{
    fn ranges(self) -> RangesIterator<T>;
}

impl<T> Ranges<T::IntoIter> for T
where
    T: IntoIterator,
    T::Item: From<u8>,
{
    fn ranges(self) -> RangesIterator<T::IntoIter> {
        self.into_iter().into()
    }
}
