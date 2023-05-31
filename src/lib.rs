use std::fmt::{Display, Error, Formatter};
use std::ops::{Add, RangeInclusive, Sub};

/// Generate ranges from integer sequences
///
/// # Examples
///
/// ```
/// use std::ops::RangeInclusive;
/// use ranges::{Range, Ranges};
///
/// let sequence = vec![1, 2, 3, 6, 7, 9, 9, 9, 11, 20, 21, 22, 24, 23, 22];
/// let target = [1..=3, 6..=7, 9..=9, 9..=9, 9..=9, 11..=11, 20..=22, 24..=22].map(
///     |range| Range::new(*range.start(), *range.end())
/// ).into_iter().collect::<Vec<_>>();
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

impl<T> IntoIterator for Range<T>
where
    T: Display + PartialOrd,
    RangeInclusive<T>: Iterator<Item = T>,
{
    type Item = T;
    type IntoIter = RangeInclusive<T>;

    fn into_iter(self) -> Self::IntoIter {
        if self.start > self.end {
            self.end..=self.start
        } else {
            self.start..=self.end
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

#[derive(Debug, Eq, PartialEq)]
enum Order {
    Ascending,
    Descending,
}

impl Order {
    pub fn new<T>(start: T, next: T) -> Option<Self>
    where
        T: PartialEq + Add<T, Output = T> + Copy + Sub<T, Output = T> + From<u8>,
    {
        if next == start + 1.into() {
            Some(Self::Ascending)
        } else if next == start - 1.into() {
            Some(Self::Descending)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct RangesIterator<T>
where
    T: Iterator,
{
    numbers: T,
    start: Option<T::Item>,
}

impl<T> RangesIterator<T>
where
    T: Iterator,
{
    pub fn new(numbers: T) -> Self {
        Self {
            numbers,
            start: None,
        }
    }
}

impl<T> Iterator for RangesIterator<T>
where
    T: Iterator,
    T::Item: Add<T::Item, Output = T::Item>
        + Copy
        + Display
        + Sub<T::Item, Output = T::Item>
        + PartialEq
        + From<u8>,
{
    type Item = Range<T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut order: Option<Order> = None;
        let mut end: Option<T::Item> = None;

        for next in &mut self.numbers {
            match self.start {
                None => {
                    self.start = Some(next);
                }
                Some(start) => {
                    let last = end.unwrap_or(start);

                    match &order {
                        None => match Order::new(last, next) {
                            Some(new_order) => {
                                order = Some(new_order);
                                end = Some(next);
                            }
                            None => {
                                self.start = Some(next);
                                return Some(Range::new(start, last));
                            }
                        },
                        Some(order) => {
                            if let Some(new_order) = &Order::new(last, next) {
                                if new_order == order {
                                    end = Some(next);
                                    continue;
                                }
                            }
                            self.start = Some(next);
                            return Some(Range::new(start, last));
                        }
                    }
                }
            }
        }

        if let Some(start) = self.start {
            self.start = None;

            match end {
                None => Some(Range::new(start, start)),
                Some(end) => Some(Range::new(start, end)),
            }
        } else {
            None
        }
    }
}

impl<T> From<T> for RangesIterator<T>
where
    T: Iterator,
    T::Item: From<u8>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}
