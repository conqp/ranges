use either::{Either, Left, Right};
use std::fmt::{Display, Error, Formatter};
use std::iter::Rev;
use std::ops::{Add, Range as OpsRange, RangeInclusive};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Range<T>
where
    T: Display + PartialEq,
{
    start: T,
    end: T,
}

impl<T> Range<T>
where
    T: Display + PartialEq,
{
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
}

/// Iterator for ranges
///
/// # Examples
///
/// ```
/// use ranges::Ranges;
///
/// let sequence = vec![1, 2, 3, 6, 7, 9, 9, 9, 11, 20, 21, 22, 24, 23, 22];
/// let ranges: Vec<_> = sequence.clone().ranges().collect();
///
/// assert_eq!(sequence, ranges.into_iter().flatten().into_iter().collect::<Vec<_>>());
/// ```
impl<T> IntoIterator for Range<T>
where
    T: Display + PartialOrd,
    RangeInclusive<T>: Iterator<Item = T> + DoubleEndedIterator,
{
    type Item = T;
    type IntoIter = Either<RangeInclusive<T>, Rev<RangeInclusive<T>>>;

    fn into_iter(self) -> Self::IntoIter {
        if self.start > self.end {
            Right((self.end..=self.start).rev())
        } else {
            Left(self.start..=self.end)
        }
    }
}

impl<T> Display for Range<T>
where
    T: Display + PartialEq,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if self.start == self.end {
            write!(f, "{}", self.start)
        } else {
            write!(f, "{{{}..{}}}", self.start, self.end)
        }
    }
}

impl<T> From<RangeInclusive<T>> for Range<T>
where
    T: Clone + Display + PartialEq,
{
    fn from(value: RangeInclusive<T>) -> Self {
        Self::new(value.start().clone(), value.end().clone())
    }
}

impl<T> From<OpsRange<T>> for Range<T>
where
    T: Add<T, Output = T> + Clone + Display + From<u8> + PartialEq,
{
    fn from(value: OpsRange<T>) -> Self {
        Self::new(value.start, value.end + 1.into())
    }
}
