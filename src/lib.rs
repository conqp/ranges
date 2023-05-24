use std::ops::{Add, RangeInclusive, Sub};

/// Generate ranges from integer sequences
///
/// # Examples
///
/// ```
/// use std::ops::RangeInclusive;
/// use ranges::Ranges;
///
/// let sequence: Vec<i64> = vec![1, 2, 3, 6, 7, 9, 9, 9, 11, 20, 21, 22, 24, 23, 22];
/// let target: Vec<RangeInclusive<i64>> = vec![1..=3, 6..=7, 9..=9, 9..=9, 9..=9, 11..=11, 20..=22, 24..=22];
/// let ranges: Vec<RangeInclusive<i64>> = sequence.ranges().collect();
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

#[derive(Debug)]
pub struct RangesIterator<T>
where
    T: Iterator,
{
    numbers: T,
    start: Option<T::Item>,
    one: T::Item,
}

#[derive(Eq, PartialEq)]
enum Order {
    Ascending,
    Descending,
}

impl<T> RangesIterator<T>
where
    T: Iterator,
    T::Item: From<u8>,
{
    pub fn new(numbers: T) -> Self {
        Self {
            numbers,
            start: None,
            one: 1.into(),
        }
    }
}

impl<T> Iterator for RangesIterator<T>
where
    T: Iterator,
    T::Item: Add<T::Item, Output = T::Item>
        + Sub<T::Item, Output = T::Item>
        + PartialEq
        + From<u8>
        + Copy,
{
    type Item = RangeInclusive<T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut order: Option<Order> = None;
        let mut end: Option<T::Item> = None;

        loop {
            match self.numbers.next() {
                None => {
                    return match self.start {
                        None => None,
                        Some(start) => {
                            self.start = None;

                            match end {
                                None => Some(start..=start),
                                Some(end) => Some(start..=end),
                            }
                        }
                    }
                }
                Some(next) => match self.start {
                    None => {
                        self.start = Some(next);
                    }
                    Some(start) => {
                        let last = end.unwrap_or(start);

                        match &order {
                            None => {
                                if next == last + self.one {
                                    end = Some(next);
                                    order = Some(Order::Ascending);
                                } else if next == last - self.one {
                                    end = Some(next);
                                    order = Some(Order::Descending);
                                } else {
                                    self.start = Some(next);
                                    return Some(start..=last);
                                }
                            }
                            Some(order) => {
                                if (order == &Order::Ascending && next == last + self.one)
                                    || (order == &Order::Descending && next == last - self.one)
                                {
                                    end = Some(next)
                                } else {
                                    self.start = Some(next);
                                    return Some(start..=last);
                                }
                            }
                        }
                    }
                },
            }
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
